use crate::langs::lang_type::LangType;

pub mod lex_status;
pub mod lexer;

pub struct LexerFactory;

impl LexerFactory {
    pub fn new() -> Self {
        LexerFactory {}
    }

    pub fn get_lexer(lang_type: LangType) -> Option<Box<dyn lexer::Lexer>> {
        match lang_type {
            LangType::Python => Some(Box::new(lexer::PythonLexer::new())),
            LangType::Markdown => Some(Box::new(lexer::MdLexer::new())),
            LangType::Unknown => None,
            _ => Some(Box::new(lexer::DefaultLexer::new(lang_type))),
        }
    }
}

