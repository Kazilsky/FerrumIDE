use std::path::PathBuf;

/// `ferrum [path]`, default it's present dir
pub fn parse_path() -> String {
    std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .to_string_lossy()
        .into_owned()
}