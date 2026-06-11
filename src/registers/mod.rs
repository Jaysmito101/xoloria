pub type Register = u64;

mod misa;
pub use misa::{ISAExtensions, Misa};

mod names;
pub use names::{ControlRegisterName, GeneralRegisterName};
