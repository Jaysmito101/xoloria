use crate::{
    bus::{Address, BusError},
    instructions::InstructionError,
    registers::RegisterError,
    vm::VmError,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Instruction Error {0:?}")]
    InstructionError(InstructionError),
    #[error("Bus Error {0:?}")]
    BusError(BusError),
    #[error("VM Error {0:?}")]
    VmError(VmError),
    #[error("Register Error {0:?}")]
    RegisterError(RegisterError),
    #[error("Allocation Failed: {0}")]
    AllocationFailed(#[from] std::collections::TryReserveError),
    #[error("Invalid Mapping for Address {0}")]
    InvalidMapping(Address),
    #[error("Invalid Parameter Error: {0}")]
    InvalidParameter(String),
    #[error("Generic Error : {0}")]
    Generic(String),
    #[error("Thread Spawn Failed: {0}")]
    ThreadSpawnFailed(#[from] std::io::Error),
    #[error("Thread Join Failed")]
    ThreadJoinFailed,
}

#[derive(Debug)]
pub struct ErrorReport {
    inner: Error,
    trace: std::backtrace::Backtrace,
}

impl From<Error> for ErrorReport {
    fn from(value: Error) -> Self {
        Self {
            inner: value,
            trace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl From<BusError> for ErrorReport {
    fn from(value: BusError) -> Self {
        Self {
            inner: Error::BusError(value),
            trace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl From<InstructionError> for ErrorReport {
    fn from(value: InstructionError) -> Self {
        Self {
            inner: Error::InstructionError(value),
            trace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl From<VmError> for ErrorReport {
    fn from(value: VmError) -> Self {
        Self {
            inner: Error::VmError(value),
            trace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl From<RegisterError> for ErrorReport {
    fn from(value: RegisterError) -> Self {
        Self {
            inner: Error::RegisterError(value),
            trace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl std::fmt::Display for ErrorReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nBacktrace:\n{}",
            self.inner,
            match self.trace.status() {
                std::backtrace::BacktraceStatus::Captured => format!("{}", self.trace),
                std::backtrace::BacktraceStatus::Unsupported =>
                    "Backtrace unsupported on this platform".into(),
                std::backtrace::BacktraceStatus::Disabled =>
                    "Backtrace capture is disabled, enable RUST_BACKTRACE=1 to capture backtraces"
                        .into(),
                _ => "Unknown backtrace status".into(),
            }
        )
    }
}

impl std::error::Error for ErrorReport {}

impl ErrorReport {
    pub fn inner(&self) -> &Error {
        &self.inner
    }
}

pub type Result<T> = std::result::Result<T, ErrorReport>;

#[macro_export]
macro_rules! err {
    ($x:expr) => {
        Err($x.into())
    };
}
