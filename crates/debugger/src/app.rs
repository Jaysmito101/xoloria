use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use emulator::instructions::Instruction;
use emulator::{BusIO, Machine, MachineBuilder};
use strum::IntoEnumIterator;

use crate::debug_symbols::DebugSymbols;
use crate::disassembly::DisasmCache;
use crate::state::RegisterIdentifier;
use crate::ui_state::UiState;
use crate::{stack::StackAnalyzer, state::*};

pub enum TickResult {
    Ok,
    Error(String),
    Breakpoint(u64),
    Watchpoint(u64, String),
    RegisterWatchpoint(u64, RegisterIdentifier),
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
    pub(crate) theme: crate::theme::Theme,

    pub(crate) ui: UiState,

    pub(crate) disasm_cache: Option<DisasmCache>,

    pub(crate) debug_symbols: Option<crate::debug_symbols::DebugSymbols>,

    pub(crate) stack_analyzers: Vec<crate::stack::StackAnalyzer>,
    pub(crate) watches: Vec<crate::state::WatchItem>,
}

pub(crate) struct WatchPointSnapshot {
    has_watch: bool,
    pre_watch_values: Vec<Option<Vec<u8>>>,
}

impl WatchPointSnapshot {
    pub(crate) fn capture(watches: &[crate::state::WatchItem], bus: &impl BusIO) -> Self {
        let mut has_watch = false;
        let mut pre_watch_values = Vec::new();
        for watch in watches {
            if watch.break_on_change {
                has_watch = true;
                pre_watch_values.push(Some(watch.read_value(bus)));
            } else {
                pre_watch_values.push(None);
            }
        }
        Self {
            has_watch,
            pre_watch_values,
        }
    }

    pub(crate) fn check(
        self,
        watches: &[crate::state::WatchItem],
        bus: &impl BusIO,
    ) -> Option<String> {
        if self.has_watch {
            for (i, watch) in watches.iter().enumerate() {
                if let Some(old_val) = &self.pre_watch_values[i] {
                    let new_val = watch.read_value(bus);
                    if old_val != &new_val {
                        return Some(watch.name.clone());
                    }
                }
            }
        }
        None
    }
}

pub(crate) struct RegWatchSnapshot {
    has_watch: bool,
    pre_values: Vec<(RegisterIdentifier, u64)>,
}

impl RegWatchSnapshot {
    pub(crate) fn capture(
        watch_names: &[crate::state::RegisterIdentifier],
        hart: &emulator::Hart,
    ) -> Self {
        let mut has_watch = false;
        let mut pre_values = Vec::new();
        let regs = hart.registers();
        for ident in watch_names {
            has_watch = true;
            let val = match ident {
                crate::state::RegisterIdentifier::Pc => regs.pc(),
                crate::state::RegisterIdentifier::Gpr(gpr) => regs.x()[*gpr as usize],
                crate::state::RegisterIdentifier::Csr(csr) => regs
                    .csr()
                    .read(*csr, emulator::PrivilageMode::Machine)
                    .unwrap_or_default(),
            };
            pre_values.push((ident.clone(), val));
        }
        Self {
            has_watch,
            pre_values,
        }
    }

    pub(crate) fn check(self, hart: &emulator::Hart) -> Option<crate::state::RegisterIdentifier> {
        if !self.has_watch {
            return None;
        }
        let regs = hart.registers();
        for (ident, old_val) in &self.pre_values {
            let new_val = match ident {
                crate::state::RegisterIdentifier::Pc => regs.pc(),
                crate::state::RegisterIdentifier::Gpr(gpr) => regs.x()[*gpr as usize],
                crate::state::RegisterIdentifier::Csr(csr) => regs
                    .csr()
                    .read(*csr, emulator::PrivilageMode::Machine)
                    .unwrap_or_default(),
            };
            if new_val != *old_val {
                return Some(ident.clone());
            }
        }
        None
    }
}

pub(crate) struct TickContext<'a> {
    inst: Option<Instruction>,
    inst_size: u64,
    snapshot: WatchPointSnapshot,
    reg_snapshot: RegWatchSnapshot,
    pub pc: u64,
    pub sp: u64,
    hart: &'a mut emulator::Hart,
    watches: &'a [crate::state::WatchItem],
    bus: &'a emulator::Bus,
    analyzer: &'a mut crate::stack::StackAnalyzer,
    trace: Option<&'a mut crate::ui_state::TraceState>,
}

impl<'a> TickContext<'a> {
    pub(crate) fn begin(
        hart: &'a mut emulator::Hart,
        watches: &'a [crate::state::WatchItem],
        reg_watch_names: &[RegisterIdentifier],
        bus: &'a emulator::Bus,
        analyzer: &'a mut crate::stack::StackAnalyzer,
        trace: Option<&'a mut crate::ui_state::TraceState>,
    ) -> Self {
        let pc = hart.registers().pc();
        let sp = hart.registers().x()[2];
        let inst_val = bus.read::<u32>(pc).unwrap_or(0);
        let inst_size = if (inst_val & 0b11) != 0b11 { 2 } else { 4 };
        let inst = Instruction::try_from(inst_val).ok();
        let snapshot = WatchPointSnapshot::capture(watches, bus);
        let reg_snapshot = RegWatchSnapshot::capture(reg_watch_names, hart);
        Self {
            inst,
            inst_size,
            snapshot,
            reg_snapshot,
            pc,
            sp,
            hart,
            watches,
            bus,
            analyzer,
            trace,
        }
    }

    pub(crate) fn tick(
        self,
    ) -> (
        emulator::Result<()>,
        Option<String>,
        Option<String>,
        Option<crate::state::RegisterIdentifier>,
    ) {
        let result = self.hart.tick(self.bus);
        let mut stack_warning = None;
        if result.is_ok()
            && let Some(i) = self.inst
        {
            let next_pc = self.hart.registers().pc();
            let current_sp = self.hart.registers().x()[2];
            let mut args = [0; 8];
            for i in 0..8 {
                args[i] = self.hart.registers().x()[10 + i];
            }
            stack_warning = self.analyzer.on_instruction_executed(
                &i,
                self.pc,
                self.pc + self.inst_size,
                next_pc,
                current_sp,
                args,
            );
        }
        let watch_hit = self.snapshot.check(self.watches, self.bus);
        let reg_watch_hit = self.reg_snapshot.check(self.hart);

        if let Some(trace) = self.trace
            && trace.stack.last().map(|e| e.pc) != Some(self.pc)
        {
            trace
                .stack
                .push(crate::ui_state::TraceEntry::new(self.pc, self.sp));
            trace.forward_stack.clear();
        }

        (result, watch_hit, stack_warning, reg_watch_hit)
    }
}

impl Debugger {
    pub fn new(binary_path: &str, elf_path: Option<&str>) -> anyhow::Result<Self> {
        let binary = std::fs::read(binary_path)?;
        let (console_log, debug_symbols) = if let Some(ds) = elf_path.map(DebugSymbols::new) {
            (
                vec![ConsoleEntry {
                    message: format!(
                        "Loaded {} source locations and {} symbols from ELF debug info",
                        ds.source_lines.len(),
                        ds.symbols.len()
                    ),
                    level: ConsoleLevel::Info,
                    tick: 0,
                }],
                Some(ds),
            )
        } else {
            (Vec::new(), None)
        };

        let min_ram = ((binary.len() as u64).next_power_of_two().ilog2() - 10).max(4) as u32;
        let mut app = Self {
            binary,
            config_harts: 1,
            config_memory_exp: min_ram,
            machine: None,
            hart_modes: vec![HartMode::Debug; 1],
            screen: Screen::Setup,
            running: true,
            tick_count: 0,
            last_message: None,
            breakpoints: HashSet::new(),
            console_log,
            tracing_log: Arc::new(Mutex::new(Vec::new())),
            theme: crate::theme::Theme::default(),
            ui: UiState::new(),
            disasm_cache: None,
            debug_symbols,
            stack_analyzers: vec![StackAnalyzer::new(); 1],
            watches: Vec::new(),
        };

        match crate::theme::Theme::load() {
            Ok(theme) => app.theme = theme,
            Err(e) => app.set_error(e),
        }

        Ok(app)
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
                self.stack_analyzers
                    .resize(config.harts, crate::stack::StackAnalyzer::new());
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
            self.stack_analyzers
                .resize(new, crate::stack::StackAnalyzer::new());
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

    fn handle_tick_result(&mut self, hart_idx: usize, result: TickResult) -> bool {
        match result {
            TickResult::Ok => false,
            TickResult::Breakpoint(pc) => {
                self.hart_modes[hart_idx] = HartMode::Debug;
                self.ui.selected_hart = hart_idx;
                self.ui.disasm.cursor = 0;
                self.tick_count += 1;
                self.set_info(format!("Breakpoint at {:#x}", pc));
                self.disasm_cache = None;
                true
            }
            TickResult::Watchpoint(pc, name) => {
                self.hart_modes[hart_idx] = HartMode::Debug;
                self.ui.selected_hart = hart_idx;
                self.ui.disasm.cursor = 0;
                self.tick_count += 1;
                self.set_info(format!("Watchpoint '{}' triggered at {:#x}", name, pc));
                self.disasm_cache = None;
                true
            }
            TickResult::RegisterWatchpoint(pc, ident) => {
                self.hart_modes[hart_idx] = HartMode::Debug;
                self.ui.selected_hart = hart_idx;
                self.ui.disasm.cursor = 0;
                self.tick_count += 1;
                self.set_info(format!(
                    "Register watchpoint '{}' triggered at {:#x}",
                    ident, pc
                ));
                self.disasm_cache = None;
                self.ui.panel = Panel::Registers;
                true
            }
            TickResult::Error(msg) => {
                self.hart_modes[hart_idx] = HartMode::Stalled;
                self.set_error(msg);
                false
            }
        }
    }

    fn tick_hart(&mut self, hart_idx: usize) -> TickResult {
        let Some(machine) = self.machine.as_mut() else {
            return TickResult::Error("No machine".into());
        };
        let hart = &mut machine.harts[hart_idx];

        let trace_opt = if hart_idx == self.ui.selected_hart {
            Some(&mut self.ui.trace)
        } else {
            None
        };

        let reg_watch_names = self.ui.registers.break_on_change.clone();
        let ctx = TickContext::begin(
            hart,
            &self.watches,
            &reg_watch_names,
            &machine.bus,
            &mut self.stack_analyzers[hart_idx],
            trace_opt,
        );
        let pc = ctx.pc;

        let (result, watch_hit, stack_warning, reg_watch_hit) = ctx.tick();

        if let Some(warn) = stack_warning {
            return TickResult::Error(warn);
        }
        if let Some(name) = watch_hit {
            return TickResult::Watchpoint(pc, name);
        }
        if let Some(ident) = reg_watch_hit {
            return TickResult::RegisterWatchpoint(pc, ident);
        }

        match result {
            Ok(()) => {
                let pc = hart.registers().pc();
                if self.breakpoints.contains(&pc) {
                    TickResult::Breakpoint(pc)
                } else {
                    TickResult::Ok
                }
            }
            Err(e) => TickResult::Error(format!("{}", e)),
        }
    }

    pub fn step_hart(&mut self, n: usize) {
        for _ in 0..n {
            let hart_idx = self.ui.selected_hart;

            let result = self.tick_hart(hart_idx);

            match result {
                TickResult::Ok => {
                    self.tick_count += 1;
                    self.last_message = None;
                }
                other => {
                    self.handle_tick_result(hart_idx, other);
                    break;
                }
            }
        }
        self.ui.disasm.cursor = 0;
        self.disasm_cache = None;
    }

    pub fn tick(&mut self) {
        let has_running_harts =
            self.hart_modes.contains(&HartMode::Running) && self.screen == Screen::Debug;

        if !has_running_harts {
            return;
        }

        for _ in 0..10000 {
            let Some(machine) = self.machine.as_mut() else {
                return;
            };
            let running: Vec<bool> = self
                .hart_modes
                .iter()
                .map(|m| *m == HartMode::Running)
                .collect();

            let mut tick_results: Vec<(usize, TickResult)> = Vec::new();

            for (i, hart) in machine.harts.iter_mut().enumerate() {
                if !running[i] {
                    continue;
                }

                let trace_opt = if i == self.ui.selected_hart {
                    Some(&mut self.ui.trace)
                } else {
                    None
                };

                let reg_watch_names = self.ui.registers.break_on_change.clone();
                let ctx = TickContext::begin(
                    hart,
                    &self.watches,
                    &reg_watch_names,
                    &machine.bus,
                    &mut self.stack_analyzers[i],
                    trace_opt,
                );
                let pc = ctx.pc;

                let (result, watch_hit, stack_warning, reg_watch_hit) = ctx.tick();

                if let Some(warn) = stack_warning {
                    tick_results.push((i, TickResult::Error(warn)));
                    continue;
                }
                if let Some(name) = watch_hit {
                    tick_results.push((i, TickResult::Watchpoint(pc, name)));
                    continue;
                }
                if let Some(ident) = reg_watch_hit {
                    tick_results.push((i, TickResult::RegisterWatchpoint(pc, ident)));
                    continue;
                }

                match result {
                    Ok(()) => {
                        let pc = hart.registers().pc();
                        if self.breakpoints.contains(&pc) {
                            tick_results.push((i, TickResult::Breakpoint(pc)));
                        }
                    }
                    Err(e) => {
                        tick_results.push((i, TickResult::Error(format!("Hart {}: {}", i, e))))
                    }
                }
            }

            let mut should_return = false;
            for (i, res) in tick_results {
                match res {
                    TickResult::Error(_) => {
                        self.handle_tick_result(i, res);
                    }
                    _ => {
                        if !should_return && self.handle_tick_result(i, res) {
                            should_return = true;
                        }
                    }
                }
            }

            if should_return {
                return;
            }
            self.tick_count += 1;
        }
        self.disasm_cache = None;
    }

    #[inline(always)]
    pub(crate) fn do_read_memory(&mut self, data_type: crate::state::DataType, addr: u64) {
        use emulator::BusIO;
        let bus = match self.machine.as_ref() {
            Some(m) => &m.bus,
            None => return self.set_error("Machine not loaded"),
        };

        macro_rules! read_mem {
            ($ty:ty, $cast:ty, $name:expr) => {
                match bus.read::<$ty>(addr) {
                    Ok(v) => {
                        let v = v as $cast;
                        self.set_info(format!("Read {} at {:#x}: {:#x} ({})", $name, addr, v, v))
                    }
                    Err(e) => self.set_error(format!("Read failed: {:?}", e)),
                }
            };
        }

        match data_type {
            crate::state::DataType::U8 => read_mem!(u8, u8, "u8"),
            crate::state::DataType::U16 => read_mem!(u16, u16, "u16"),
            crate::state::DataType::U32 => read_mem!(u32, u32, "u32"),
            crate::state::DataType::U64 => read_mem!(u64, u64, "u64"),
            crate::state::DataType::I8 => read_mem!(u8, i8, "i8"),
            crate::state::DataType::I16 => read_mem!(u16, i16, "i16"),
            crate::state::DataType::I32 => read_mem!(u32, i32, "i32"),
            crate::state::DataType::I64 => read_mem!(u64, i64, "i64"),
        }
    }

    #[inline(always)]
    pub(crate) fn do_write_memory(
        &mut self,
        data_type: crate::state::DataType,
        addr: u64,
        val: u64,
    ) {
        use emulator::BusIO;
        let bus = match self.machine.as_ref() {
            Some(m) => &m.bus,
            None => return self.set_error("Machine not loaded"),
        };

        macro_rules! write_mem {
            ($ty:ty) => {
                match bus.write::<$ty>(addr, val as $ty) {
                    Ok(_) => self.set_info(format!("Wrote {:#x} to {:#x}", val as $ty, addr)),
                    Err(e) => self.set_error(format!("Write failed: {:?}", e)),
                }
            };
        }

        match data_type {
            crate::state::DataType::U8 | crate::state::DataType::I8 => write_mem!(u8),
            crate::state::DataType::U16 | crate::state::DataType::I16 => write_mem!(u16),
            crate::state::DataType::U32 | crate::state::DataType::I32 => write_mem!(u32),
            crate::state::DataType::U64 | crate::state::DataType::I64 => write_mem!(u64),
        }
    }

    pub(crate) fn save_workspace(&mut self) {
        use std::hash::{DefaultHasher, Hasher};
        let mut hasher = DefaultHasher::new();
        hasher.write(&self.binary);
        let hash = hasher.finish();

        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "Xoloria", "Debugger") {
            let config_dir = proj_dirs.config_dir();
            if !config_dir.exists() {
                let _ = std::fs::create_dir_all(config_dir);
            }
            let file_path = config_dir.join(format!("workspace_{:016x}.json", hash));
            let bps: Vec<u64> = self.breakpoints.iter().copied().collect();
            let ws = crate::state::Workspace {
                breakpoints: bps,
                watches: self.watches.clone(),
                pinned_registers: self.ui.registers.pinned.clone(),
                register_watchpoints: self.ui.registers.break_on_change.clone(),
                ui: Some(self.ui.clone()),
            };
            match std::fs::write(
                &file_path,
                serde_json::to_string_pretty(&ws).unwrap_or_default(),
            ) {
                Ok(_) => self.set_info(format!(
                    "Saved workspace ({} bps, {} watches, {} pinned regs, {} reg watchpoints)",
                    self.breakpoints.len(),
                    self.watches.len(),
                    self.ui.registers.pinned.len(),
                    self.ui.registers.break_on_change.len()
                )),
                Err(e) => self.set_error(format!("Failed to save workspace: {}", e)),
            }
        } else {
            self.set_error("Could not find configuration directory");
        }
    }

    pub(crate) fn load_workspace(&mut self) {
        use std::hash::{DefaultHasher, Hasher};
        let mut hasher = DefaultHasher::new();
        hasher.write(&self.binary);
        let hash = hasher.finish();

        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "Xoloria", "Debugger") {
            let config_dir = proj_dirs.config_dir();
            let file_path = config_dir.join(format!("workspace_{:016x}.json", hash));
            if file_path.exists() {
                match std::fs::read_to_string(&file_path) {
                    Ok(json) => {
                        if let Ok(ws) = serde_json::from_str::<crate::state::Workspace>(&json) {
                            for addr in ws.breakpoints {
                                self.breakpoints.insert(addr);
                            }
                            self.watches = ws.watches;
                            if let Some(ui) = ws.ui {
                                self.ui = ui;
                            }
                            self.disasm_cache = None;
                            self.set_info(format!(
                                "Loaded workspace ({} bps, {} watches, {} pinned regs, {} reg watchpoints)",
                                self.breakpoints.len(),
                                self.watches.len(),
                                self.ui.registers.pinned.len(),
                                self.ui.registers.break_on_change.len()
                            ));
                        } else {
                            self.set_error("Failed to parse workspace JSON");
                        }
                    }
                    Err(e) => self.set_error(format!("Failed to read workspace: {}", e)),
                }
            } else {
                self.set_error("No workspace found for this binary");
            }
        } else {
            self.set_error("Could not find configuration directory");
        }
    }

    pub(crate) fn read_memory_block(&self, addr: u64, len: usize) -> Vec<u8> {
        let Some(machine) = self.machine.as_ref() else {
            return vec![0xFF; len];
        };
        (0..len)
            .map(|offset| machine.bus.read::<u8>(addr + offset as u64).unwrap_or(0xFF))
            .collect()
    }

    pub(crate) fn get_register_at_cursor(&self) -> Option<crate::state::RegisterIdentifier> {
        use emulator::registers::{ControlRegisterName, GeneralRegisterName};
        match self.ui.registers_tab {
            crate::ui_state::RegistersTab::Gpr => {
                let mut all_idents: Vec<crate::state::RegisterIdentifier> = Vec::with_capacity(33);

                for pinned_ident in &self.ui.registers.pinned {
                    if matches!(pinned_ident, crate::state::RegisterIdentifier::Csr(_)) {
                        continue;
                    }
                    all_idents.push(pinned_ident.clone());
                }

                let pc_ident = crate::state::RegisterIdentifier::Pc;
                let pc_pinned = self.ui.registers.pinned.iter().any(|n| n == &pc_ident);
                if !pc_pinned {
                    all_idents.push(pc_ident);
                }

                for gpr in GeneralRegisterName::iter() {
                    let ident = crate::state::RegisterIdentifier::Gpr(gpr);
                    let is_pinned = self.ui.registers.pinned.iter().any(|n| n == &ident);
                    if is_pinned {
                        continue;
                    }
                    all_idents.push(ident);
                }

                let cursor = self.ui.registers.cursor;
                all_idents.get(cursor).cloned()
            }
            crate::ui_state::RegistersTab::Csr => {
                let mut all_idents: Vec<crate::state::RegisterIdentifier> = Vec::new();

                for pinned_ident in &self.ui.registers.pinned {
                    if matches!(pinned_ident, crate::state::RegisterIdentifier::Csr(_)) {
                        all_idents.push(pinned_ident.clone());
                    }
                }

                for csr in ControlRegisterName::iter() {
                    let ident = crate::state::RegisterIdentifier::Csr(csr);
                    let is_pinned = self.ui.registers.pinned.iter().any(|n| n == &ident);
                    if is_pinned {
                        continue;
                    }
                    all_idents.push(ident);
                }

                let cursor = self.ui.registers.cursor;
                all_idents.get(cursor).cloned()
            }
        }
    }

    pub(crate) fn find_register_index(&self, search: &str) -> Option<usize> {
        let Some(machine) = self.machine.as_ref() else {
            return None;
        };
        let search = search.to_lowercase();
        if search.is_empty() {
            return None;
        }

        use emulator::registers::{ControlRegisterName, GeneralRegisterName};
        let regs = machine.harts[self.ui.selected_hart].registers();

        let matches_search = |ident: &crate::state::RegisterIdentifier, val: u64| -> bool {
            let name_lower = ident.to_string();
            let hex_val = format!("{:#018x}", val);
            name_lower.contains(&search) || hex_val.contains(&search)
        };

        match self.ui.registers_tab {
            crate::ui_state::RegistersTab::Gpr => {
                let mut idx = 0;
                for pinned_ident in &self.ui.registers.pinned {
                    if matches!(pinned_ident, crate::state::RegisterIdentifier::Csr(_)) {
                        continue;
                    }
                    let val = match pinned_ident {
                        crate::state::RegisterIdentifier::Pc => regs.pc(),
                        crate::state::RegisterIdentifier::Gpr(gpr) => regs.x()[*gpr as usize],
                        _ => 0,
                    };
                    if matches_search(pinned_ident, val) {
                        return Some(idx);
                    }
                    idx += 1;
                }

                let pc_ident = crate::state::RegisterIdentifier::Pc;
                let pc_pinned = self.ui.registers.pinned.iter().any(|n| n == &pc_ident);
                if !pc_pinned {
                    if matches_search(&pc_ident, regs.pc()) {
                        return Some(idx);
                    }
                    idx += 1;
                }

                for gpr in GeneralRegisterName::iter() {
                    let ident = crate::state::RegisterIdentifier::Gpr(gpr);
                    let is_pinned = self.ui.registers.pinned.iter().any(|n| n == &ident);
                    if is_pinned {
                        continue;
                    }
                    if matches_search(&ident, regs.x()[gpr as usize]) {
                        return Some(idx);
                    }
                    idx += 1;
                }
                None
            }
            crate::ui_state::RegistersTab::Csr => {
                let mut idx = 0;
                for pinned_ident in &self.ui.registers.pinned {
                    if let crate::state::RegisterIdentifier::Csr(csr) = pinned_ident {
                        let val = regs
                            .csr()
                            .read(*csr, emulator::PrivilageMode::Machine)
                            .unwrap_or_default();
                        if matches_search(pinned_ident, val) {
                            return Some(idx);
                        }
                        idx += 1;
                    }
                }

                use strum::IntoEnumIterator;
                for csr in ControlRegisterName::iter() {
                    let ident = crate::state::RegisterIdentifier::Csr(csr);
                    let is_pinned = self.ui.registers.pinned.iter().any(|n| n == &ident);
                    if is_pinned {
                        continue;
                    }
                    if matches_search(
                        &ident,
                        regs.csr()
                            .read(csr, emulator::PrivilageMode::Machine)
                            .unwrap_or_default(),
                    ) {
                        return Some(idx);
                    }
                    idx += 1;
                }
                None
            }
        }
    }
}
