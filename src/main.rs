// ============= src/main.rs =============
use anyhow::Result;
use dotenvy::dotenv;

mod config;
mod export;
mod github;
mod input;
mod ui;
mod utils;

use config::Config;
use export::export_to_markdown;
use github::GitHubClient;
use ui::get_repository_info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let config = Config::load()?;
    let client = GitHubClient::new(config.github_token);

    println!("🚀 GitHub Repository Exporter");
    println!("================================\n");

    let (owner, repo) = get_repository_info()?;

    println!("📂 Fetching repository contents for {}/{}...", owner, repo);

    match client.fetch_repo_files(&owner, &repo).await {
        Ok(files) => {
            if files.is_empty() {
                println!("⚠️  No files found in the repository or all files were skipped.");
                return Ok(());
            }

            let output_file = export_to_markdown(&owner, &repo, files)?;
            println!("✅ Export complete: {}", output_file);
        }
        Err(e) => {
            println!("❌ Failed to fetch repository: {}", e);
            print_error_suggestions();
        }
    }

    Ok(())
}

fn print_error_suggestions() {
    println!("\nPossible causes:");
    println!("  • Repository doesn't exist (check for typos)");
    println!("  • Repository is private (check your GITHUB_TOKEN permissions)");
    println!("  • Network issues or GitHub API is down");
    println!("  • Rate limit exceeded");
}