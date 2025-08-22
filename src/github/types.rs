// ============= src/github/types.rs =============
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitTreeResponse {
    pub tree: Vec<GitTreeEntry>,
}

#[derive(Debug, Deserialize)]
pub struct GitTreeEntry {
    pub path: String,
    #[serde(rename = "type")]
    pub kind: String, // "blob" or "tree"
}

#[derive(Debug, Deserialize)]
pub struct FileContent {
    pub content: String,
    pub encoding: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubError {
    pub message: String,
    #[allow(dead_code)]
    pub documentation_url: Option<String>,
}