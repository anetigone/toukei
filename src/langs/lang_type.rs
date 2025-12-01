use std::hash::{Hash, Hasher};

use strum_macros::{Display, EnumString, VariantNames};

#[derive(Debug, EnumString, VariantNames, Display)]
pub enum LangType {
    C,
    Cpp,
    H,
    Hpp,
    Rust,
    Go,
    Java,
    CSharp,
    Python,
    Php,
    Swift,
    Kotlin,
    Dart,
    Shell,
    Perl,
    Ruby,
    Lua,
    Sql,
    JavaScript,
    TypeScript,
    Html,
    Css,
    Json,
    Xml,
    Yaml,
    Toml,
    Markdown,
    Text,
    Unknown,
}

impl Hash for LangType { 
    fn hash<H: Hasher>(&self, state: &mut H) { 
        self.to_string().hash(state); 
    }
}