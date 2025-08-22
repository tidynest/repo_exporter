use super::types::*;
use crate::utils::should_skip_path;
use anyhow::{Result, anyhow};
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;

pub struct GitHubClient {
    client: Client,
    token: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    pub async fn fetch_repo_files(&self, owner: &str, repo: &str) -> Result<Vec<(String, String)>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/git/trees/HEAD?recursive=1",
            owner, repo
        );

        println!("🔍 Checking repository existence...");

        let response = self.client
            .get(&url)
            .bearer_auth(&self.token)
            .header("User-Agent", "Rust-GitHubClient")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            return if let Ok(error) = response.json::<GitHubError>().await {
                Err(anyhow!("GitHub API error ({}): {}", status, error.message))
            } else {
                Err(anyhow!("GitHub API returned status: {}", status))
            };
        }

        let tree: GitTreeResponse = response.json().await?;
        let mut files = Vec::new();

        let eligible_files: Vec<_> = tree.tree.iter()
            .filter(|entry| entry.kind == "blob" && !should_skip_path(&entry.path))
            .collect();

        println!("Found {} files to process", eligible_files.len());

        for (i, entry) in eligible_files.iter().enumerate() {
            println!("📄 Processing file {}/{}: {}", i + 1, eligible_files.len(), entry.path);

            if let Ok(content) = self.fetch_file_content(owner, repo, &entry.path).await {
                files.push((entry.path.clone(), content));
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(files)
    }

    async fn fetch_file_content(&self, owner: &str, repo: &str, path: &str) -> Result<String> {
        let content_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            owner, repo, path
        );

        let response = self.client
            .get(&content_url)
            .bearer_auth(&self.token)
            .header("User-Agent", "Rust-GitHubClient")
            .send()
            .await?;

        // Handle rate limit and other error cases...
        // (Move the existing logic here)

        if response.status().is_success() {
            let file: FileContent = response.json().await?;
            if file.encoding == "base64" {
                match general_purpose::STANDARD.decode(file.content.replace('\n', "")) {
                    Ok(decoded) => {
                        if let Ok(text) = String::from_utf8(decoded) {
                            return Ok(text);
                        }
                    }
                    Err(_) => {}
                }
            }
        }

        Err(anyhow!("Failed to fetch file content"))
    }
}