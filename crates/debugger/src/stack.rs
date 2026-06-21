use emulator::instructions::Instruction;
use emulator::registers::GeneralRegisterName;

use crate::state::DataType;

#[derive(Debug, Clone)]
pub struct StackPush {
    pub offset: i32,
    pub data_type: DataType,
    pub reg: GeneralRegisterName,
}

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub size: i32,
    pub pushes: Vec<StackPush>,
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub target_pc: u64,
    pub return_pc: u64,
    pub stack_frame: Option<StackFrame>,
    pub entry_sp: u64,
}

#[derive(Debug, Clone)]
pub struct StackAnalyzer {
    pub call_stack: Vec<CallFrame>,
}

impl Default for StackAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl StackAnalyzer {
    pub fn new() -> Self {
        Self {
            call_stack: vec![CallFrame {
                target_pc: 0,
                return_pc: 0,
                stack_frame: None,
                entry_sp: 0,
            }],
        }
    }

    pub fn current_frame_mut(&mut self) -> Option<&mut Option<StackFrame>> {
        self.call_stack.last_mut().map(|f| &mut f.stack_frame)
    }

    pub fn current_frame(&self) -> Option<&StackFrame> {
        self.call_stack.last().and_then(|f| f.stack_frame.as_ref())
    }

    pub fn on_instruction_executed(&mut self, inst: &Instruction, _pc: u64, return_pc: u64, next_pc: u64, current_sp: u64) -> Option<String> {
        match inst {
            Instruction::Jal { rd, .. } if *rd == GeneralRegisterName::Ra => {
                self.call_stack.push(CallFrame {
                    target_pc: next_pc,
                    return_pc,
                    stack_frame: None,
                    entry_sp: current_sp,
                });
            }
            Instruction::Jalr { rd, rs1, .. } => {
                if *rd == GeneralRegisterName::Ra {
                    self.call_stack.push(CallFrame {
                        target_pc: next_pc,
                        return_pc,
                        stack_frame: None,
                        entry_sp: current_sp,
                    });
                } else if *rs1 == GeneralRegisterName::Ra && *rd == GeneralRegisterName::Zero {
                    if let Some(last) = self.call_stack.last() {
                        if last.return_pc == next_pc && self.call_stack.len() > 1 {
                            let popped = self.call_stack.pop().unwrap();
                            if popped.entry_sp != current_sp {
                                return Some(format!(
                                    "Stack pointer mismatch on return from {:#x}: expected {:#x}, got {:#x}",
                                    popped.target_pc, popped.entry_sp, current_sp
                                ));
                            }
                        }
                    }
                }
            }
            Instruction::Addi { rd, rs1, imm }
                if *rd == GeneralRegisterName::Sp && *rs1 == GeneralRegisterName::Sp =>
            {
                if *imm < 0 {
                    if let Some(frame_opt) = self.current_frame_mut() {
                        *frame_opt = Some(StackFrame {
                            size: -imm,
                            pushes: Vec::new(),
                        });
                    }
                } else if *imm > 0 {
                    if let Some(Some(frame)) = self.current_frame_mut() {
                        frame.size -= *imm;
                        if frame.size <= 0 {
                            if let Some(f) = self.current_frame_mut() {
                                *f = None;
                            }
                        }
                    }
                }
            }
            Instruction::Sb { rs1, rs2, imm } if *rs1 == GeneralRegisterName::Sp => {
                self.record_push(*imm, DataType::U8, *rs2);
            }
            Instruction::Sh { rs1, rs2, imm } if *rs1 == GeneralRegisterName::Sp => {
                self.record_push(*imm, DataType::U16, *rs2);
            }
            Instruction::Sw { rs1, rs2, imm } if *rs1 == GeneralRegisterName::Sp => {
                self.record_push(*imm, DataType::U32, *rs2);
            }
            Instruction::Sd { rs1, rs2, imm } if *rs1 == GeneralRegisterName::Sp => {
                self.record_push(*imm, DataType::U64, *rs2);
            }
            _ => {}
        }
        None
    }

    fn record_push(&mut self, offset: i32, data_type: DataType, reg: GeneralRegisterName) {
        if let Some(Some(frame)) = self.current_frame_mut() {
            if let Some(existing) = frame.pushes.iter_mut().find(|p| p.offset == offset) {
                existing.data_type = data_type;
                existing.reg = reg;
            } else {
                frame.pushes.push(StackPush {
                    offset,
                    data_type,
                    reg,
                });
            }
        }
    }
}
