#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic Error : {0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, Error>;
