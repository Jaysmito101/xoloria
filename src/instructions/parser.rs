use crate::instructions::{Instruction, InstructionError, InstructionResult};

impl TryFrom<u32> for Instruction {
    type Error = InstructionError;

    fn try_from(_value: u32) -> InstructionResult<Self> {
        Ok(Self::Noop)
    }
}
