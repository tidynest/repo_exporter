// ============= src/ui/menu.rs =============
use crate::input::parser::{parse_github_url, parse_owner_repo_format};
use anyhow::Result;
use std::io::{self, Write};

/// Interactive menu system for getting repository information
pub fn get_repository_info() -> Result<(String, String)> {
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