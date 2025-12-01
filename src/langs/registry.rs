use lazy_static::lazy_static;
use strum::VariantNames;
use crate::langs::lang_type::LangType;
use crate::langs::lang_def::LangDef;

lazy_static! {
    pub static ref SUPPORTED_LANGUAGES: Vec<&'static str> = LangType::VARIANTS.to_vec();

    pub static ref LANG_DEFINITIONS: Vec<&'static LangDef> = vec![
        &RUST_DEF,
        &CPP_DEF,
        &GO_DEF,
        &PY_DEF,
        &JS_DEF,
        &C_DEF,
        &CS_DEF,
        &JAVA_DEF,
        &SWIFT_DEF,
        &KOTLIN_DEF,
        &DART_DEF,
        &SH_DEF,
        &PERL_DEF,
        &RUBY_DEF,
        &LUA_DEF,
        &SQL_DEF,
        &TS_DEF,
        &PHP_DEF,
        &MARKDOWN_DEF,
        &HTML_DEF,
        &CSS_DEF,
        &JSON_DEF,
        &XML_DEF,
        &YAML_DEF,
        &TOML_DEF,
        &TEXT_DEF,
    ];

    pub static ref LANG_DEF_MAP: std::collections::HashMap<&'static str, &'static LangDef> = {
        let mut map = std::collections::HashMap::new();
        for def in LANG_DEFINITIONS.iter() {
            map.insert(def.name, *def);
        }
        map
    };
}

pub static RUST_DEF: LangDef = LangDef {
        name: "Rust",
        extensions: &["rs"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*fn\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(->\s*[^\{]*)?\s*\{",
            r"^\s*pub\s+fn\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(->\s*[^\{]*)?\s*\{",
        ],
        class_pattern: Some(r"^\s*(pub\s+)?struct\s+[a-zA-Z0-9_]+"),
    };

pub static CPP_DEF: LangDef = LangDef {
        name: "Cpp",
        extensions: &["cpp", "cc", "cxx", "hpp", "h"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*[a-zA-Z0-9_\*&<>\[\]]+\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(const\s*)?\s*\{",
            r"^\s*void\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(const\s*)?\s*\{",
        ],
        class_pattern: Some(r"^\s*class\s+[a-zA-Z0-9_]+"),
    };

pub static GO_DEF: LangDef = LangDef {
        name: "Go",
        extensions: &["go"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*func\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
            r"^\s*func\s+\([^)]*\)\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
        ],
        class_pattern: Some(r"^\s*type\s+[a-zA-Z0-9_]+\s+(struct|interface)\s*\{"),
    };

pub static PY_DEF: LangDef = LangDef {
        name: "Python",
        extensions: &["py"],
        line_comment: Some("#"),
        block_comment: Some(("\"\"\"", "\"\"\"")),
        doc_comment: Some("'''"),
        function_patterns: &[
            r"^\s*def\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*:",
            r"^\s*async\s+def\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*:",
        ],
        class_pattern: Some(r"^\s*class\s+[a-zA-Z0-9_]+")
    };  

pub static JS_DEF: LangDef = LangDef {
        name: "JavaScript",
        extensions: &["js"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("/**"),
        function_patterns: &[
            r"^\s*function\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
            r"^\s*const\s+[a-zA-Z0-9_]+\s*=\s*\([^)]*\)\s*=>",
            r"^\s*let\s+[a-zA-Z0-9_]+\s*=\s*\([^)]*\)\s*=>",
            r"^\s*var\s+[a-zA-Z0-9_]+\s*=\s*\([^)]*\)\s*=>",
            r"^\s*[a-zA-Z0-9_]+\s*=\s*\([^)]*\)\s*=>",
        ],
        class_pattern: Some(r"^\s*(export\s+)?class\s+[a-zA-Z0-9_]+"),
    };

pub static C_DEF: LangDef = LangDef {
        name: "C",
        extensions: &["c"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*[a-zA-Z0-9_\*&\[\]]+\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
            r"^\s*void\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
        ],
        class_pattern: None,
    };

pub static CS_DEF: LangDef = LangDef {
        name: "CSharp",
        extensions: &["cs"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*(public\s+|private\s+|protected\s+)?(static\s+)?[a-zA-Z0-9_<>,\[\]]+\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
            r"^\s*(public\s+|private\s+|protected\s+)?(static\s+)?void\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
        ],
        class_pattern: Some(r"^\s*(public\s+|private\s+|protected\s+)?class\s+[a-zA-Z0-9_]+"),
    };

pub static JAVA_DEF: LangDef = LangDef {
        name: "Java",
        extensions: &["java"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("/**"),
        function_patterns: &[
            r"^\s*(public\s+|private\s+|protected\s+)?(static\s+)?[a-zA-Z0-9_<>,\[\]]+\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(\{|\{[^;]*\{)",
            r"^\s*(public\s+|private\s+|protected\s+)?(static\s+)?void\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(\{|\{[^;]*\{)",
        ],
        class_pattern: Some(r"^\s*(public\s+|private\s+|protected\s+)?class\s+[a-zA-Z0-9_]+"),
    };

pub static SWIFT_DEF: LangDef = LangDef {
        name: "Swift",
        extensions: &["swift"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*func\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(->\s*[^{]*)?\s*\{",
        ],
        class_pattern: Some(r"^\s*class\s+[a-zA-Z0-9_]+"),
    };

pub static KOTLIN_DEF: LangDef = LangDef {
        name: "Kotlin",
        extensions: &["kt"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*fun\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(->\s*[^{]*)?\s*\{",
        ],
        class_pattern: Some(r"^\s*class\s+[a-zA-Z0-9_]+"),
    };

pub static DART_DEF: LangDef = LangDef {
        name: "Dart",
        extensions: &["dart"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
        ],
        class_pattern: Some(r"^\s*class\s+[a-zA-Z0-9_]+"),
    };

pub static SH_DEF: LangDef = LangDef {
        name: "Shell",
        extensions: &["sh", "bash"],
        line_comment: Some("#"),
        block_comment: None,
        doc_comment: None,
        function_patterns: &[],
        class_pattern: None,
    };

pub static PERL_DEF: LangDef = LangDef {
        name: "Perl",
        extensions: &["pl"],
        line_comment: Some("#"),
        block_comment: None,
        doc_comment: None,
        function_patterns: &[],
        class_pattern: None,
    };

pub static RUBY_DEF: LangDef = LangDef {
        name: "Ruby",
        extensions: &["rb"],
        line_comment: Some("#"),
        block_comment: Some(("=begin", "=end")),
        doc_comment: None,
        function_patterns: &[
            r"^\s*def\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
        ],
        class_pattern: Some(r"^\s*class\s+[a-zA-Z0-9_]+"),
    };

pub static LUA_DEF: LangDef = LangDef {
        name: "Lua",
        extensions: &["lua"],
        line_comment: Some("--"),
        block_comment: Some(("--[[", "]]")),
        doc_comment: None,
        function_patterns: &[
            r"^\s*function\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
        ],
        class_pattern: Some(r"^\s*class\s+[a-zA-Z0-9_]+"),
    };

pub static SQL_DEF: LangDef = LangDef {
        name: "SQL",
        extensions: &["sql"],
        line_comment: Some("--"),
        block_comment: Some(("/*", "*/")),
        doc_comment: None,
        function_patterns: &[],
        class_pattern: None,
    };

pub static TS_DEF: LangDef = LangDef {
        name: "TypeScript",
        extensions: &["ts"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*(public\s+|private\s+|protected\s+)?(static\s+)?[a-zA-Z0-9_<>,\[\]]+\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(\{|\{[^;]*\{)",
            r"^\s*(public\s+|private\s+|protected\s+)?(static\s+)?void\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*(\{|\{[^;]*\{)",
        ],
        class_pattern: Some(r"^\s*(public\s+|private\s+|protected\s+)?class\s+[a-zA-Z0-9_]+"),
    };

pub static PHP_DEF: LangDef = LangDef {
        name: "PHP",
        extensions: &["php"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[
            r"^\s*function\s+[a-zA-Z0-9_]+\s*\([^)]*\)\s*\{",
        ],
        class_pattern: Some(r"^\s*class\s+[a-zA-Z0-9_]+"),
    };

pub static MARKDOWN_DEF: LangDef = LangDef {
        name: "Markdown",
        extensions: &["md", "markdown"],
        line_comment: None,
        block_comment: None,
        doc_comment: None,
        function_patterns: &[],
        class_pattern: None,
    };

pub static HTML_DEF: LangDef = LangDef {
        name: "HTML",
        extensions: &["html"],
        line_comment: Some("<!--"),
        block_comment: Some(("<!--", "-->")),
        function_patterns: &[],
        class_pattern: None,
        doc_comment: None,
    };

pub static CSS_DEF: LangDef = LangDef {
        name: "CSS",
        extensions: &["css"],
        line_comment: Some("//"),
        block_comment: Some(("/*", "*/")),
        doc_comment: Some("///"),
        function_patterns: &[],
        class_pattern: None,
    };

pub static JSON_DEF:LangDef =     LangDef {
        name: "JSON",
        extensions: &["json"],
        line_comment: None,
        block_comment: None,
        doc_comment: None,
        class_pattern: None,
        function_patterns: &[],
    };

pub static XML_DEF: LangDef = LangDef {
        name: "XML",
        extensions: &["xml"],
        line_comment: Some("<!--"),
        block_comment: Some(("<!--", "-->")),
        doc_comment: None,
        class_pattern: None,
        function_patterns: &[],
    };

pub static YAML_DEF: LangDef = LangDef {
        name: "YAML",
        extensions: &["yaml", "yml"],
        line_comment: Some("#"),
        block_comment: None,
        doc_comment: None,
        class_pattern: None,
        function_patterns: &[],
    };

pub static TOML_DEF: LangDef = LangDef {
        name: "TOML",
        extensions: &["toml"],
        line_comment: Some("#"),
        block_comment: None,
        doc_comment: None,
        class_pattern: None,
        function_patterns: &[],
    };

pub static TEXT_DEF: LangDef = LangDef {
        name: "Text",
        extensions: &["txt"],
        line_comment: None,
        block_comment: None,
        doc_comment: None,
        class_pattern: None,
        function_patterns: &[],
    };