#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmError {}

pub type VmResult<T> = std::result::Result<T, VmError>;

pub mod lui;
