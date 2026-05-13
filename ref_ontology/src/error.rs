use thiserror::Error;

#[derive(Debug, Error)]
pub enum RefOntologyError {
    /// Failed to perform I/O (open/read the file).
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
