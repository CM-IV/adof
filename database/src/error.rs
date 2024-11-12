use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DBError {
    #[error("Failed to read the file: {file:?}. Make sure you have the file with enough permissions. Source: {source:?}.")]
    FileError {
        #[source]
        source: io::Error,
        file: String,
    },

    #[error("Something went wrong. Please try again!")]
    UnknownIssue,
}
