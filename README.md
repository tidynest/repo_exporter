# 📦 GitHub Repository Exporter

A fast and efficient Rust CLI tool to export GitHub repositories to Markdown format. 

Perfect for documentation, archiving, or analyzing repository contents offline.


## ✨ Features

- **Multiple Input Methods**: Accept GitHub URLs, owner/repo format, or interactive input
- **Smart Filtering**: Automatically excludes binary files, build artifacts, and common ignored paths
- **Progress Tracking**: Real-time progress indicators for large repositories
- **Rate Limit Handling**: Intelligent handling of GitHub API rate limits
- **Error Recovery**: Robust error handling with retry logic
- **Markdown Export**: Clean, readable Markdown output with syntax highlighting support

## 🚀 Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- GitHub Personal Access Token (for private repos or higher rate limits)

### From Source

```bash
git clone https://github.com/tidynest/repo_exporter.git
cd repo_exporter
cargo build --release
```

The binary will be available at `target/release/repo_exporter`

### Using Cargo

```bash
cargo install --git https://github.com/tidynest/repo_exporter.git
```

## 🔧 Configuration

Set your GitHub token as an environment variable:

```bash
export GITHUB_TOKEN="your_github_token_here"
```

Or create a `.env` file in the project directory:

```
GITHUB_TOKEN=your_github_token_here
```

## 📖 Usage

Run the tool:

```bash
cargo run
# or if installed
repo_exporter
```

### Input Methods

The tool offers three ways to specify a repository:

1. **Full GitHub URL**
   ```
   https://github.com/tidynest/security_toolkit
   ```

2. **Owner/Repository Format**
   ```
   tidynest/security_toolkit
   ```

3. **Interactive Input**
    - Enter username/organization separately
    - Then enter repository name

### Example Output

The tool generates a Markdown file with the format:
```
{repo_name}_repo_export_{timestamp}.md
```

Example: `security_toolkit_repo_export_20250822_200405.md`

## 📁 What Gets Exported

### Included
- Source code files (.rs, .py, .js, .java, etc.)
- Configuration files (Cargo.toml, package.json, etc.)
- Documentation (README, LICENSE, etc.)
- Scripts and text files

### Automatically Excluded
- Binary files and executables
- Build directories (target/, node_modules/, dist/)
- Version control (.git/)
- Large files (>1MB)
- System files (.DS_Store, Thumbs.db)

## 🎯 Use Cases

- **Documentation**: Create offline documentation of repository structure
- **Code Review**: Export repos for offline code review
- **Archiving**: Create snapshots of repositories at specific points in time
- **Analysis**: Prepare repositories for AI/ML analysis
- **Teaching**: Share complete repository contents in a single file

## 🛠️ Development

### Project Structure

```
repo_exporter/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Dependencies and metadata
├── .env.example         # Example environment configuration
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

### Key Dependencies

- `reqwest` - HTTP client for GitHub API
- `tokio` - Async runtime
- `base64` - Decode file contents from GitHub API
- `chrono` - Timestamp generation
- `dotenvy` - Environment variable management
- `serde` - JSON deserialization

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📊 Performance

- Processes repositories with thousands of files efficiently
- Implements concurrent file fetching with rate limit respect
- Memory-efficient streaming for large files
- Automatic retry on transient failures

## 🔒 Security

- Never stores or logs your GitHub token
- Validates all user inputs
- Sanitizes file paths and names
- Respects GitHub API rate limits

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with Rust 🦀
- Uses the GitHub REST API v3
- Inspired by the need for better repository documentation tools

## 📞 Contact

**Author**: Hans Eric Luiz Jingryd  
**GitHub**: [@tidynest](https://github.com/tidynest)  
**Email**: tidynest@proton.me

---

⭐ If you find this tool useful, please consider giving it a star on GitHub!