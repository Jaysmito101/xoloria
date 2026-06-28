mod misa;
pub use misa::{ISAExtensions, Misa};

mod mie;
pub use mie::Mie;

mod mip;
pub use mip::Mip;

use std::sync::atomic::AtomicU64;

mod names;
pub use names::{ControlRegisterName, GeneralRegisterName};

mod set;
pub use set::RegisterSet;

mod csr;
pub use csr::ControlStatusRegisters;

use crate::PrivilageMode;

pub type Register = u64;
pub type AtomicRegister = AtomicU64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegisterError {
    UnknownControlRegister(u16),
    InvalidControlRegister(u16),
    UnknownGeneralRegister(u8),
    InvalidCSRWrite(ControlRegisterName, Register, PrivilageMode),
    InvalidCSRRead(ControlRegisterName, PrivilageMode),
    UnprivilegedAccess(ControlRegisterName, PrivilageMode),
}

pub type RegisterResult<T> = std::result::Result<T, RegisterError>;
