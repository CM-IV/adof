use anyhow::{anyhow, ensure, Context, Result};
use reqwest::Client;
use url::Url;

pub async fn github_repo(repo_link: &str) -> Result<()> {
    let url = Url::parse(repo_link).map_err(|_| anyhow!("Invalid GitHub URL: {:?}", repo_link))?;

    ensure!(
        url.host_str() == Some("github.com"),
        "Invalid GitHub link, expected 'github.com' domain. Link: {:?}",
        repo_link
    );

    let mut path_segments = url.path_segments().ok_or_else(|| {
        anyhow!(
            "Invalid GitHub URL structure, unable to parse path. Link: {:?}",
            repo_link
        )
    })?;

    let _user = path_segments.next().ok_or_else(|| {
        anyhow!(
            "Invalid GitHub link structure, missing user segment. Link: {:?}",
            repo_link
        )
    })?;
    let _repo = path_segments.next().ok_or_else(|| {
        anyhow!(
            "Invalid GitHub link structure, missing repository segment. Link: {:?}",
            repo_link
        )
    })?;

    ensure!(
        path_segments.next().is_none(),
        "Invalid GitHub URL: extra path segments found. Link: {:?}",
        repo_link
    );

    let client = Client::new();
    let response = client.head(url.as_str()).send().await.context(
        "Failed to validate the GitHub link. Check your internet connection and try again.",
    )?;

    ensure!(
        response.status().is_success(),
        "GitHub repository not found or invalid permissions. Link: {:?}",
        repo_link
    );

    Ok(())
}

pub fn auto_update_time(min: u64) -> Result<()> {
    ensure!(
        min >= 11,
        "The auto-update interval must be at least 10 minutes. Value provided: {} minutes",
        min
    );
    Ok(())
}

pub fn log_counts(num: u8) -> Result<()> {
    ensure!(
        num <= 100,
        "Requested log count is too high. Please provide a value less than or equal to 100. Value provided: {}",
        num
    );
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_update_time() {
        assert!(auto_update_time(11).is_ok());
        assert!(auto_update_time(10).is_err());
        assert!(auto_update_time(1).is_err());
    }

    #[test]
    fn test_log_counts() {
        assert!(log_counts(1).is_ok());
        assert!(log_counts(100).is_ok());
        assert!(log_counts(101).is_err());
    }

    #[tokio::test]
    async fn test_github_repo() {
        assert!(github_repo("github").await.is_err());
        assert!(github_repo("github.com").await.is_err());
        assert!(github_repo("https://github.com/fnabinash/adof.gi").await.is_err());
        assert!(github_repo("https://github.com/fnabinash/adof.git").await.is_ok());
    }
}
