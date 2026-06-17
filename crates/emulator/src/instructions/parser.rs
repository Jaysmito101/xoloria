use crate::{
    instructions::{
        Instruction, InstructionError, InstructionResult, OpcodeName,
        payload::{self, BitsExt},
    },
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

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
    fn try_parse_reg(raw: u16) -> InstructionResult<GeneralRegisterName> {
        GeneralRegisterName::try_from(raw as u8)
            .map_err(|_| InstructionError::UnknownRegister(raw as u8))
    }

    #[inline(always)]
    fn try_compressed_q0(value: u16) -> InstructionResult<Self> {
        let funct3 = value.bits(13, 3);
        match funct3 {
            0 => {
                let imm = (value.bits(7, 4) << 6)
                    | (value.bits(11, 2) << 4)
                    | (value.bits(5, 1) << 3)
                    | (value.bits(6, 1) << 2);
                let imm = if imm & (1 << 8) != 0 {
                    (imm | !0 << 9) as i16 as i32
                } else {
                    imm as i16 as i32
                };
                if imm == 0 {
                    return Err(InstructionError::InvalidInstruction);
                }
                Ok(Self::Addi {
                    rd: Self::try_parse_reg(value.bits(2, 3))?,
                    rs1: GeneralRegisterName::Sp,
                    imm: imm,
                })
            }
            1 => unimplemented!("C.FLD"),
            2 => unimplemented!("C.LW"),
            3 => {
                let imm = (value.bits(5, 2) << 6) | (value.bits(10, 3) << 3);
                let rd = Self::try_parse_reg(value.bits(2, 3))?;
                let rs1 = Self::try_parse_reg(value.bits(7, 3))?;
                Ok(Self::Ld {
                    rd,
                    rs1,
                    imm: imm as i32,
                })
            }
            5 => unimplemented!("C.FSD"),
            6 => unimplemented!("C.SW"),
            7 => unimplemented!("C.SD"),
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    #[inline(always)]
    fn try_compressed_q1(value: u16) -> InstructionResult<Self> {
        let funct3 = value.bits(13, 3);
        match funct3 {
            0 => {
                let rd = Self::try_parse_reg(value.bits(7, 5))?;
                let imm = (value.bits(2, 5) as i32) | (value.bits(12, 1) as i32) << 5;
                match rd {
                    GeneralRegisterName::Zero => Ok(Self::Noop),
                    _ => Ok(Self::Addi {
                        rd,
                        rs1: rd,
                        imm: if imm & (1 << 5) != 0 {
                            (imm | !0 << 6) as i16 as i32
                        } else {
                            imm as i16 as i32
                        },
                    }),
                }
            }
            1 => {
                let imm = ((value.bits(12, 1) as i32) << 31) >> 26 | value.bits(2, 5) as i32;
                let imm = if imm & (1 << 5) != 0 {
                    (imm | !0 << 6) as i16 as i32
                } else {
                    imm as i16 as i32
                };
                let rd = Self::try_parse_reg(value.bits(7, 5))?;
                if rd == GeneralRegisterName::Zero {
                    return Err(InstructionError::InvalidInstruction);
                }
                Ok(Self::Addiw { rd, rs1: rd, imm })
            }
            2 => {
                let rd = Self::try_parse_reg(value.bits(7, 5))?;
                let imm = ((value.bits(12, 1) as i32) << 31) >> 26 | value.bits(2, 5) as i32;
                match rd {
                    GeneralRegisterName::Zero => Ok(Self::Noop),
                    _ => Ok(Self::Addi {
                        rd,
                        rs1: GeneralRegisterName::Zero,
                        imm: if imm & (1 << 5) != 0 {
                            (imm | !0 << 6) as i16 as i32
                        } else {
                            imm as i16 as i32
                        },
                    }),
                }
            }
            3 => {
                let rd = Self::try_parse_reg(value.bits(7, 5))?;
                match rd {
                    GeneralRegisterName::Zero => Err(InstructionError::IllegalInstruction),
                    GeneralRegisterName::Sp => {
                        let imm = ((value.bits(12, 1) as i32) << 31) >> 22
                            | (value.bits(3, 2) as i32) << 7
                            | (value.bits(5, 1) as i32) << 6
                            | (value.bits(2, 1) as i32) << 5
                            | (value.bits(6, 1) as i32) << 4;
                        let nzimm = if imm & (1 << 8) != 0 {
                            (imm | !0 << 9) as i16 as i32
                        } else {
                            imm as i16 as i32
                        };
                        if nzimm == 0 {
                            return Err(InstructionError::IllegalInstruction);
                        }
                        Ok(Self::Addi {
                            rd: GeneralRegisterName::Sp,
                            rs1: GeneralRegisterName::Sp,
                            imm: nzimm,
                        })
                    }
                    _ => {
                        let imm = ((value.bits(12, 1) as i32) << 31) >> 14
                            | (value.bits(2, 5) as i32) << 12;
                        let imm = if imm & (1 << 16) != 0 {
                            (imm | !0 << 17) as i16 as i32
                        } else {
                            imm as i16 as i32
                        };
                        if imm == 0 {
                            return Err(InstructionError::InvalidInstruction);
                        }
                        Ok(Self::Lui { rd, imm })
                    }
                }
            }
            4 => {
                let funct2 = value.bits(10, 2);
                let rd_creg = Self::try_parse_reg(value.bits(7, 3) + 8)?;
                match funct2 {
                    0b00 => {
                        let shamt = (value.bits(12, 1) << 5) | value.bits(2, 5);
                        Ok(Self::Srli {
                            rd: rd_creg,
                            rs1: rd_creg,
                            imm: shamt as u8,
                        })
                    }
                    0b01 => {
                        let shamt = (value.bits(12, 1) << 5) | value.bits(2, 5);
                        Ok(Self::Srai {
                            rd: rd_creg,
                            rs1: rd_creg,
                            imm: shamt as u8,
                        })
                    }
                    0b10 => {
                        let imm =
                            ((value.bits(12, 1) as i32) << 31) >> 26 | value.bits(2, 5) as i32;
                        Ok(Self::Andi {
                            rd: rd_creg,
                            rs1: rd_creg,
                            imm,
                        })
                    }
                    0b11 => {
                        let bit12 = value.bits(12, 1);
                        let funct1 = value.bits(5, 2);
                        let rs2_creg = Self::try_parse_reg(value.bits(2, 3) + 8)?;
                        match (bit12, funct1) {
                            (0, 0b00) => Ok(Self::Sub {
                                rd: rd_creg,
                                rs1: rd_creg,
                                rs2: rs2_creg,
                            }),
                            (0, 0b01) => Ok(Self::Xor {
                                rd: rd_creg,
                                rs1: rd_creg,
                                rs2: rs2_creg,
                            }),
                            (0, 0b10) => Ok(Self::Or {
                                rd: rd_creg,
                                rs1: rd_creg,
                                rs2: rs2_creg,
                            }),
                            (0, 0b11) => Ok(Self::And {
                                rd: rd_creg,
                                rs1: rd_creg,
                                rs2: rs2_creg,
                            }),
                            (1, 0b00) => Ok(Self::Subw {
                                rd: rd_creg,
                                rs1: rd_creg,
                                rs2: rs2_creg,
                            }),
                            (1, 0b01) => Ok(Self::Addw {
                                rd: rd_creg,
                                rs1: rd_creg,
                                rs2: rs2_creg,
                            }),
                            _ => Err(InstructionError::InvalidInstruction),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            5 => {
                let offset11 = value.bits(12, 1) << 11
                    | value.bits(8, 1) << 10
                    | value.bits(9, 2) << 8
                    | value.bits(6, 1) << 7
                    | value.bits(7, 1) << 6
                    | value.bits(2, 1) << 5
                    | value.bits(11, 1) << 4
                    | value.bits(3, 3) << 1;
                Ok(Self::Jal {
                    rd: GeneralRegisterName::Zero,
                    imm: if offset11 & (1 << 11) != 0 {
                        (offset11 | !0 << 12) as i16 as i32
                    } else {
                        offset11 as i16 as i32
                    },
                })
            }
            6 => {
                let rs1 = Self::try_parse_reg(value.bits(7, 3) + 8)?;
                let imm = ((value.bits(12, 1) as i32) << 31) >> 23
                    | (value.bits(5, 2) as i32) << 6
                    | (value.bits(2, 1) as i32) << 5
                    | (value.bits(10, 2) as i32) << 3
                    | (value.bits(3, 2) as i32) << 1;
                Ok(Self::Beq {
                    rs1,
                    rs2: GeneralRegisterName::Zero,
                    imm,
                })
            }
            7 => {
                let rs1 = Self::try_parse_reg(value.bits(7, 3) + 8)?;
                let imm = ((value.bits(12, 1) as i32) << 31) >> 23
                    | (value.bits(5, 2) as i32) << 6
                    | (value.bits(2, 1) as i32) << 5
                    | (value.bits(10, 2) as i32) << 3
                    | (value.bits(3, 2) as i32) << 1;
                Ok(Self::Bne {
                    rs1,
                    rs2: GeneralRegisterName::Zero,
                    imm,
                })
            }
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    #[inline(always)]
    fn try_compressed_q2(value: u16) -> InstructionResult<Self> {
        let funct3 = value.bits(13, 3);
        match funct3 {
            0 => {
                let shamt = (value.bits(12, 1) << 5) | value.bits(2, 5);
                let rd = Self::try_parse_reg(value.bits(7, 5))?;
                Ok(Self::Slli {
                    rd,
                    rs1: rd,
                    imm: shamt as u8,
                })
            }
            1 => unimplemented!("C.FLDSP"), // we dont support floating point instructions
            2 => {
                let imm =
                    (value.bits(2, 2) << 6) | (value.bits(12, 1) << 5) | (value.bits(4, 3) << 2);
                let rd = Self::try_parse_reg(value.bits(7, 5))?;
                if rd == GeneralRegisterName::Zero {
                    return Err(InstructionError::InvalidInstruction);
                }
                Ok(Self::Lw {
                    rd,
                    rs1: GeneralRegisterName::Sp,
                    imm: imm as i32,
                })
            }
            3 => {
                let imm =
                    (value.bits(2, 3) << 6) | (value.bits(12, 1) << 5) | (value.bits(5, 2) << 3);
                let rd = Self::try_parse_reg(value.bits(7, 5))?;
                if rd == GeneralRegisterName::Zero {
                    return Err(InstructionError::InvalidInstruction);
                }
                Ok(Self::Ld {
                    rd,
                    rs1: GeneralRegisterName::Sp,
                    imm: imm as i32,
                })
            }
            4 => {
                let bit12 = value.bits(12, 1);
                let rs1 = Self::try_parse_reg(value.bits(7, 5))?;
                let rs2 = Self::try_parse_reg(value.bits(2, 5))?;

                if bit12 == 0 {
                    if rs2 == GeneralRegisterName::Zero {
                        if rs1 == GeneralRegisterName::Zero {
                            return Err(InstructionError::InvalidInstruction);
                        }
                        Ok(Self::Jalr {
                            rd: GeneralRegisterName::Zero,
                            rs1,
                            imm: 0,
                        })
                    } else {
                        if rs1 == GeneralRegisterName::Zero {
                            return Err(InstructionError::InvalidInstruction);
                        }
                        Ok(Self::Add {
                            rd: rs1,
                            rs1: GeneralRegisterName::Zero,
                            rs2,
                        })
                    }
                } else if rs1 == GeneralRegisterName::Zero && rs2 == GeneralRegisterName::Zero {
                    Ok(Self::Ebreak)
                } else if rs2 == GeneralRegisterName::Zero {
                    if rs1 == GeneralRegisterName::Zero {
                        return Err(InstructionError::InvalidInstruction);
                    }
                    Ok(Self::Jalr {
                        rd: GeneralRegisterName::Ra,
                        rs1,
                        imm: 0,
                    })
                } else {
                    if rs1 == GeneralRegisterName::Zero || rs2 == GeneralRegisterName::Zero {
                        return Err(InstructionError::InvalidInstruction);
                    }
                    Ok(Self::Add { rd: rs1, rs1, rs2 })
                }
            }
            5 => unimplemented!("C.FSDSP"), // we dont support floating point instructions
            6 => {
                let imm = (value.bits(7, 2) << 6) | (value.bits(9, 4) << 2);
                Ok(Self::Sw {
                    rs1: GeneralRegisterName::Sp,
                    rs2: Self::try_parse_reg(value.bits(2, 5))?,
                    imm: imm as i32,
                })
            }
            7 => {
                let rs2 = Self::try_parse_reg(value.bits(2, 5))?;
                let imm = (value.bits(7, 3) << 6) | (value.bits(10, 3) << 3);
                Ok(Self::Sd {
                    imm: imm as i32,
                    rs1: GeneralRegisterName::Sp,
                    rs2,
                })
            }
            _ => Err(InstructionError::InvalidInstruction),
        }
    }

    fn try_from_compressed(value: u16) -> InstructionResult<Self> {
        let op = value.bits(0, 2);
        match op {
            0 => Self::try_compressed_q0(value),
            1 => Self::try_compressed_q1(value),
            2 => Self::try_compressed_q2(value),
            _ => unreachable!(
                "cannot be reached as this function is only called for compressed instructions, which have opcodes 0, 1, or 2"
            ),
        }
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
