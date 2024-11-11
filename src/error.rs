use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdofError {
    #[error("Your update time is too fast. Expected: \"More than 10\", Found: {0}")]
    TooFastUpdateTime(u64),

    #[error("You are requesting too many logs. Expected: \"Less than 100\", Found: {0}")]
    TooManyLogs(u8),

    #[error("The GitHub link is invalid. Link: {0:?}")]
    InvalidLink(String),

    #[error("Something went wrong. Please try again!")]
    UnknownIssue,
}
