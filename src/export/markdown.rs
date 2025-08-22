// ============= src/export/markdown.rs =============
use anyhow::Result;
use chrono::Local;
use std::fs::File;
use std::io::Write;

/// Exports repository files to a Markdown file
pub fn export_to_markdown(
    owner: &str,
    repo: &str,
    files: Vec<(String, String)>
) -> Result<String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let output_file = format!("{}_repo_export_{}.md", repo, timestamp);
    let mut file = File::create(&output_file)?;

    writeln!(file, "# Repository Export: {}/{}\n", owner, repo)?;

    for (path, content) in files {
        writeln!(file, "## {}\n", path)?;
        writeln!(file, "```text\n{}\n```", content)?;
    }

    Ok(output_file)
}