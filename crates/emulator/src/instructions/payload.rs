use crate::{
    instructions::{InstructionError, InstructionResult, OpcodeName},
    registers::GeneralRegisterName,
};

pub trait BitsExt {
    fn bits(&self, shift: u32, width: u32) -> u16;
}

impl BitsExt for u32 {
    #[inline(always)]
    fn bits(&self, shift: u32, width: u32) -> u16 {
        ((*self >> shift) & ((1 << width) - 1)) as u16
    }
}

impl BitsExt for u16 {
    #[inline(always)]
    fn bits(&self, shift: u32, width: u32) -> u16 {
        (*self >> shift) & ((1 << width) - 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RType {
    pub(crate) opcode: OpcodeName,
    pub(crate) rd: GeneralRegisterName,
    pub(crate) funct3: u8,
    pub(crate) rs1: GeneralRegisterName,
    pub(crate) rs2: GeneralRegisterName,
    pub(crate) funct7: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IType {
    pub(crate) opcode: OpcodeName,
    pub(crate) rd: GeneralRegisterName,
    pub(crate) funct3: u8,
    pub(crate) rs1: GeneralRegisterName,
    pub(crate) imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SType {
    pub(crate) opcode: OpcodeName,
    pub(crate) funct3: u8,
    pub(crate) rs1: GeneralRegisterName,
    pub(crate) rs2: GeneralRegisterName,
    pub(crate) imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BType {
    pub(crate) opcode: OpcodeName,
    pub(crate) funct3: u8,
    pub(crate) rs1: GeneralRegisterName,
    pub(crate) rs2: GeneralRegisterName,
    pub(crate) imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UType {
    pub(crate) opcode: OpcodeName,
    pub(crate) rd: GeneralRegisterName,
    pub(crate) imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JType {
    pub(crate) opcode: OpcodeName,
    pub(crate) rd: GeneralRegisterName,
    pub(crate) imm: i32,
}

impl TryFrom<u32> for RType {
    type Error = InstructionError;

    #[inline]
    fn try_from(value: u32) -> InstructionResult<Self> {
        let opcode = OpcodeName::try_from(value.bits(0, 7) as u32)?;
        let rd_b = value.bits(7, 5) as u8;
        let rd = GeneralRegisterName::try_from(rd_b)
            .map_err(|_| InstructionError::UnknownRegister(rd_b))?;
        let funct3 = value.bits(12, 3) as u8;
        let rs1_b = value.bits(15, 5) as u8;
        let rs1 = GeneralRegisterName::try_from(rs1_b)
            .map_err(|_| InstructionError::UnknownRegister(rs1_b))?;
        let rs2_b = value.bits(20, 5) as u8;
        let rs2 = GeneralRegisterName::try_from(rs2_b)
            .map_err(|_| InstructionError::UnknownRegister(rs2_b))?;
        let funct7 = value.bits(25, 7) as u8;
        Ok(Self {
            opcode,
            rd,
            funct3,
            rs1,
            rs2,
            funct7,
        })
    }
}

impl TryFrom<u32> for IType {
    type Error = InstructionError;

    #[inline]
    fn try_from(value: u32) -> InstructionResult<Self> {
        let opcode = OpcodeName::try_from(value.bits(0, 7) as u32)?;
        let rd_b = value.bits(7, 5) as u8;
        let rd = GeneralRegisterName::try_from(rd_b)
            .map_err(|_| InstructionError::UnknownRegister(rd_b))?;
        let funct3 = value.bits(12, 3) as u8;
        let rs1_b = value.bits(15, 5) as u8;
        let rs1 = GeneralRegisterName::try_from(rs1_b)
            .map_err(|_| InstructionError::UnknownRegister(rs1_b))?;
        let imm = (value as i32) >> 20;
        Ok(Self {
            opcode,
            rd,
            funct3,
            rs1,
            imm,
        })
    }
}

impl TryFrom<u32> for SType {
    type Error = InstructionError;

    #[inline]
    fn try_from(value: u32) -> InstructionResult<Self> {
        let opcode = OpcodeName::try_from(value.bits(0, 7) as u32)?;
        let funct3 = value.bits(12, 3) as u8;
        let rs1_b = value.bits(15, 5) as u8;
        let rs1 = GeneralRegisterName::try_from(rs1_b)
            .map_err(|_| InstructionError::UnknownRegister(rs1_b))?;
        let rs2_b = value.bits(20, 5) as u8;
        let rs2 = GeneralRegisterName::try_from(rs2_b)
            .map_err(|_| InstructionError::UnknownRegister(rs2_b))?;

        let imm11_5 = value.bits(25, 7) as i32;
        let imm4_0 = value.bits(7, 5) as i32;
        let imm = (imm11_5 << 5) | imm4_0;
        // sign extend imm
        let imm = (imm << 20) >> 20;
        Ok(Self {
            opcode,
            funct3,
            rs1,
            rs2,
            imm,
        })
    }
}

impl TryFrom<u32> for BType {
    type Error = InstructionError;

    #[inline]
    fn try_from(value: u32) -> InstructionResult<Self> {
        let opcode = OpcodeName::try_from(value.bits(0, 7) as u32)?;
        let funct3 = value.bits(12, 3) as u8;
        let rs1_b = value.bits(15, 5) as u8;
        let rs1 = GeneralRegisterName::try_from(rs1_b)
            .map_err(|_| InstructionError::UnknownRegister(rs1_b))?;
        let rs2_b = value.bits(20, 5) as u8;
        let rs2 = GeneralRegisterName::try_from(rs2_b)
            .map_err(|_| InstructionError::UnknownRegister(rs2_b))?;

        let imm12 = value.bits(31, 1) as i32;
        let imm10_5 = value.bits(25, 6) as i32;
        let imm4_1 = value.bits(8, 4) as i32;
        let imm11 = value.bits(7, 1) as i32;
        let imm = (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1);
        // sign extend imm
        let imm = (imm << 19) >> 19;
        Ok(Self {
            opcode,
            funct3,
            rs1,
            rs2,
            imm,
        })
    }
}

impl TryFrom<u32> for UType {
    type Error = InstructionError;

    #[inline]
    fn try_from(value: u32) -> InstructionResult<Self> {
        let opcode = OpcodeName::try_from(value.bits(0, 7) as u32)?;
        let rd_b = value.bits(7, 5) as u8;
        let rd = GeneralRegisterName::try_from(rd_b)
            .map_err(|_| InstructionError::UnknownRegister(rd_b))?;
        let imm = (value & 0xfffff000) as i32;
        Ok(Self { opcode, rd, imm })
    }
}

impl TryFrom<u32> for JType {
    type Error = InstructionError;

    #[inline]
    fn try_from(value: u32) -> InstructionResult<Self> {
        let opcode = OpcodeName::try_from(value.bits(0, 7) as u32)?;
        let rd_b = value.bits(7, 5) as u8;
        let rd = GeneralRegisterName::try_from(rd_b)
            .map_err(|_| InstructionError::UnknownRegister(rd_b))?;

        let imm20 = value.bits(31, 1) as i32;
        let imm10_1 = ((value >> 21) & 0x3ff) as i32;
        let imm11 = value.bits(20, 1) as i32;
        let imm19_12 = value.bits(12, 8) as i32;
        let imm = (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);
        // sign extend imm
        let imm = (imm << 11) >> 11;
        Ok(Self { opcode, rd, imm })
    }
}
