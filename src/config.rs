// ============= src/config.rs =============
use anyhow::{anyhow, Result};
use std::env;

pub struct Config {
    pub github_token: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let github_token = env::var("GITHUB_TOKEN")
            .map_err(|_| anyhow!("GITHUB_TOKEN environment variable not set"))?;

        Ok(Config { github_token })
    }
}
