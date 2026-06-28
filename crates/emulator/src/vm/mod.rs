use crate::{Address, bus::BusError, registers::RegisterError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmError {
    BusError(BusError),
    RegisterError(RegisterError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmOutput {
    NextInstruction,
    Jump(Address),
}

pub type VmResult = std::result::Result<VmOutput, VmError>;

pub mod atomic;
pub mod branch;
pub mod csr;
pub mod jump;
pub mod load;
pub mod opimm;
pub mod opreg;
pub mod store;
