use crate::{config::Config, langs::registry::get_type_from_ext, stats::FileStat, syntax::LexerFactory};

use std::path::Path;
use std::io::{BufReader, Read, Seek};
use std::fs::File;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

#[derive(Debug, Clone)]
pub struct Counter {
    config: Config,
}

impl Counter {
    fn is_binary_file(file: &mut File) -> bool {
        let mut buffer = [0; 1024];
        match file.read(&mut buffer) {
            Ok(0) => false,
            Ok(n) => {
                let _ = file.seek(std::io::SeekFrom::Start(0));
                buffer.iter().take(n).any(|&b| b == 0)
            }
            Err(_) => false,
        }
    }
    pub fn new(config: Config) -> Self {
        Counter {
            config
        }
    }

    pub fn count(&self, path: impl AsRef<Path>) -> Result<FileStat, CounterError> {
        let ext = path.as_ref().extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let lang_type = get_type_from_ext(&ext)
            .ok_or_else(|| CounterError::LexError(format!("Unknown language for extension: {}", ext)))?;
        let mut file = File::open(path.as_ref()).map_err(|e| CounterError::IoError(e.to_string()))?;

        if Self::is_binary_file(&mut file) {
            return Err(CounterError::BinaryFile);
        }

        let reader = DecodeReaderBytesBuilder::new()
            .encoding(None)
            .build(file);

        let mut buf_reader = BufReader::new(reader);

        let lexer = LexerFactory::get_lexer(lang_type)
            .ok_or_else(|| CounterError::LexError("Unknown language".to_string()))?;

        let mut stat = lexer.lex(&mut buf_reader).map_err(|e| CounterError::LexError(e))?;
        stat.lang = lang_type;
        stat.path = path.as_ref().display().to_string();
        stat.name = path.as_ref().file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        Ok(stat)
    }

    /// 异步版本的计数函数
    pub async fn count_async(&self, path: impl AsRef<Path> + Send) -> Result<FileStat, CounterError> {
        // 使用spawn_blocking在阻塞线程中执行同步代码
        let path = path.as_ref().to_path_buf();
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let counter = Counter::new(config);
            counter.count(path)
        }).await
        .map_err(|e| CounterError::IoError(format!("Task join error: {}", e)))?
    }
} 

#[derive(Debug)]
pub enum CounterError {
    IoError(String),
    LexError(String),
    BinaryFile,
}

impl std::fmt::Display for CounterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CounterError::IoError(msg) => write!(f, "IO Error: {}", msg),
            CounterError::LexError(msg) => write!(f, "Lexing Error: {}", msg),
            CounterError::BinaryFile => write!(f, "Binary file detected"),
        }
    }
}

impl std::error::Error for CounterError {}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_counter() {
        let counter = Counter::new(Config::new());
        let stat = counter.count("./src/counter.rs").unwrap();

        assert_eq!(stat.path, "./src/counter.rs");
        assert_eq!(stat.name, "counter.rs");
        assert_eq!(stat.lines, 125);
    }
}