#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Expected int: {0}")]
    ExpectedInt(#[from] std::num::ParseIntError),
}