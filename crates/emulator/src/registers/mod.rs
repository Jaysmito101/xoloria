mod misa;
use std::sync::atomic::AtomicU64;

pub use misa::{ISAExtensions, Misa};

mod names;
pub use names::{ControlRegisterName, GeneralRegisterName};

mod set;
pub use set::RegisterSet;

mod csr;
pub use csr::ControlStatusRegisters;

pub type Register = u64;
pub type AtomicRegister = AtomicU64;
