/// 保存错误类型
#[derive(Debug)]
pub enum SaveError {
    Io(std::io::Error),
    Json(serde_json::Error),
    UnsupportedFormat,
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Io(e) => write!(f, "IO error: {}", e),
            SaveError::Json(e) => write!(f, "JSON error: {}", e),
            SaveError::UnsupportedFormat => write!(f, "Unsupported output format for saving"),
        }
    }
}

impl std::error::Error for SaveError {}