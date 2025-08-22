use base64::{engine::general_purpose, Engine as _};
use chrono::Local;
use dotenvy::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::io::{self, Write};
use std::env;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct GitTreeResponse {
    tree: Vec<GitTreeEntry>,
}

#[derive(Debug, Deserialize)]
struct GitTreeEntry {
    path: String,
    #[serde(rename = "type")]
    kind: String, // "blob" or "tree"
}

#[derive(Debug, Deserialize)]
struct FileContent {
    content: String,
    encoding: String,
}

#[derive(Debug, Deserialize)]
struct GitHubError {
    message: String,
    #[allow(dead_code)]
    documentation_url: Option<String>,
}

// Helper function for exclusion patterns
fn should_skip_path(path: &str) -> bool {
    path.starts_with("target/")
        || path.starts_with("node_modules/")
        || path.starts_with("dist/")
        || path.contains("/.DS_Store")
        || path.starts_with("build/")
        || path.ends_with(".dll")
        || path.ends_with(".so")
        || path.ends_with(".dylib")
        || path.ends_with(".exe")
        || path.ends_with(".bin")
        || path.starts_with(".git/")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let token = env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN environment variable not set");

    // Display welcome message
    println!("🚀 GitHub Repository Exporter");
    println!("================================\n");

    // Use the interactive menu to get repository information
    let (owner, repo) = get_repository_info()?;

    let client = Client::new();

    println!("📂 Fetching repository contents for {}/{}...", owner, repo);

    // Fetch files with better error handling
    match fetch_repo_files(&client, &token, &owner, &repo).await {
        Ok(files) => {
            if files.is_empty() {
                println!("⚠️  No files found in the repository or all files were skipped.");
                return Ok(());
            }

            // Save export
            let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
            let output_file = format!("{}_repo_export_{}.md", repo, timestamp);
            let mut file = File::create(&output_file)?;

            writeln!(file, "# Repository Export: {}/{}\n", owner, repo)?;
            for (path, content) in files {
                writeln!(file, "## {}\n", path)?;
                writeln!(file, "```text\n{}\n```", content)?;
            }

            println!("✅ Export complete: {}", output_file);
        }
        Err(e) => {
            println!("❌ Failed to fetch repository: {}", e);
            println!("\nPossible causes:");
            println!("  • Repository doesn't exist (check for typos)");
            println!("  • Repository is private (check your GITHUB_TOKEN permissions)");
            println!("  • Network issues or GitHub API is down");
            println!("  • Rate limit exceeded");
        }
    }

    Ok(())
}

async fn fetch_repo_files(
    client: &Client,
    token: &str,
    owner: &str,
    repo: &str,
) -> anyhow::Result<Vec<(String, String)>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/git/trees/HEAD?recursive=1",
        owner,
        repo
    );

    println!("🔍 Checking repository existence...");

    let response = client
        .get(&url)
        .bearer_auth(token)
        .header("User-Agent", "Rust-GitHubClient")
        .send()
        .await?;

    // Check response status before trying to parse
    if !response.status().is_success() {
        let status = response.status();

        // Try to get error message from GitHub
        return if let Ok(error) = response.json::<GitHubError>().await {
            Err(anyhow::anyhow!(
                "GitHub API error ({}): {}",
                status,
                error.message
            ))
        } else {
            Err(anyhow::anyhow!(
                "GitHub API returned status: {}",
                status
            ))
        };
    }

    let tree: GitTreeResponse = response.json().await?;

    let mut files = Vec::new();

    // Filter eligible files first
    let eligible_files: Vec<_> = tree.tree.iter()
        .filter(|entry| entry.kind == "blob" && !should_skip_path(&entry.path))
        .collect();

    println!("Found {} files to process", eligible_files.len());

    for (i, entry) in eligible_files.iter().enumerate() {
        println!("📄 Processing file {}/{}: {}", i + 1, eligible_files.len(), entry.path);

        // Fetch file content
        let content_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            owner,
            repo,
            entry.path
        );

        let response = client
            .get(&content_url)
            .bearer_auth(token)
            .header("User-Agent", "Rust-GitHubClient")
            .send()
            .await?;

        // Handle rate limit
        if response.status() == 403 {
            if let Some(remaining) = response.headers().get("x-ratelimit-remaining") {
                if let Ok(remaining_str) = remaining.to_str() {
                    if let Ok(remaining_count) = remaining_str.parse::<i32>() {
                        if remaining_count == 0 {
                            println!("⚠️  Rate limit hit, waiting 60 seconds...");
                            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                            continue;
                        }
                    }
                }
            }
            println!("⚠️  Access forbidden for file: {}", entry.path);
            continue;
        }

        if response.status().is_success() {
            // Add file size limits to avoid huge files
            if let Some(content_length) = response.headers().get("content-length") {
                if let Ok(size_str) = content_length.to_str() {
                    if let Ok(size) = size_str.parse::<u64>() {
                        if size > 1_000_000 { // Skip files > 1MB
                            println!("⭕️ Skipping large file ({}KB): {}", size / 1024, entry.path);
                            continue;
                        }
                    }
                }
            }

            let file: FileContent = response.json().await?;
            if file.encoding == "base64" {
                // Decode base64 content
                match general_purpose::STANDARD.decode(file.content.replace('\n', "")) {
                    Ok(decoded) => {
                        // Try UTF-8; if it fails, skip (binary file)
                        if let Ok(text) = String::from_utf8(decoded) {
                            files.push((entry.path.clone(), text));
                        } else {
                            println!("⭕️ Skipping binary file: {}", entry.path);
                        }
                    }
                    Err(e) => {
                        println!("⚠️  Failed to decode {}: {}", entry.path, e);
                    }
                }
            }
        } else {
            println!("⚠️  Failed to fetch {}: {}", entry.path, response.status());
        }

        // Small delay to be nice to GitHub API
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    Ok(files)
}

/// Parses repository input which can be either a GitHub URL or owner/repo format.
///
/// Supports multiple input formats:
/// - Full URL: https://github.com/tidynest/security_toolkit
/// - Short URL: github.com/tidynest/security_toolkit
/// - Owner/repo: tidynest/security_toolkit
/// - Interactive: tidynest (will prompt for repo name)
///
/// # Arguments
/// * `input` - User input string
///
/// # Returns
/// Tuple of (owner, repo) strings or error for invalid format
#[allow(dead_code)]
fn parse_repo_input(input: &str) -> anyhow::Result<(String, String)> {
    let input = input.trim();

    // Handle GitHub URLs
    if input.starts_with("https://github.com/") || input.starts_with("http://github.com/") {
        let path = input
            .strip_prefix("https://github.com/")
            .or_else(|| input.strip_prefix("http://github.com/"))
            .unwrap();
        return parse_owner_repo_path(path);
    }

    // Handle github.com/ URLs without protocol
    if input.starts_with("github.com/") {
        let path = input.strip_prefix("github.com/").unwrap();
        return parse_owner_repo_path(path);
    }

    // Handle owner/repo format
    if input.contains('/') {
        return parse_owner_repo_path(input);
    }

    // Handle just owner - prompt for repo name
    print!("Enter repository name: ");
    io::stdout().flush()?;
    let mut repo = String::new();
    io::stdin().read_line(&mut repo)?;
    let repo = repo.trim();

    if repo.is_empty() {
        return Err(anyhow::anyhow!("Repository name cannot be empty"));
    }

    Ok((input.to_string(), repo.to_string()))
}

/// Parses owner/repo from a path string.
///
/// # Arguments
/// * `path` - Path portion like "tidynest/security_toolkit" or "tidynest/security_toolkit.git"
///
/// # Returns
/// Tuple of (owner, repo) or error if format is invalid
fn parse_owner_repo_path(path: &str) -> anyhow::Result<(String, String)> {
    let path = path.trim_end_matches(".git"); // Remove .git suffix if present
    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() != 2 {
        return Err(anyhow::anyhow!(
            "Invalid repository format. Expected 'owner/repo' or GitHub URL"
        ));
    }

    let owner = parts[0].trim();
    let repo = parts[1].trim();

    if owner.is_empty() || repo.is_empty() {
        return Err(anyhow::anyhow!("Owner and repository name cannot be empty"));
    }

    Ok((owner.to_string(), repo.to_string()))
}

// The following functions are kept for potential future use with an interactive menu
// Mark them as allowed dead code to suppress warnings

// Interactive menu system for getting repository information
fn get_repository_info() -> anyhow::Result<(String, String)> {
    println!("Select input method:");
    println!("  1. Enter full GitHub URL");
    println!("  2. Enter in format 'owner/repo'");
    println!("  3. Enter owner and repo separately");
    println!("  4. Exit\n");

    loop {
        print!("Choose an option (1-4): ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                // Full GitHub URL
                print!("\n🔗 Enter the full GitHub URL: ");
                io::stdout().flush()?;
                let mut url = String::new();
                io::stdin().read_line(&mut url)?;
                let url = url.trim();

                if url.is_empty() {
                    println!("❌ URL cannot be empty. Please try again.\n");
                    continue;
                }

                return parse_github_url(url);
            }

            "2" => {
                // Owner/repo format
                print!("\n📋 Enter in format 'owner/repo': ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim();

                if input.is_empty() {
                    println!("❌ Input cannot be empty. Please try again.\n");
                    continue;
                }

                return parse_owner_repo_format(input);
            }

            "3" => {
                // Separate owner and repo
                print!("\n👤 Enter GitHub username/organization: ");
                io::stdout().flush()?;
                let mut owner = String::new();
                io::stdin().read_line(&mut owner)?;
                let owner = owner.trim();

                if owner.is_empty() {
                    println!("❌ Username/organization cannot be empty. Please try again.\n");
                    continue;
                }

                print!("📁 Enter repository name: ");
                io::stdout().flush()?;
                let mut repo = String::new();
                io::stdin().read_line(&mut repo)?;
                let repo = repo.trim();

                if repo.is_empty() {
                    println!("❌ Repository name cannot be empty. Please try again.\n");
                    continue;
                }

                println!("\n✅ Repository: {}/{}\n", owner, repo);
                return Ok((owner.to_string(), repo.to_string()));
            }

            "4" => {
                println!("\n👋 Goodbye!");
                std::process::exit(0);
            }

            _ => {
                println!("❌ Invalid choice. Please enter 1, 2, 3, or 4.\n");
                continue;
            }
        }
    }
}

fn parse_github_url(url: &str) -> anyhow::Result<(String, String)> {
    let url = url.trim();

    // Handle different URL formats
    let path = if url.starts_with("https://github.com/") {
        url.strip_prefix("https://github.com/").unwrap()
    } else if url.starts_with("http://github.com/") {
        url.strip_prefix("http://github.com/").unwrap()
    } else if url.starts_with("github.com/") {
        url.strip_prefix("github.com/").unwrap()
    } else {
        return Err(anyhow::anyhow!("Invalid GitHub URL. Expected format: https://github.com/owner/repo"));
    };

    parse_owner_repo_path(path)
}

#[allow(dead_code)]
fn parse_owner_repo_format(input: &str) -> anyhow::Result<(String, String)> {
    if !input.contains('/') {
        return Err(anyhow::anyhow!("Invalid format. Expected 'owner/repo' (e.g., 'tidynest/security_toolkit')"));
    }

    parse_owner_repo_path(input)
}