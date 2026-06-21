use std::collections::HashSet;
use emulator::instructions::Instruction;
use emulator::BusIO;
use crate::app::Debugger;

pub enum JumpTarget {
    Known(u64),
    Unknown,
}

impl Clone for JumpTarget {
    fn clone(&self) -> Self {
        match self {
            Self::Known(a) => Self::Known(*a),
            Self::Unknown => Self::Unknown,
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
    pub is_compressed: bool,
}

pub(crate) struct DisasmCache {
    pub hart: usize,
    pub pc: u64,
    pub breakpoint_gen: u64,
    pub cursor: i32,
    pub entries: Vec<DisasmEntry>,
}

pub struct Disassembler<'a> {
    pub bus: &'a emulator::Bus,
    pub breakpoints: &'a HashSet<u64>,
}

impl<'a> Disassembler<'a> {
    pub fn new(bus: &'a emulator::Bus, breakpoints: &'a HashSet<u64>) -> Self {
        Self { bus, breakpoints }
    }

    pub fn disassemble_instruction_at(&self, addr: u64) -> Option<String> {
        let raw: u32 = self.bus.read(addr).ok()?;
        let is_compressed = raw & 0b11 != 0b11;
        let decode_result = Instruction::try_from(raw);
        match decode_result {
            Ok(instr) => Some(format!("{}", instr)),
            Err(_) if is_compressed => Some(format!(".half {:#06x}", raw & 0xFFFF)),
            Err(_) => Some(format!(".word {:#010x}", raw)),
        }
    }

    pub fn decode_at(
        &self,
        addr: u64,
        pc: u64,
        x_regs: &[u64; 32],
    ) -> Option<(DisasmEntry, u64)> {
        let raw: u32 = self.bus.read(addr).ok()?;
        let is_compressed = raw & 0b11 != 0b11;
        let step = if is_compressed { 2 } else { 4 };

        let decode_result = Instruction::try_from(raw);

        let text = match decode_result {
            Ok(instr) => format!("{}", instr),
            Err(_) if is_compressed => format!(".half {:#06x}", raw & 0xFFFF),
            Err(_) => format!(".word {:#010x}", raw),
        };

        let jump_target = self.extract_jump_target(raw, addr, pc, x_regs);

        let entry = DisasmEntry {
            addr,
            text,
            is_pc: addr == pc,
            is_bp: self.breakpoints.contains(&addr),
            jump_target,
            symbol: None,
            is_compressed,
        };
        Some((entry, step))
    }

    pub fn extract_jump_target(&self, raw: u32, addr: u64, pc: u64, x_regs: &[u64; 32]) -> Option<JumpTarget> {
        let decode_result = Instruction::try_from(raw);

        let instr = match decode_result {
            Ok(instr) => instr,
            Err(_) => return None,
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

impl Debugger {
    pub(crate) fn disassemble_instruction_at(&self, addr: u64) -> Option<String> {
        let machine = self.machine.as_ref()?;
        let disassembler = Disassembler::new(&machine.bus, &self.breakpoints);
        disassembler.disassemble_instruction_at(addr)
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
        let abs = (center_idx + self.ui.disasm.cursor).max(0) as usize;
        let abs = abs.min(entries.len().saturating_sub(1));

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

        if let Some(ref cache) = self.disasm_cache
            && cache.hart == self.ui.selected_hart
            && cache.pc == pc
            && cache.breakpoint_gen == bp_gen
            && cache.cursor == self.ui.disasm.cursor
        {
            return cache.entries.clone();
        }

        let x_regs = hart.registers().x();

        let cursor = self.ui.disasm.cursor;
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
        let disassembler = Disassembler::new(&machine.bus, &self.breakpoints);

        if before > 0 {
            let scan_start = pc.saturating_sub(before as u64 * 4);
            let mut addr = scan_start;
            while addr < pc {
                if let Some((entry, step)) =
                    disassembler.decode_at(addr, hw_pc, x_regs)
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
            if let Some((entry, step)) =
                disassembler.decode_at(addr, hw_pc, x_regs)
            {
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
                    is_compressed: false,
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
            cursor: self.ui.disasm.cursor,
            entries: entries.clone(),
        });

        entries
    }
}
