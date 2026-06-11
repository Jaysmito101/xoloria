use crate::instructions::{Instruction, InstructionError, InstructionResult, OpcodeName, payload};

impl Instruction {
    #[inline(always)]
    fn try_lui(raw: payload::UType) -> InstructionResult<Self> {
        Ok(Self::Lui {
            rd: raw.rd,
            imm: raw.imm,
        })
    }

    #[inline(always)]
    fn try_auipc(raw: payload::UType) -> InstructionResult<Self> {
        Ok(Self::Auipc {
            rd: raw.rd,
            imm: raw.imm,
        })
    }

    #[inline(always)]
    fn try_jal(raw: payload::JType) -> InstructionResult<Self> {
        Ok(Self::Jal {
            rd: raw.rd,
            imm: raw.imm,
        })
    }

    #[inline(always)]
    fn try_jalr(raw: payload::IType) -> InstructionResult<Self> {
        Ok(Self::Jalr {
            rd: raw.rd,
            rs1: raw.rs1,
            imm: raw.imm,
        })
    }

    #[inline(always)]
    fn try_branch(raw: payload::BType) -> InstructionResult<Self> {
        match raw.funct3 {
            0 => Ok(Self::Beq {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            1 => Ok(Self::Bne {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            4 => Ok(Self::Blt {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            5 => Ok(Self::Bge {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            6 => Ok(Self::Bltu {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            7 => Ok(Self::Bgeu {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    #[inline(always)]
    fn try_load(raw: payload::IType) -> InstructionResult<Self> {
        match raw.funct3 {
            0 => Ok(Self::Lb {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            1 => Ok(Self::Lh {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            2 => Ok(Self::Lw {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            4 => Ok(Self::Lbu {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            5 => Ok(Self::Lhu {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            3 => Ok(Self::Ld {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            6 => Ok(Self::Lwu {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    fn try_store(raw: payload::SType) -> InstructionResult<Self> {
        match raw.funct3 {
            0 => Ok(Self::Sb {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            1 => Ok(Self::Sh {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            2 => Ok(Self::Sw {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            3 => Ok(Self::Sd {
                rs1: raw.rs1,
                rs2: raw.rs2,
                imm: raw.imm,
            }),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    pub fn try_op_imm(raw: payload::IType) -> InstructionResult<Self> {
        match raw.funct3 {
            0 => Ok(Self::Addi {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            2 => Ok(Self::Slti {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            3 => Ok(Self::Sltiu {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            4 => Ok(Self::Xori {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            6 => Ok(Self::Ori {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            7 => Ok(Self::Andi {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            1 => {
                // ammount is the lower 5 bits of the immediate
                let shamt = (raw.imm & 0x1f) as u8;
                match (raw.imm >> 10) & 0x3f {
                    0 => Ok(Self::Slli {
                        rd: raw.rd,
                        rs1: raw.rs1,
                        imm: shamt,
                    }),
                    _ => Err(InstructionError::InvalidInstruction),
                }
            }
            5 => {
                // ammount is the lower 5 bits of the immediate
                let shamt = (raw.imm & 0x1f) as u8;
                // the right shift type is determined by bits bit 30 of the instruction (which is bit 10 of the immediate)
                match (raw.imm >> 10) & 0x3f {
                    0 => Ok(Self::Srli {
                        rd: raw.rd,
                        rs1: raw.rs1,
                        imm: shamt,
                    }),
                    1 => Ok(Self::Srai {
                        rd: raw.rd,
                        rs1: raw.rs1,
                        imm: shamt,
                    }),
                    _ => Err(InstructionError::InvalidInstruction),
                }
            }
            _ => Err(InstructionError::InvalidInstruction),
        }
    }
}

impl TryFrom<u32> for Instruction {
    type Error = InstructionError;

    fn try_from(value: u32) -> InstructionResult<Self> {
        match OpcodeName::try_from(value & 0x7f)? {
            OpcodeName::Load => Self::try_load(payload::IType::try_from(value)?),
            OpcodeName::OpImm => Self::try_op_imm(payload::IType::try_from(value)?),
            OpcodeName::Auipc => Self::try_auipc(payload::UType::try_from(value)?),
            OpcodeName::Store => Self::try_store(payload::SType::try_from(value)?),
            OpcodeName::OpReg => {
                tracing::info!("Parsing OpReg instruction with value {:#010x}", value);
                todo!()
            }
            OpcodeName::Lui => Self::try_lui(payload::UType::try_from(value)?),
            OpcodeName::Branch => Self::try_branch(payload::BType::try_from(value)?),
            OpcodeName::Jalr => Self::try_jalr(payload::IType::try_from(value)?),
            OpcodeName::Jal => Self::try_jal(payload::JType::try_from(value)?),
            OpcodeName::OpImm64 => {
                tracing::info!("Parsing OpImm64 instruction with value {:#010x}", value);
                todo!()
            }
            OpcodeName::OpReg64 => {
                tracing::info!("Parsing OpReg64 instruction with value {:#010x}", value);
                todo!()
            }
            OpcodeName::Atomic => {
                tracing::info!("Parsing Atomic instruction with value {:#010x}", value);
                todo!()
            }
            OpcodeName::Fence => {
                tracing::info!("Parsing Fence instruction with value {:#010x}", value);
                todo!()
            }
            OpcodeName::System => {
                tracing::info!("Parsing System instruction with value {:#010x}", value);
                todo!()
            }
        }
    }
}
