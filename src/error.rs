use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdofError {
    #[error("Your update time is too fast. Expected: {expected}, Found: {found}")]
    TooFashUpdateTime {
        expected: u8,
        found: u64
    },

    #[error("You are requesting too many logs. Expected: Less than 100, Found: {0}")]
    TooManyLogs(u8)
}
