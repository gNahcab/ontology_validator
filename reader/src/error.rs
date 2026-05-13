use oxrdfxml::RdfXmlParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    /// Failed to perform I/O (open/read the file).
    #[error("{0}")]
    Io(#[from] std::io::Error),


    /// Wrong File-Content.
    #[error("{0}")]
    FileContent(String),

    /// Failed to parse JSON content.
    #[error("{0}")]
    ParseJSON(#[from] serde_json::Error),

    /// Failed to parse RDF content.
    #[error("{0}")]
    ParseRDF(#[from] RdfXmlParseError),

    /// Failed to read file extension.
    #[error("{0}")]
    FilePath(String),
}
