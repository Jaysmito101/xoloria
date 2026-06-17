use std::collections::{HashMap, HashSet};

use emulator::instructions::Instruction;
use emulator::{BusIO, Machine, MachineBuilder};

use crate::state::*;
use crate::ui_state::UiState;

const TICKS_PER_FRAME: usize = 10000;

pub enum TickResult {
    Ok,
    Error(String),
    Panic(String),
    Breakpoint(u64),
}

pub struct Debugger {
    pub(crate) binary: Vec<u8>,
    pub(crate) config_harts: usize,
    pub(crate) config_memory_exp: u32,

    pub(crate) machine: Option<Machine>,
    pub(crate) hart_modes: Vec<HartMode>,
    pub(crate) screen: Screen,
    pub(crate) running: bool,
    pub(crate) tick_count: u64,
    pub(crate) last_message: Option<(String, bool)>,
    pub(crate) breakpoints: HashSet<u64>,
    pub(crate) console_log: Vec<ConsoleEntry>,
    pub(crate) tracing_log: std::sync::Arc<std::sync::Mutex<Vec<ConsoleEntry>>>,
    pub(crate) theme: Theme,

    pub(crate) ui: UiState,

    pub(crate) disasm_cache: Option<DisasmCache>,

    pub(crate) source_lines: HashMap<u64, String>,
    pub(crate) symbols: HashMap<u64, String>,
    pub(crate) sorted_symbols: Vec<(u64, String)>,
}

pub(crate) struct DisasmCache {
    pub hart: usize,
    pub pc: u64,
    pub breakpoint_gen: u64,
    pub cursor: i32,
    pub entries: Vec<DisasmEntry>,
}

impl Debugger {
    pub fn new(binary_path: &str, elf_path: Option<&str>) -> anyhow::Result<Self> {
        let binary = std::fs::read(binary_path)?;
        let (source_lines, symbols) = match elf_path {
            Some(path) => Self::load_elf_symbols(path),
            None => (HashMap::new(), HashMap::new()),
        };
        let mut console_log = Vec::new();
        if !source_lines.is_empty() || !symbols.is_empty() {
            console_log.push(ConsoleEntry {
                message: format!(
                    "Loaded {} source locations and {} symbols from ELF debug info",
                    source_lines.len(),
                    symbols.len()
                ),
                level: ConsoleLevel::Info,
                tick: 0,
            });
        }
        let min_ram = (binary.len() as u64).next_power_of_two().ilog2() - 10;
        Ok(Self {
            binary,
            config_harts: 1,
            config_memory_exp: min_ram.clamp(4, 20),
            machine: None,
            hart_modes: vec![HartMode::Debug; 1],
            screen: Screen::Setup,
            running: true,
            tick_count: 0,
            last_message: None,
            breakpoints: HashSet::new(),
            console_log,
            tracing_log: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            theme: Theme::default(),
            ui: UiState::new(),
            disasm_cache: None,
            source_lines,
            symbols: symbols.clone(),
            sorted_symbols: {
                let mut s: Vec<_> = symbols.into_iter().collect();
                s.sort_by_key(|(addr, _)| *addr);
                s
            },
        })
    }

    fn load_elf_symbols(path: &str) -> (HashMap<u64, String>, HashMap<u64, String>) {
        let mut source_map = HashMap::new();
        let mut symbol_map = HashMap::new();
        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Warning: could not read ELF file: {}", e);
                return (source_map, symbol_map);
            }
        };

        let obj = match object::File::parse(&*data) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Warning: could not parse ELF: {}", e);
                return (source_map, symbol_map);
            }
        };

        use object::{Object, ObjectSection, ObjectSymbol};

        for sym in obj.symbols() {
            if sym.is_definition() {
                if let Ok(name) = sym.name() {
                    if !name.is_empty() && !name.starts_with(".L") {
                        let demangled = rustc_demangle::demangle(name).to_string();
                        symbol_map.insert(sym.address(), demangled);
                    }
                }
            }
        }
        let endian = if obj.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };

        let load_section = |id: gimli::SectionId| -> Result<
            gimli::EndianSlice<'_, gimli::RunTimeEndian>,
            gimli::Error,
        > {
            let section_data = obj
                .section_by_name(id.name())
                .and_then(|s| s.uncompressed_data().ok());
            let slice = match section_data {
                Some(std::borrow::Cow::Borrowed(bytes)) => bytes,
                Some(std::borrow::Cow::Owned(_)) => &[],
                None => &[],
            };
            Ok(gimli::EndianSlice::new(slice, endian))
        };

        let dwarf = match gimli::Dwarf::load(&load_section) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Warning: could not load DWARF sections: {}", e);
                return (source_map, symbol_map);
            }
        };

        let ctx = match addr2line::Context::from_dwarf(dwarf) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: could not build addr2line context: {}", e);
                return (source_map, symbol_map);
            }
        };

        for section in obj.sections() {
            let addr = section.address();
            let size = section.size();
            if size == 0 {
                continue;
            }
            let mut offset = 0u64;
            while offset < size {
                let pc = addr + offset;
                if let Ok(Some(loc)) = ctx.find_location(pc)
                    && let (Some(file), Some(line)) = (loc.file, loc.line)
                {
                    let short: &str = file.rsplit(['/', '\\']).next().unwrap_or(file);
                    source_map.insert(pc, format!("{}:{}", short, line));
                }
                offset += 2;
            }
        }

        (source_map, symbol_map)
    }

    pub(crate) fn set_error(&mut self, msg: impl Into<String>) {
        let message = msg.into();
        self.console_log.push(ConsoleEntry {
            message: message.clone(),
            level: ConsoleLevel::Error,
            tick: self.tick_count,
        });
        self.last_message = Some((message, true));
    }

    pub(crate) fn set_info(&mut self, msg: impl Into<String>) {
        let message = msg.into();
        self.console_log.push(ConsoleEntry {
            message: message.clone(),
            level: ConsoleLevel::Info,
            tick: self.tick_count,
        });
        self.last_message = Some((message, false));
    }

    pub(crate) fn log_panic(&mut self, msg: impl Into<String>) {
        let message = msg.into();
        self.console_log.push(ConsoleEntry {
            message: message.clone(),
            level: ConsoleLevel::Panic,
            tick: self.tick_count,
        });
        self.last_message = Some((message, true));
    }

    pub(crate) fn memory_size(&self) -> usize {
        1 << (self.config_memory_exp + 10)
    }

    pub(crate) fn memory_label(&self) -> String {
        let size = self.memory_size();
        if size >= 1024 * 1024 * 1024 {
            format!("{} GB", size / (1024 * 1024 * 1024))
        } else if size >= 1024 * 1024 {
            format!("{} MB", size / (1024 * 1024))
        } else {
            format!("{} KB", size / 1024)
        }
    }

    pub(crate) fn rebuild_machine(&mut self, config: MachineConfig) {
        match MachineBuilder::new("Xoloria/Debugger")
            .with_harts(config.harts)
            .and_then(|builder| builder.with_memory(config.memory_size)?.build())
            .and_then(|machine| {
                machine.load_binary(0x80000000, &self.binary)?;
                Ok(machine)
            }) {
            Ok(m) => {
                self.machine = Some(m);
                self.hart_modes.resize(config.harts, HartMode::Debug);
                self.last_message = None;
                self.disasm_cache = None;
            }
            Err(e) => self.set_error(format!("{}", e)),
        }
    }

    pub(crate) fn total_setup_fields(&self) -> usize {
        2 + self.config_harts
    }

    pub(crate) fn adjust_harts(&mut self, delta: i32) {
        let new = (self.config_harts as i32 + delta).clamp(1, 8) as usize;
        if new != self.config_harts {
            self.config_harts = new;
            self.hart_modes.resize(new, HartMode::Debug);
            let max = self.total_setup_fields().saturating_sub(1);
            self.ui.setup_cursor = self.ui.setup_cursor.min(max);
        }
    }

    pub(crate) fn adjust_memory(&mut self, delta: i32) {
        let new = (self.config_memory_exp as i32 + delta).clamp(0, 20) as u32;
        if new != self.config_memory_exp {
            self.config_memory_exp = new;
        }
    }

    pub(crate) fn toggle_breakpoint_at(&mut self, addr: u64) {
        if self.breakpoints.remove(&addr) {
            self.set_info(format!("Removed breakpoint at {:#x}", addr));
        } else {
            self.breakpoints.insert(addr);
            self.set_info(format!("Set breakpoint at {:#x}", addr));
        }
        self.disasm_cache = None;
    }

    pub(crate) fn submit_goto_memory(&mut self, input: &str) {
        let input = input.trim().trim_start_matches("0x");
        match u64::from_str_radix(input, 16) {
            Ok(addr) => {
                self.ui.memory_addr = addr;
                self.ui.panel = Panel::Memory;
            }
            Err(_) => self.set_error("Invalid hex address"),
        }
        self.ui.set_input_mode(InputMode::Normal);
    }

    fn tick_hart(&mut self, hart_idx: usize) -> TickResult {
        let Some(machine) = self.machine.as_mut() else {
            return TickResult::Error("No machine".into());
        };
        let bus = machine.bus().clone();
        let hart = &mut machine.harts_mut()[hart_idx];
        crate::SUPPRESS_PANIC_HOOK.with(|f| f.set(true));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hart.tick(&bus)));
        crate::SUPPRESS_PANIC_HOOK.with(|f| f.set(false));
        match result {
            Ok(Ok(())) => {
                let pc = hart.registers().pc();
                if self.breakpoints.contains(&pc) {
                    TickResult::Breakpoint(pc)
                } else {
                    TickResult::Ok
                }
            }
            Ok(Err(e)) => TickResult::Error(format!("{}", e)),
            Err(panic) => {
                let msg = if let Some(s) = panic.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "unknown panic".into()
                };
                TickResult::Panic(msg)
            }
        }
    }

    pub fn step_hart(&mut self, n: usize) {
        for _ in 0..n {
            match self.tick_hart(self.ui.selected_hart) {
                TickResult::Ok => {
                    self.tick_count += 1;
                    self.last_message = None;
                }
                TickResult::Breakpoint(pc) => {
                    self.tick_count += 1;
                    self.set_info(format!("Breakpoint at {:#x}", pc));
                    break;
                }
                TickResult::Error(msg) => {
                    self.set_error(msg);
                    break;
                }
                TickResult::Panic(msg) => {
                    self.log_panic(format!("PANIC: {}", msg));
                    break;
                }
            }
        }
        self.ui.disasm_cursor = 0;
        self.disasm_cache = None;
    }

    pub fn tick(&mut self) {
        let has_running_harts =
            self.hart_modes.contains(&HartMode::Running) && self.screen == Screen::Debug;

        if !has_running_harts {
            return;
        }

        for _ in 0..TICKS_PER_FRAME {
            let Some(machine) = self.machine.as_ref() else {
                return;
            };
            let bus = machine.bus().clone();

            let running: Vec<bool> = self
                .hart_modes
                .iter()
                .map(|m| *m == HartMode::Running)
                .collect();

            let harts = self.machine.as_mut().unwrap().harts_mut();
            let mut stalls: Vec<(usize, String)> = Vec::new();
            let mut bp_hits: Vec<(usize, u64)> = Vec::new();

            for (i, hart) in harts.iter_mut().enumerate() {
                if !running[i] {
                    continue;
                }
                crate::SUPPRESS_PANIC_HOOK.with(|f| f.set(true));
                let result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hart.tick(&bus)));
                crate::SUPPRESS_PANIC_HOOK.with(|f| f.set(false));
                match result {
                    Ok(Ok(())) => {
                        let pc = hart.registers().pc();
                        if self.breakpoints.contains(&pc) {
                            bp_hits.push((i, pc));
                        }
                    }
                    Ok(Err(e)) => stalls.push((i, format!("Hart {}: {}", i, e))),
                    Err(_) => stalls.push((i, format!("Hart {}: unimplemented instruction", i))),
                }
            }

            for (i, msg) in stalls {
                self.hart_modes[i] = HartMode::Stalled;
                self.set_error(msg);
            }
            if let Some((i, pc)) = bp_hits.into_iter().next() {
                self.hart_modes[i] = HartMode::Debug;
                self.ui.selected_hart = i;
                self.ui.disasm_cursor = 0;
                self.tick_count += 1;
                self.set_info(format!("Breakpoint at {:#x}", pc));
                self.disasm_cache = None;
                return;
            }
            self.tick_count += 1;
        }
        self.disasm_cache = None;
    }

    pub(crate) fn execute_command(&mut self) {
        let input = self.ui.input_buffer_take();
        let cmd = input.trim().to_string();
        self.ui.set_input_mode(InputMode::Normal);

        if cmd.is_empty() {
            return;
        }

        match DebugCommand::parse(&cmd) {
            Err(msg) => {
                if !msg.is_empty() {
                    self.set_error(msg);
                }
            }
            Ok(command) => match command {
                DebugCommand::Breakpoint(addr) => {
                    let addr = addr.unwrap_or_else(|| {
                        self.machine
                            .as_ref()
                            .map(|m| m.harts()[self.ui.selected_hart].registers().pc())
                            .unwrap_or(0)
                    });
                    self.toggle_breakpoint_at(addr);
                }
                DebugCommand::Delete(target) => match target {
                    DeleteTarget::All => {
                        let count = self.breakpoints.len();
                        self.breakpoints.clear();
                        self.set_info(format!("Cleared {} breakpoints", count));
                        self.disasm_cache = None;
                    }
                    DeleteTarget::Address(addr) => {
                        if self.breakpoints.remove(&addr) {
                            self.set_info(format!("Removed breakpoint at {:#x}", addr));
                            self.disasm_cache = None;
                        } else {
                            self.set_error(format!("No breakpoint at {:#x}", addr));
                        }
                    }
                },
                DebugCommand::Info(target) => match target {
                    InfoTarget::Breakpoints => {
                        if self.breakpoints.is_empty() {
                            self.set_info("No breakpoints set");
                        } else {
                            let list: Vec<String> = self
                                .breakpoints
                                .iter()
                                .map(|a| format!("{:#x}", a))
                                .collect();
                            self.set_info(format!("Breakpoints: {}", list.join(", ")));
                        }
                    }
                    InfoTarget::Registers => {
                        if let Some(m) = self.machine.as_ref() {
                            let regs = m.harts()[self.ui.selected_hart].registers();
                            self.set_info(format!("pc={:#x}", regs.pc()));
                        } else {
                            self.set_error("No machine");
                        }
                    }
                },
                DebugCommand::Memory(addr) => {
                    self.ui.memory_addr = addr;
                    self.ui.panel = Panel::Memory;
                    self.set_info(format!("Memory at {:#x}", addr));
                }
                DebugCommand::Step(n) => {
                    if self.hart_modes[self.ui.selected_hart] == HartMode::Debug {
                        self.step_hart(n);
                        self.set_info(format!("Stepped {} instruction(s)", n));
                    } else {
                        self.set_error("Hart is not in Debug mode");
                    }
                }
                DebugCommand::Continue => {
                    self.hart_modes[self.ui.selected_hart] = HartMode::Running;
                    self.set_info("Continuing...");
                }
                DebugCommand::Pause => {
                    self.hart_modes[self.ui.selected_hart] = HartMode::Debug;
                    self.set_info("Paused");
                }
                DebugCommand::Stall => {
                    self.hart_modes[self.ui.selected_hart] = HartMode::Stalled;
                    self.set_info("Stalled");
                }
                DebugCommand::Hart(idx) => {
                    if idx < self.hart_modes.len() {
                        self.ui.selected_hart = idx;
                        self.ui.disasm_cursor = 0;
                        self.set_info(format!("Selected hart {}", idx));
                        self.disasm_cache = None;
                    } else {
                        self.set_error(format!(
                            "Hart {} does not exist (0-{})",
                            idx,
                            self.hart_modes.len() - 1
                        ));
                    }
                }
                DebugCommand::Reset => {
                    let config = MachineConfig {
                        harts: self.config_harts,
                        memory_size: self.memory_size(),
                    };
                    self.rebuild_machine(config);
                    self.tick_count = 0;
                    self.ui.disasm_cursor = 0;
                    self.set_info("Machine reset");
                }
                DebugCommand::Targets => {
                    self.ui.show_targets = !self.ui.show_targets;
                    self.set_info(if self.ui.show_targets {
                        "Jump targets: ON"
                    } else {
                        "Jump targets: OFF"
                    });
                }
                DebugCommand::Help => {
                    self.set_info(
                        "bp [addr] | del <addr|all> | info bp | mem <addr> | step [n] | continue | pause | hart <n> | reset | targets | help"
                    );
                }
            },
        }
    }

    pub(crate) fn read_memory_block(&self, addr: u64, len: usize) -> Vec<u8> {
        let Some(machine) = self.machine.as_ref() else {
            return vec![0xFF; len];
        };
        let bus = machine.bus();
        (0..len)
            .map(|offset| bus.read::<u8>(addr + offset as u64).unwrap_or(0xFF))
            .collect()
    }

    pub(crate) fn disassemble_around(&mut self, count: usize) -> Vec<DisasmEntry> {
        let Some(machine) = self.machine.as_ref() else {
            return Vec::new();
        };

        let hart = &machine.harts()[self.ui.selected_hart];
        let hw_pc = hart.registers().pc();
        let bp_gen = self.breakpoints.len() as u64;
        let pc = self.ui.view_center_addr.unwrap_or(hw_pc);

        if let Some(ref cache) = self.disasm_cache
            && cache.hart == self.ui.selected_hart
            && cache.pc == pc
            && cache.breakpoint_gen == bp_gen
            && cache.cursor == self.ui.disasm_cursor
        {
            return cache.entries.clone();
        }

        let x_regs = hart.registers().x();
        let bus = machine.bus();

        let cursor = self.ui.disasm_cursor;
        let before = if cursor < 0 {
            count / 3 + (-cursor) as usize + 50
        } else {
            count / 3
        };

        let after = if cursor > 0 {
            count - count / 3 + cursor as usize + 50
        } else {
            count - count / 3
        };

        let mut entries: Vec<DisasmEntry> = Vec::new();

        if before > 0 {
            let scan_start = pc.saturating_sub(before as u64 * 4);
            let mut addr = scan_start;
            while addr < pc {
                if let Some((entry, step)) =
                    Self::decode_at(addr, bus, hw_pc, &self.breakpoints, x_regs)
                {
                    entries.push(entry);
                    addr += step;
                } else {
                    addr += 2;
                }
            }
            let skip = entries.len().saturating_sub(before);
            entries = entries.into_iter().skip(skip).collect();
        }

        let mut addr = pc;
        for _ in 0..after {
            if let Some((entry, step)) = Self::decode_at(addr, bus, hw_pc, &self.breakpoints, x_regs) {
                entries.push(entry);
                addr += step;
            } else {
                entries.push(DisasmEntry {
                    addr,
                    text: "???".into(),
                    is_pc: addr == pc,
                    is_bp: false,
                    jump_target: None,
                    symbol: None,
                });
                addr += 2;
            }
        }

        for entry in &mut entries {
            entry.symbol = self.symbols.get(&entry.addr).cloned();
        }

        self.disasm_cache = Some(DisasmCache {
            hart: self.ui.selected_hart,
            pc,
            breakpoint_gen: bp_gen,
            cursor: self.ui.disasm_cursor,
            entries: entries.clone(),
        });

        entries
    }

    fn decode_at(
        addr: u64,
        bus: &emulator::Bus,
        pc: u64,
        breakpoints: &HashSet<u64>,
        x_regs: &[u64; 32],
    ) -> Option<(DisasmEntry, u64)> {
        let raw: u32 = bus.read(addr).ok()?;
        let is_compressed = raw & 0b11 != 0b11;
        let step = if is_compressed { 2 } else { 4 };

        crate::SUPPRESS_PANIC_HOOK.with(|f| f.set(true));
        let decode_result = std::panic::catch_unwind(|| Instruction::try_from(raw));
        crate::SUPPRESS_PANIC_HOOK.with(|f| f.set(false));

        let text = match decode_result {
            Ok(Ok(instr)) => format!("{}", instr),
            Ok(Err(_)) if is_compressed => format!(".half {:#06x}", raw & 0xFFFF),
            Ok(Err(_)) => format!(".word {:#010x}", raw),
            Err(_) if is_compressed => format!("<unimpl> {:#06x}", raw & 0xFFFF),
            Err(_) => format!("<unimpl> {:#010x}", raw),
        };

        let jump_target = Self::extract_jump_target(raw, addr, pc, x_regs);

        let entry = DisasmEntry {
            addr,
            text,
            is_pc: addr == pc,
            is_bp: breakpoints.contains(&addr),
            jump_target,
            symbol: None, // Filled in later by disassemble_around
        };
        Some((entry, step))
    }

    fn extract_jump_target(raw: u32, addr: u64, pc: u64, x_regs: &[u64; 32]) -> Option<JumpTarget> {
        crate::SUPPRESS_PANIC_HOOK.with(|f| f.set(true));
        let decode_result = std::panic::catch_unwind(|| Instruction::try_from(raw));
        crate::SUPPRESS_PANIC_HOOK.with(|f| f.set(false));

        let instr = match decode_result {
            Ok(Ok(instr)) => instr,
            _ => return None,
        };

        match instr {
            Instruction::Jal { imm, .. } => {
                let target = (addr as i64 + imm as i64) as u64;
                Some(JumpTarget::Known(target))
            }
            Instruction::Jalr { rs1, imm, .. } => {
                if addr == pc {
                    let rs1_idx = rs1 as u8 as usize;
                    let target = ((x_regs[rs1_idx] as i64 + imm as i64) & !1) as u64;
                    Some(JumpTarget::Known(target))
                } else {
                    Some(JumpTarget::Unknown)
                }
            }
            Instruction::Beq { imm, .. }
            | Instruction::Bne { imm, .. }
            | Instruction::Blt { imm, .. }
            | Instruction::Bge { imm, .. }
            | Instruction::Bltu { imm, .. }
            | Instruction::Bgeu { imm, .. } => {
                let target = (addr as i64 + imm as i64) as u64;
                Some(JumpTarget::Known(target))
            }
            _ => None,
        }
    }
}

pub enum JumpTarget {
    Known(u64),
    Unknown,
}

#[derive(Clone)]
pub struct DisasmEntry {
    pub addr: u64,
    pub text: String,
    pub is_pc: bool,
    pub is_bp: bool,
    pub jump_target: Option<JumpTarget>,
    pub symbol: Option<String>,
}

impl Clone for JumpTarget {
    fn clone(&self) -> Self {
        match self {
            Self::Known(a) => Self::Known(*a),
            Self::Unknown => Self::Unknown,
        }
    }
}
