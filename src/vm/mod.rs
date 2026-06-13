use crate::Address;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmOutput {
    NextInstruction,
    Jump(Address),
}

pub type VmResult = std::result::Result<VmOutput, VmError>;

pub mod jump;
pub mod load;
