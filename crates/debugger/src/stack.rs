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

#[derive(Debug, Clone, Default)]
pub struct StackAnalyzer {
    pub current_frame: Option<StackFrame>,
}

impl StackAnalyzer {
    pub fn new() -> Self {
        Self { current_frame: None }
    }

    pub fn on_instruction_executed(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Addi { rd, rs1, imm } if *rd == GeneralRegisterName::Sp && *rs1 == GeneralRegisterName::Sp => {
                if *imm < 0 {
                    // Start new allocation
                    self.current_frame = Some(StackFrame {
                        size: -imm,
                        pushes: Vec::new(),
                    });
                } else if *imm > 0 {
                    // Deallocate or shrink
                    if let Some(frame) = &mut self.current_frame {
                        frame.size -= *imm;
                        if frame.size <= 0 {
                            self.current_frame = None;
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
    }

    fn record_push(&mut self, offset: i32, data_type: DataType, reg: GeneralRegisterName) {
        if let Some(frame) = &mut self.current_frame {
            if let Some(existing) = frame.pushes.iter_mut().find(|p| p.offset == offset) {
                existing.data_type = data_type;
                existing.reg = reg;
            } else {
                frame.pushes.push(StackPush { offset, data_type, reg });
            }
        }
    }
}
