use crate::{config::Config, langs::{lang_type::LangType, registry::get_type_from_ext}, stats::FileStat, syntax::LexerFactory};

use std::path::Path;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
pub struct Counter {
    config: Config,
}

impl Counter {
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
        let file = File::open(path.as_ref()).map_err(|e| CounterError::IoError(e.to_string()))?;
        let mut reader = BufReader::new(file);
        let lexer = LexerFactory::get_lexer(lang_type)
            .ok_or_else(|| CounterError::LexError("Unknown language".to_string()))?;

        let mut stat = lexer.lex(&mut reader).map_err(|e| CounterError::LexError(e))?;
        stat.lang = lang_type;
        stat.path = path.as_ref().display().to_string();
        stat.name = path.as_ref().file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        Ok(stat)
    }
} 

#[derive(Debug)]
pub enum CounterError {
    IoError(String),
    LexError(String),
}

impl std::fmt::Display for CounterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CounterError::IoError(msg) => write!(f, "IO Error: {}", msg),
            CounterError::LexError(msg) => write!(f, "Lexing Error: {}", msg),
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
        assert_eq!(stat.lines, 41);
    }
}