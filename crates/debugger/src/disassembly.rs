use crate::app::Debugger;
use emulator::BusIO;
use emulator::instructions::Instruction;
use std::collections::HashSet;

#[derive(Clone)]
pub enum JumpTarget {
    Known(u64),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DisasmbledInstruction {
    pub text: String,
    pub instruction: Option<Instruction>,
    pub is_compressed: bool,
}

impl DisasmbledInstruction {
    #[inline(always)]
    pub fn from_raw(raw: u32) -> Self {
        let is_compressed = raw & 0b11 != 0b11;
        match Instruction::try_from(raw) {
            Ok(instr) => Self {
                text: format!("{}", instr),
                instruction: Some(instr),
                is_compressed,
            },
            Err(_) if is_compressed => Self {
                text: format!(".half {:#06x}", raw & 0xFFFF),
                instruction: None,
                is_compressed,
            },
            Err(_) => Self {
                text: format!(".word {:#010x}", raw),
                instruction: None,
                is_compressed,
            },
        }
    }
}

#[derive(Clone)]
pub struct DisasmEntry {
    pub addr: u64,
    pub text: String,
    pub is_pc: bool,
    pub is_bp: bool,
    pub jump_target: Option<JumpTarget>,
    pub symbol: Option<String>,
    pub instruction: Option<Instruction>,
    pub is_compressed: bool,
}

impl DisasmEntry {
    #[inline(always)]
    fn unknown(addr: u64, pc: u64) -> Self {
        Self {
            addr,
            text: "???".into(),
            is_pc: addr == pc,
            is_bp: false,
            jump_target: None,
            symbol: None,
            instruction: None,
            is_compressed: false,
        }
    }

    #[inline(always)]
    pub fn with_jump_target(mut self, x_regs: &[u64; 32], pc: u64) -> Self {
        let Some(instr) = &self.instruction else {
            self.jump_target = None;
            return self;
        };
        self.jump_target = match instr {
            Instruction::Jal { imm, .. } => {
                Some(JumpTarget::Known((self.addr as i64 + *imm as i64) as u64))
            }
            Instruction::Jalr { rs1, imm, .. } => {
                if self.addr == pc {
                    let target = ((x_regs[*rs1 as u8 as usize] as i64 + *imm as i64) & !1) as u64;
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
                Some(JumpTarget::Known((self.addr as i64 + *imm as i64) as u64))
            }
            _ => None,
        };
        self
    }

    #[inline(always)]
    pub fn from_raw(raw: u32, addr: u64, pc: u64, breakpoints: &HashSet<u64>) -> Self {
        let DisasmbledInstruction {
            text,
            instruction,
            is_compressed,
        } = DisasmbledInstruction::from_raw(raw);
        Self {
            addr,
            text,
            is_pc: addr == pc,
            is_bp: breakpoints.contains(&addr),
            jump_target: None,
            symbol: None,
            instruction,
            is_compressed,
        }
    }
}

pub(crate) struct DisasmCache {
    pub hart: usize,
    pub pc: u64,
    pub breakpoint_gen: u64,
    pub cursor: i32,
    pub entries: Vec<DisasmEntry>,
}

impl DisasmCache {
    #[inline(always)]
    fn matches(&self, hart: usize, pc: u64, bp_gen: u64, cursor: i32) -> bool {
        self.hart == hart && self.pc == pc && self.breakpoint_gen == bp_gen && self.cursor == cursor
    }
}

pub struct Disassembler<'a> {
    pub bus: &'a emulator::Bus,
    pub breakpoints: &'a HashSet<u64>,
}

impl<'a> Disassembler<'a> {
    pub fn new(bus: &'a emulator::Bus, breakpoints: &'a HashSet<u64>) -> Self {
        Self { bus, breakpoints }
    }

    #[inline(always)]
    fn read_raw(&self, addr: u64) -> Option<(u32, bool)> {
        let raw: u32 = self.bus.read(addr).ok()?;
        let is_compressed = raw & 0b11 != 0b11;
        Some((raw, is_compressed))
    }

    #[inline(always)]
    pub fn disassemble_instruction_at(&self, addr: u64) -> Option<String> {
        Some(DisasmbledInstruction::from_raw(self.read_raw(addr)?.0).text)
    }

    pub fn decode_at(&self, addr: u64, pc: u64, x_regs: &[u64; 32]) -> Option<(DisasmEntry, u64)> {
        let raw = self.read_raw(addr)?;
        Some((
            DisasmEntry::from_raw(raw.0, addr, pc, self.breakpoints).with_jump_target(x_regs, pc),
            if raw.1 { 2 } else { 4 },
        ))
    }


}

impl Debugger {
    pub(crate) fn disassembler(&self) -> Option<Disassembler<'_>> {
        let machine = self.machine.as_ref()?;
        Some(Disassembler::new(&machine.bus, &self.breakpoints))
    }

    pub(crate) fn resolve_cursor_target(&mut self) -> (u64, Option<DisasmEntry>, Vec<DisasmEntry>) {
        let entries = self.disassemble_around(200);
        let hw_pc = self
            .machine
            .as_ref()
            .map(|m| m.harts[self.ui.selected_hart].registers().pc())
            .unwrap_or(0);
        let center_addr = self.ui.disasm.view_center_addr.unwrap_or(hw_pc);
        let center_idx = entries
            .iter()
            .position(|e| e.addr == center_addr)
            .unwrap_or(0) as i32;
        let abs = (center_idx + self.ui.disasm.cursor)
            .max(0)
            .min(entries.len().saturating_sub(1) as i32) as usize;

        let target_entry = entries.get(abs).cloned();
        let target_addr = target_entry.as_ref().map(|e| e.addr).unwrap_or(center_addr);
        (target_addr, target_entry, entries)
    }

    pub(crate) fn disassemble_around(&mut self, count: usize) -> Vec<DisasmEntry> {
        let Some(machine) = self.machine.as_ref() else {
            return Vec::new();
        };

        let hart = &machine.harts[self.ui.selected_hart];
        let hw_pc = hart.registers().pc();
        let bp_gen = self.breakpoints.len() as u64;
        let pc = self.ui.disasm.view_center_addr.unwrap_or(hw_pc);
        let cursor = self.ui.disasm.cursor;

        if let Some(ref cache) = self.disasm_cache
            && cache.matches(self.ui.selected_hart, pc, bp_gen, cursor)
        {
            return cache.entries.clone();
        }

        let x_regs = hart.registers().x();

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

        let disassembler = Disassembler::new(&machine.bus, &self.breakpoints);

        let mut entries: Vec<DisasmEntry> = Vec::new();

        if before > 0 {
            let scan_start = pc.saturating_sub(before as u64 * 4);
            let mut addr = scan_start;
            while addr < pc {
                if let Some((entry, step)) = disassembler.decode_at(addr, hw_pc, x_regs) {
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
            match disassembler.decode_at(addr, hw_pc, x_regs) {
                Some((entry, step)) => {
                    addr += step;
                    entries.push(entry);
                }
                None => {
                    entries.push(DisasmEntry::unknown(addr, pc));
                    addr += 2;
                }
            }
        }

        for entry in &mut entries {
            entry.symbol = self.symbols.get(&entry.addr).cloned();
        }

        self.disasm_cache = Some(DisasmCache {
            hart: self.ui.selected_hart,
            pc,
            breakpoint_gen: bp_gen,
            cursor,
            entries: entries.clone(),
        });

        entries
    }
}
