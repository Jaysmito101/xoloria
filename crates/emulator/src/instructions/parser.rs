use crate::{
    instructions::{Instruction, InstructionError, InstructionResult, OpcodeName, payload},
    registers::{ControlRegisterName, GeneralRegisterName},
};

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

    fn try_op_imm(raw: payload::IType) -> InstructionResult<Self> {
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

    fn try_op_reg(raw: payload::RType) -> InstructionResult<Self> {
        match (raw.funct3, raw.funct7) {
            (0, 0) => Ok(Self::Add {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (0, 0x20) => Ok(Self::Sub {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (1, 0) => Ok(Self::Sll {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (2, 0) => Ok(Self::Slt {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (3, 0) => Ok(Self::Sltu {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (4, 0) => Ok(Self::Xor {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (5, 0) => Ok(Self::Srl {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (5, 0x20) => Ok(Self::Sra {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (6, 0) => Ok(Self::Or {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (7, 0) => Ok(Self::And {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (0, 1) => Ok(Self::Mul {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (1, 1) => Ok(Self::Mulh {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (2, 1) => Ok(Self::Mulhsu {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (3, 1) => Ok(Self::Mulhu {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (4, 1) => Ok(Self::Div {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (5, 1) => Ok(Self::Divu {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (6, 1) => Ok(Self::Rem {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (7, 1) => Ok(Self::Remu {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    fn try_op_imm64(raw: payload::IType) -> InstructionResult<Self> {
        match raw.funct3 {
            0 => Ok(Self::Addiw {
                rd: raw.rd,
                rs1: raw.rs1,
                imm: raw.imm,
            }),
            1 => {
                // ammount is the lower 6 bits of the immediate
                let shamt = (raw.imm & 0x3f) as u8;
                match (raw.imm >> 12) & 0x7f {
                    0 => Ok(Self::Slliw {
                        rd: raw.rd,
                        rs1: raw.rs1,
                        imm: shamt,
                    }),
                    _ => Err(InstructionError::InvalidInstruction),
                }
            }
            5 => {
                // ammount is the lower 6 bits of the immediate
                let shamt = (raw.imm & 0x3f) as u8;
                // the right shift type is determined by bit 30 of the instruction (which is bit 12 of the immediate)
                match (raw.imm >> 12) & 0x7f {
                    0 => Ok(Self::Srliw {
                        rd: raw.rd,
                        rs1: raw.rs1,
                        imm: shamt,
                    }),
                    1 => Ok(Self::Sraiw {
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

    fn try_op_reg64(raw: payload::RType) -> InstructionResult<Self> {
        match (raw.funct3, raw.funct7) {
            (0, 0) => Ok(Self::Addw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (0, 0x20) => Ok(Self::Subw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (1, 0) => Ok(Self::Sllw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (5, 0) => Ok(Self::Srlw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (5, 0x20) => Ok(Self::Sraw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (0, 1) => Ok(Self::Mulw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (4, 1) => Ok(Self::Divw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (5, 1) => Ok(Self::Divuw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (6, 1) => Ok(Self::Remw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            (7, 1) => Ok(Self::Remuw {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
            }),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    fn try_fence(raw: payload::IType) -> InstructionResult<Self> {
        if raw.rd != GeneralRegisterName::Zero || raw.rs1 != GeneralRegisterName::Zero {
            return Err(InstructionError::InvalidInstruction);
        }
        match raw.funct3 {
            0 => Ok(Self::Fence),
            1 => Ok(Self::Fencei),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    fn try_system(raw: payload::IType) -> InstructionResult<Self> {
        let parse_csr = |csr_num: i32| {
            ControlRegisterName::try_from(csr_num as u16)
                .map_err(|_| InstructionError::InvalidInstruction)
        };
        match raw.funct3 {
            0 => match raw.imm {
                0 => Ok(Self::Ecall),
                1 => Ok(Self::Ebreak),
                0x102 => Ok(Self::Mret),
                0x302 => Ok(Self::Sret),
                0x105 => Ok(Self::Wfi),
                _ => Err(InstructionError::InvalidInstruction),
            },
            1 => Ok(Self::Csrrw {
                rd: raw.rd,
                rs1: raw.rs1,
                csr: parse_csr(raw.imm)?,
            }),
            2 => Ok(Self::Csrrs {
                rd: raw.rd,
                rs1: raw.rs1,
                csr: parse_csr(raw.imm)?,
            }),
            3 => Ok(Self::Csrrc {
                rd: raw.rd,
                rs1: raw.rs1,
                csr: parse_csr(raw.imm)?,
            }),
            5 => Ok(Self::Csrrwi {
                rd: raw.rd,
                csr: parse_csr(raw.imm)?,
                imm: (raw.rs1 as u8) & 0x1f,
            }),
            6 => Ok(Self::Csrrsi {
                rd: raw.rd,
                csr: parse_csr(raw.imm)?,
                imm: (raw.rs1 as u8) & 0x1f,
            }),
            7 => Ok(Self::Csrrci {
                rd: raw.rd,
                csr: parse_csr(raw.imm)?,
                imm: (raw.rs1 as u8) & 0x1f,
            }),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    fn try_atomic(raw: payload::RType) -> InstructionResult<Self> {
        let width = match raw.funct3 {
            2 => false, // word
            3 => true,  // double word
            _ => return Err(InstructionError::InvalidAtomicWidth(raw.funct3)),
        };
        let aq = ((raw.funct7 >> 1) & 1) != 0;
        let rl = (raw.funct7 & 1) != 0;
        match (raw.funct7 >> 2) & 0x1f {
            0 => Ok(Self::Amoadd {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            1 => Ok(Self::Amoswap {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            2 => Ok(Self::Lr {
                rd: raw.rd,
                rs1: raw.rs1,
                aq,
                rl,
                width,
            }),
            3 => Ok(Self::Sc {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            4 => Ok(Self::Amoxor {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            8 => Ok(Self::Amoor {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            12 => Ok(Self::Amoand {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            16 => Ok(Self::Amomin {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            20 => Ok(Self::Amomax {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            24 => Ok(Self::Amominu {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            28 => Ok(Self::Amomaxu {
                rd: raw.rd,
                rs1: raw.rs1,
                rs2: raw.rs2,
                aq,
                rl,
                width,
            }),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    fn try_from_compressed(value: u16) -> InstructionResult<Self> {
        Ok(Instruction::Noop)
    }

    fn try_from_uncompressed(value: u32) -> InstructionResult<Self> {
        let opcode = OpcodeName::try_from(value & 0x7f)?;
        match opcode {
            OpcodeName::Load => Self::try_load(payload::IType::try_from(value)?),
            OpcodeName::OpImm => Self::try_op_imm(payload::IType::try_from(value)?),
            OpcodeName::Auipc => Self::try_auipc(payload::UType::try_from(value)?),
            OpcodeName::Store => Self::try_store(payload::SType::try_from(value)?),
            OpcodeName::OpReg => Self::try_op_reg(payload::RType::try_from(value)?),
            OpcodeName::Lui => Self::try_lui(payload::UType::try_from(value)?),
            OpcodeName::Branch => Self::try_branch(payload::BType::try_from(value)?),
            OpcodeName::Jalr => Self::try_jalr(payload::IType::try_from(value)?),
            OpcodeName::Jal => Self::try_jal(payload::JType::try_from(value)?),
            OpcodeName::OpImm64 => Self::try_op_imm64(payload::IType::try_from(value)?),
            OpcodeName::OpReg64 => Self::try_op_reg64(payload::RType::try_from(value)?),
            OpcodeName::Atomic => Self::try_atomic(payload::RType::try_from(value)?),
            OpcodeName::Fence => Self::try_fence(payload::IType::try_from(value)?),
            OpcodeName::System => Self::try_system(payload::IType::try_from(value)?),
        }
    }
}

impl TryFrom<u32> for Instruction {
    type Error = InstructionError;

    fn try_from(value: u32) -> InstructionResult<Self> {
        if value & 0b11 != 0b11 {
            Self::try_from_compressed((value & 0xffff) as u16)
        } else {
            Self::try_from_uncompressed(value)
        }
    }
}
