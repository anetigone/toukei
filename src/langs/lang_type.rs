use std::hash::{Hash, Hasher};

use strum_macros::{Display, EnumString, VariantNames};

#[derive(Debug, EnumString, VariantNames, Display, Clone, Copy)]
pub enum LangType {
    Asciidoc,
    Astro,
    C,
    Clojure,
    Cpp,
    Csharp,
    Css,
    D,
    Dart,
    Elm,
    Erlang,
    Fsharp,
    Go,
    Graphql,
    H,
    Hpp,
    Haskell,
    Html,
    Java,
    Javascript,
    Json,
    Jsonnet,
    Julia,
    Kotlin,
    Lua,
    Markdown,
    Nix,
    Ocaml,
    Php,
    Python,
    Qcl,
    Qsharp,
    R,
    Regex,
    Ruby,
    Rust,
    Sass,
    Scala,
    Sql,
    Swift,
    Tcl,
    Tex,
    Toml,
    Typescript,
    V,
    WenYan,
    Xml,
    Yaml,
    Zig,
    Shell,
    Perl,
    Text,
    Unknown,
}

impl PartialEq for LangType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for LangType {}

impl Hash for LangType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl Default for LangType {
    fn default() -> Self {
        LangType::Unknown
    }
}