use crate::instructions::{InstructionError, InstructionResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpcodeName {
    // RV32I Base
    Load = 3,
    OpImm = 19,
    Auipc = 23,
    Store = 35,
    OpReg = 51,
    Lui = 55,
    Branch = 99,
    Jalr = 103,
    Jal = 111,

    // RV64I Extensions
    OpImm64 = 27,
    OpReg64 = 59,

    // A Extension
    Atomic = 47,

    // System, Zicsr, Zifencei Extensions
    Fence = 15,
    System = 115,
}

impl TryFrom<u32> for OpcodeName {
    type Error = InstructionError;

    #[inline(always)]
    fn try_from(value: u32) -> InstructionResult<Self> {
        match value & 0x7f {
            3 => Ok(Self::Load),
            19 => Ok(Self::OpImm),
            23 => Ok(Self::Auipc),
            35 => Ok(Self::Store),
            51 => Ok(Self::OpReg),
            55 => Ok(Self::Lui),
            99 => Ok(Self::Branch),
            103 => Ok(Self::Jalr),
            111 => Ok(Self::Jal),
            27 => Ok(Self::OpImm64),
            59 => Ok(Self::OpReg64),
            47 => Ok(Self::Atomic),
            15 => Ok(Self::Fence),
            115 => Ok(Self::System),
            other => Err(InstructionError::UnknownOpcode(other as u8)),
        }
    }
}
