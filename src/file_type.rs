use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Local(PathBuf),
    Url(String),
}

impl From<String> for FileType {
    fn from(file_path: String) -> Self {
        if file_path.starts_with("http") || file_path.starts_with("https") {
            return FileType::Url(file_path);
        }

        FileType::Local(PathBuf::from(file_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handles_local_path() {
        let ft = FileType::from("../local/path".to_string());

        assert!(ft == FileType::Local(PathBuf::from("../local/path".to_string())))
    }
    #[test]
    fn handles_url() {
        let ft = FileType::from(
            "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string(),
        );

        assert!(
            ft == FileType::Url(
                "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string()
            )
        )
    }
}
