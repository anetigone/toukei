use std::io::BufRead;

use crate::langs::lang_type::LangType;
use crate::langs::registry::{get_function_regex, get_lang_def};
use crate::stats::FileStat;

use super::lex_status::LexState;

pub trait Lexer: Send + Sync {
    
    fn lex(&self, reader: &mut dyn BufRead) -> Result<FileStat, String>;
}

pub struct DefaultLexer {
    pub lang_type: LangType
}

impl DefaultLexer {
    pub fn new(lang_type: LangType) -> Self {
        DefaultLexer { lang_type }
    }
}

impl Lexer for DefaultLexer {
    fn lex(&self, reader: &mut dyn BufRead) -> Result<FileStat, String> {
        let def = get_lang_def(&self.lang_type).ok_or("Language not supported")?;
        let functions = get_function_regex(&self.lang_type);
        let mut stat = FileStat::default();
        let mut state = LexState::Code;
        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            stat.lines += 1;

            let line = line.trim();
            if line.is_empty() {
                stat.blanks += 1;
                continue;
            }

            if let Some(line_comment) = def.line_comment {
                if line.starts_with(line_comment) {
                    stat.comments += 1;
                    continue;
                }
            }
            
            if let Some((start, end)) = def.block_comment {
                match state {
                    LexState::Code => {
                        stat.code += 1;
                        if line.starts_with(start) {
                            if line.ends_with(end) {
                                stat.comments += 1;
                                stat.code -= 1;
                            } else {
                                state = LexState::BlockComment;
                                stat.comments += 1;
                                stat.code -= 1;
                            }
                        }
                    },
                    LexState::BlockComment => {
                        stat.comments += 1;
                        if line.contains(end) {
                            state = LexState::Code;
                            if !line.ends_with(end) {
                                stat.comments -= 1;
                                stat.code += 1;
                            }
                        }
                    },
                }
            }

            if let Some(func_regex) = functions {
                if func_regex.is_match(line) {
                    stat.functions += 1;
                }
            }
        }

        Ok(stat)
    }
}

pub struct PythonLexer;

impl PythonLexer {
    pub fn new() -> Self {
        PythonLexer {}
    }
}

impl Lexer for PythonLexer {
    fn lex(&self, reader: &mut dyn BufRead) -> Result<FileStat, String> {
        todo!()
    }
}

pub struct MdLexer;

impl MdLexer {
    pub fn new() -> Self {
        MdLexer {}
    }
}

impl Lexer for MdLexer {
    fn lex(&self, reader: &mut dyn BufRead) -> Result<FileStat, String> {
        let mut stat = FileStat::default();
        stat.lines = reader.lines().count();
        Ok(stat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_lexer() {
        let lexer = DefaultLexer { lang_type: LangType::Rust };
        let input = r"print('Hello, world!')";
        let stat = lexer.lex(&mut input.as_bytes()).unwrap();
        assert_eq!(stat.lines, 1);
        assert_eq!(stat.blanks, 0);
        assert_eq!(stat.code, 1);
    }
}