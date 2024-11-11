use anyhow::{ensure, Result};
use reqwest::blocking::Client;
use url::Url;

use crate::error::AdofError;

pub fn github_repo(repo_link: &str) -> Result<()> {
    let url = Url::parse(repo_link).map_err(|_| AdofError::InvalidLink(repo_link.to_string()))?;

    if url.host_str() != Some("github.com") {
        return Err(AdofError::InvalidLink(repo_link.to_string()).into());
    }

    let client = Client::new();
    let response = client
        .head(url.as_str())
        .send()
        .map_err(|_| AdofError::UnknownIssue)?;

    ensure!(
        response.status().is_success(),
        AdofError::InvalidLink(repo_link.to_string())
    );

    Ok(())
}

pub fn auto_update_time(min: u64) -> Result<()> {
    ensure!(min >= 11, AdofError::TooFastUpdateTime(min));
    Ok(())
}

pub fn log_counts(num: u8) -> Result<()> {
    ensure!(num <= 100, AdofError::TooManyLogs(num));
    Ok(())
}
