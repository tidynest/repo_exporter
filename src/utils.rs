// ============= src/utils.rs =============
/// Helper function to determine if a path should be skipped during export
pub fn should_skip_path(path: &str) -> bool {
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