use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid State Setting")]
    InvalidStateSetting,
    #[error("Invalid Code : {0}")]
    InvalidCode(char),
    #[error("Invalid parser : {0}")]
    InvalidParser(String),
    #[error("Parse Int Error : {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse Float Error : {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

