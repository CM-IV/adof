use anyhow::{anyhow, ensure, Context, Result};
use reqwest::blocking::Client;
use url::Url;

pub fn github_repo(repo_link: &str) -> Result<()> {
    let url =
        Url::parse(repo_link).map_err(|_| anyhow!("Invalid GitHub link. Link: {:?}", repo_link))?;

    if url.host_str() != Some("github.com") {
        return Err(anyhow!("Invalid GitHub link. Link: {:?}", repo_link));
    }

    let client = Client::new();
    let response = client
        .head(url.as_str())
        .send()
        .context("Failed to validate the Link. Check your internet connection and try again.")?;

    ensure!(
        response.status().is_success(),
        "Invalid GitHub link. Link: {:?}",
        repo_link
    );

    Ok(())
}

pub fn auto_update_time(min: u64) -> Result<()> {
    ensure!(
        min >= 11,
        "You can not set auto update interval less than 10 min. Found: {:?}",
        min
    );
    Ok(())
}

pub fn log_counts(num: u8) -> Result<()> {
    ensure!(
        num <= 100,
        "You are requesting too many logs, Expected: \"Less than 100\", Found: {:?}",
        num
    );
    Ok(())
}
