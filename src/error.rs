#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Parameter Error: {0}")]
    InvalidParameter(String),
    #[error("Generic Error : {0}")]
    Generic(String),
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
    ( $x:expr ) => {
        Err($x.into())
    };
}
