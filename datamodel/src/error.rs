use thiserror::Error;
#[derive(Debug, PartialEq, Error)]
pub enum DataModelError {
    #[error("{0}")]
    ParsingError(String),
    #[error("{0}")]
    InputError(String)
}
