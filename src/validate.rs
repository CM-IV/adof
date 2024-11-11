use error_stack::{ensure, Report, ResultExt};
use url::Url;
use reqwest::blocking::Client;

use crate::error::AdofError;

pub fn github_repo(repo_link: &str) -> Result<(), Report<AdofError>> {
    let url = Url::parse(repo_link).change_context(AdofError::InvalidLink(repo_link.to_string()))?;

    if url.host_str() != Some("github.com") {
        return Err(Report::new(AdofError::InvalidLink(repo_link.to_string())));
    }

    let client = Client::new();
    let response = client.head(url.as_str()).send().change_context(AdofError::UnknownIssue)?;

    ensure!(response.status().is_success(), AdofError::InvalidLink(repo_link.to_string()));

    Ok(())
}

pub fn auto_update_time(min: u64) -> Result<(), Report<AdofError>> {
    ensure!(min >= 11, AdofError::TooFastUpdateTime(min));
    Ok(())
}

pub fn log_counts(num: u8) -> Result<(), Report<AdofError>> {
    ensure!(num <= 100, AdofError::TooManyLogs(num));
    Ok(())
}
