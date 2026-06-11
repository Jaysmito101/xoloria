mod opcode;
pub use opcode::OpcodeName;

mod instruction;
pub use instruction::Instruction;

mod parser;

mod display;
pub mod payload;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstructionError {
    IllegalInstruction,
}

pub type InstructionResult<T> = std::result::Result<T, InstructionError>;
