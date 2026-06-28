mod misa;
use std::sync::atomic::AtomicU64;

pub use misa::{ISAExtensions, Misa};

mod names;
pub use names::{ControlRegisterName, GeneralRegisterName};

pub type Register = u64;
pub type AtomicRegister = AtomicU64;
