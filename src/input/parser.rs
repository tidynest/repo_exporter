// ============= src/input/parser.rs =============
use anyhow::{anyhow, Result};
use std::io::{self, Write};

/// Parses repository input which can be either a GitHub URL or owner/repo format.
///
/// Supports multiple input formats:
/// - Full URL: https://github.com/tidynest/security_toolkit
/// - Short URL: github.com/tidynest/security_toolkit
/// - Owner/repo: tidynest/security_toolkit
/// - Interactive: tidynest (will prompt for repo name)
#[allow(dead_code)]
pub fn parse_repo_input(input: &str) -> Result<(String, String)> {
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
        return Err(anyhow!("Repository name cannot be empty"));
    }

    Ok((input.to_string(), repo.to_string()))
}

/// Parses a GitHub URL and extracts owner and repository name
pub fn parse_github_url(url: &str) -> Result<(String, String)> {
    let url = url.trim();

    // Handle different URL formats
    let path = if url.starts_with("https://github.com/") {
        url.strip_prefix("https://github.com/").unwrap()
    } else if url.starts_with("http://github.com/") {
        url.strip_prefix("http://github.com/").unwrap()
    } else if url.starts_with("github.com/") {
        url.strip_prefix("github.com/").unwrap()
    } else {
        return Err(anyhow!("Invalid GitHub URL. Expected format: https://github.com/owner/repo"));
    };

    parse_owner_repo_path(path)
}

/// Parses owner/repo format string
pub fn parse_owner_repo_format(input: &str) -> Result<(String, String)> {
    if !input.contains('/') {
        return Err(anyhow!("Invalid format. Expected 'owner/repo' (e.g., 'tidynest/security_toolkit')"));
    }

    parse_owner_repo_path(input)
}

/// Parses owner/repo from a path string.
///
/// # Arguments
/// * `path` - Path portion like "tidynest/security_toolkit" or "tidynest/security_toolkit.git"
pub fn parse_owner_repo_path(path: &str) -> Result<(String, String)> {
    let path = path.trim_end_matches(".git"); // Remove .git suffix if present
    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() != 2 {
        return Err(anyhow!(
            "Invalid repository format. Expected 'owner/repo' or GitHub URL"
        ));
    }

    let owner = parts[0].trim();
    let repo = parts[1].trim();

    if owner.is_empty() || repo.is_empty() {
        return Err(anyhow!("Owner and repository name cannot be empty"));
    }

    Ok((owner.to_string(), repo.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_url_https() {
        let result = parse_github_url("https://github.com/owner/repo").unwrap();
        assert_eq!(result, ("owner".to_string(), "repo".to_string()));
    }

    #[test]
    fn test_parse_github_url_with_git_suffix() {
        let result = parse_github_url("https://github.com/owner/repo.git").unwrap();
        assert_eq!(result, ("owner".to_string(), "repo".to_string()));
    }

    #[test]
    fn test_parse_owner_repo_format() {
        let result = parse_owner_repo_format("tidynest/repo_exporter").unwrap();
        assert_eq!(result, ("tidynest".to_string(), "repo_exporter".to_string()));
    }

    #[test]
    fn test_parse_owner_repo_path() {
        let result = parse_owner_repo_path("owner/repo");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("owner".to_string(), "repo".to_string()));

        let result = parse_owner_repo_path("invalid");
        assert!(result.is_err());
    }
}
