use error_stack::{ensure, Report};

use crate::error::AdofError;

pub fn github_repo(repo_link: &str) {}

pub fn auto_update_time(min: u64) -> Result<(), Report<AdofError>> {
    ensure!(min >= 10, AdofError::TooFashUpdateTime {
        expected: 10,
        found: min
    });

    Ok(())
}

pub fn log_counts(num: u8) -> Result<(), Report<AdofError>> {
    ensure!(num <= 100, AdofError::TooManyLogs(num));

    Ok(())
}
