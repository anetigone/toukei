use crate::{langs::lang_def::LangDef, stats};

pub static ASCIIDOC: LangDef = LangDef {
    name: "AsciiDoc",
    extensions: &["adoc", "asciidoc", "asc"],
    line_comment: None,
    block_comment: None,
    doc_comment: None,
    function_patterns: &[],
    class_patterns: &[],
};

pub static ASTRO: LangDef = LangDef {
    name: "Astro",
    extensions: &["astro"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: None,
    function_patterns: &["function", "const", "let", "async function"],
    class_patterns: &[],
};

pub static C: LangDef = LangDef {
    name: "C",
    extensions: &["c", "h"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &[
        r"\w+\s+\w+\s*\([^)]*\)\s*\{", 
        r"\w+\s+\*\w+\s*\([^)]*\)\s*\{"],
    class_patterns: &[r"typedef\s+struct\s+\w+"],
};

pub static CLOJURE: LangDef = LangDef {
    name: "Clojure",
    extensions: &["clj", "cljs", "cljc", "edn"],
    line_comment: Some(";;"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &["\\(defn\\s+", "\\(def\\s+", "\\(defmacro\\s+"],
    class_patterns: &["\\(defrecord\\s+"],
};

pub static CPP: LangDef = LangDef {
    name: "C++",
    extensions: &["cpp", "cxx", "cc", "c++", "hpp", "hxx", "hh", "h++"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &[
        r"\w+\s+\w+\s*\([^)]*\)\s*\{",
        r"\w+\s+\*\w+\s*\([^)]*\)\s*\{", 
        r"\w+\s+&\w+\s*\([^)]*\)\s*\{"],
    class_patterns: &[r"class\s+\w+"],
};

pub static CSHARP: LangDef = LangDef {
    name: "C#",
    extensions: &["cs"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("///"),
    function_patterns: &[
        r"\w+\s+\w+\s*\([^)]*\)\s*\{", 
        r"public\s+\w+\s+\w+\s*\([^)]*\)\s*\{"],
    class_patterns: &[r"class\s+\w+"],
};

pub static CSS: LangDef = LangDef {
    name: "CSS",
    extensions: &["css"],
    line_comment: None,
    block_comment: Some(("/*", "*/")),
    doc_comment: None,
    function_patterns: &["@\\w+\\s+", "\\w+\\s*\\{"],
    class_patterns: &["\\.\\w+"],
};

pub static D: LangDef = LangDef {
    name: "D",
    extensions: &["d", "di"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &[
        r"\w+\s+\w+\s*\([^)]*\)\s*\{", 
        r"\w+\s+\*\w+\s*\([^)]*\)\s*\{"],
    class_patterns: &["class\\s+\\w+"],
};

pub static DART: LangDef = LangDef {
    name: "Dart",
    extensions: &["dart"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("///"),
    function_patterns: &[
        r"\w+\s+\w+\s*\([^)]*\)\s*\{", 
        r"\w+\s+\w+\s*\([^)]*\)\s*async"],
    class_patterns: &["class\\s+\\w+"],
};

pub static ELM: LangDef = LangDef {
    name: "Elm",
    extensions: &["elm"],
    line_comment: Some("--"),
    block_comment: Some(("{-", "-}")),
    doc_comment: Some("{-|"),
    function_patterns: &["\\w+\\s*:\\s+", "\\w+\\s+\\w+\\s*="],
    class_patterns: &["type\\s+\\w+"],
};

pub static ERLANG: LangDef = LangDef {
    name: "Erlang",
    extensions: &["erl", "hrl"],
    line_comment: Some("%"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &["\\w+\\s*\\([^)]*\\)\\s*->"],
    class_patterns: &["-module\\s+\\w+"],
};

pub static FSHARP: LangDef = LangDef {
    name: "F#",
    extensions: &["fs", "fsi", "fsx", "fsscript"],
    line_comment: Some("//"),
    block_comment: Some(("(*", "*)")),
    doc_comment: Some("///"),
    function_patterns: &["let\\s+\\w+", "member\\s+\\w+\\."],
    class_patterns: &["type\\s+\\w+"],
};

pub static GO: LangDef = LangDef {
    name: "Go",
    extensions: &["go"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: None,
    function_patterns: &["func\\s+\\w+\\s*\\([^)]*\\)"],
    class_patterns: &["type\\s+\\w+\\s+struct"],
};

pub static GRAPHQL: LangDef = LangDef {
    name: "GraphQL",
    extensions: &["graphql", "gql"],
    line_comment: Some("#"),
    block_comment: None,
    doc_comment: Some("\"\"\""),
    function_patterns: &["type\\s+\\w+", "interface\\s+\\w+", "query\\s+\\w+"],
    class_patterns: &["type\\s+\\w+"],
};

pub static HASKELL: LangDef = LangDef {
    name: "Haskell",
    extensions: &["hs", "lhs"],
    line_comment: Some("--"),
    block_comment: Some(("{-", "-}")),
    doc_comment: Some("{-|"),
    function_patterns: &["\\w+\\s*::", "\\w+\\s+\\w+\\s*="],
    class_patterns: &["data\\s+\\w+", "class\\s+\\w+"],
};

pub static HTML: LangDef = LangDef {
    name: "HTML",
    extensions: &["html", "htm", "xhtml"],
    line_comment: None,
    block_comment: Some(("<!--", "-->")),
    doc_comment: None,
    function_patterns: &["<script", "<function"],
    class_patterns: &["class\\s*=\\s*\""],
};

pub static JAVA: LangDef = LangDef {
    name: "Java",
    extensions: &["java", "class", "jar"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &["\\w+\\s+\\w+\\s*\\([^)]*\\)\\s*\\{", "public\\s+\\w+\\s+\\w+\\s*\\([^)]*\\)\\s*\\{"],
    class_patterns: &["class\\s+\\w+", "interface\\s+\\w+"],
};

pub static JAVASCRIPT: LangDef = LangDef {
    name: "JavaScript",
    extensions: &["js", "jsx", "mjs", "cjs"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &["function\\s+\\w+", "const\\s+\\w+\\s*=\\s*\\(", "\\w+\\s*:\\s*function"],
    class_patterns: &["class\\s+\\w+"],
};

pub static JSON: LangDef = LangDef {
    name: "JSON",
    extensions: &["json", "jsonc"],
    line_comment: None,
    block_comment: None,
    doc_comment: None,
    function_patterns: &[],
    class_patterns: &[],
};

pub static JSONNET: LangDef = LangDef {
    name: "Jsonnet",
    extensions: &["jsonnet", "libsonnet"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &["function\\s+\\w+", "local\\s+\\w+"],
    class_patterns: &[],
};

pub static JULIA: LangDef = LangDef {
    name: "Julia",
    extensions: &["jl"],
    line_comment: Some("#"),
    block_comment: Some(("#=", "=#")),
    doc_comment: None,
    function_patterns: &["function\\s+\\w+", "\\w+\\s*\\([^)]*\\)\\s*="],
    class_patterns: &["struct\\s+\\w+", "type\\s+\\w+"],
};

pub static KOTLIN: LangDef = LangDef {
    name: "Kotlin",
    extensions: &["kt", "kts", "ktm"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &["fun\\s+\\w+", "val\\s+\\w+", "var\\s+\\w+"],
    class_patterns: &["class\\s+\\w+", "interface\\s+\\w+", "object\\s+\\w+"],
};

pub static LUA: LangDef = LangDef {
    name: "Lua",
    extensions: &["lua", "wlua"],
    line_comment: Some("--"),
    block_comment: Some(("--[[", "]]")),
    doc_comment: None,
    function_patterns: &["function\\s+\\w+", "local\\s+function\\s+\\w+"],
    class_patterns: &[],
};

pub static MARKDOWN: LangDef = LangDef {
    name: "Markdown",
    extensions: &["md", "markdown", "mdown", "mkdn"],
    line_comment: None,
    block_comment: Some(("<!--", "-->")),
    doc_comment: None,
    function_patterns: &[],
    class_patterns: &[],
};

pub static NIX: LangDef = LangDef {
    name: "Nix",
    extensions: &["nix"],
    line_comment: Some("#"),
    block_comment: Some(("/*", "*/")),
    doc_comment: None,
    function_patterns: &["\\w+\\s*=", "\\w+\\s*:"],
    class_patterns: &[],
};

pub static OCAML: LangDef = LangDef {
    name: "OCaml",
    extensions: &["ml", "mli", "cmi", "cmo", "cmx"],
    line_comment: None,
    block_comment: Some(("(*", "*)")),
    doc_comment: Some("(**"),
    function_patterns: &["let\\s+\\w+", "let rec\\s+\\w+"],
    class_patterns: &["type\\s+\\w+", "module\\s+\\w+", "class\\s+\\w+"],
};

pub static PERL: LangDef = LangDef { 
    name: "Perl",
    extensions: &["pl", "pm"],
    line_comment: Some("#"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &["sub\\s+\\w+"],
    class_patterns: &["class\\s+\\w+"],
};

pub static PHP: LangDef = LangDef {
    name: "PHP",
    extensions: &["php", "phtml", "php3", "php4", "php5", "phps", "phpt"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &["function\\s+\\w+", "\\w+\\s+\\w+\\s*\\([^)]*\\)\\s*\\{"],
    class_patterns: &["class\\s+\\w+", "interface\\s+\\w+"],
};

pub static PYTHON: LangDef = LangDef {
    name: "Python",
    extensions: &["py", "pyi", "pyc", "pyd", "pyw", "pyz", "pyzw"],
    line_comment: Some("#"),
    block_comment: Some(("\"\"\"", "\"\"\"")),
    doc_comment: Some("\"\"\""),
    function_patterns: &["def\\s+\\w+", "class\\s+\\w+", "async\\s+def\\s+\\w+"],
    class_patterns: &["class\\s+\\w+"],
};

pub static QCL: LangDef = LangDef {
    name: "QCL",
    extensions: &["qcl"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &["\\w+\\s+\\w+\\s*\\([^)]*\\)\\s*\\{", "procedure\\s+\\w+"],
    class_patterns: &[],
};

pub static QSHARP: LangDef = LangDef {
    name: "Q#",
    extensions: &["qs"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &["operation\\s+\\w+", "function\\s+\\w+"],
    class_patterns: &[],
};

pub static R: LangDef = LangDef {
    name: "R",
    extensions: &["r", "R", "s", "Rhistory", "Rprofile", "Renviron"],
    line_comment: Some("#"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &["\\w+\\s*<-\\s*function", "\\w+\\s*\\([^)]*\\)"],
    class_patterns: &[],
};

pub static REGEX: LangDef = LangDef {
    name: "Regex",
    extensions: &["regex"],
    line_comment: None,
    block_comment: None,
    doc_comment: None,
    function_patterns: &[],
    class_patterns: &[],
};

pub static RUBY: LangDef = LangDef {
    name: "Ruby",
    extensions: &["rb", "rbw", "gemspec", "rake", "ru", "erb"],
    line_comment: Some("#"),
    block_comment: Some(("=begin", "=end")),
    doc_comment: None,
    function_patterns: &["def\\s+\\w+", "def\\s+self\\.\\w+", "class\\s+\\w+", "module\\s+\\w+"],
    class_patterns: &["class\\s+\\w+", "module\\s+\\w+"],
};

pub static RUST: LangDef = LangDef {
    name: "Rust",
    extensions: &["rs"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("///"),
    function_patterns: &["fn\\s+\\w+", "pub\\s+fn\\s+\\w+", "async\\s+fn\\s+\\w+"],
    class_patterns: &["struct\\s+\\w+", "enum\\s+\\w+", "impl\\s+\\w+"],
};

pub static SASS: LangDef = LangDef {
    name: "Sass",
    extensions: &["sass", "scss"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: None,
    function_patterns: &["@\\w+\\s+", "\\w+\\s*\\{"],
    class_patterns: &["\\.\\w+", "%\\w+"],
};

pub static SCALA: LangDef = LangDef {
    name: "Scala",
    extensions: &["scala", "sc", "sbt"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &["def\\s+\\w+", "val\\s+\\w+", "var\\s+\\w+"],
    class_patterns: &["class\\s+\\w+", "object\\s+\\w+", "trait\\s+\\w+"],
};

pub static SHELL: LangDef = LangDef {
    name: "Shell",
    extensions: &["sh", "bash", "zsh", "ksh", "csh"],
    line_comment: Some("#"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &["function\\s+\\w+", "\\w+\\s*\\(\\s*\\)"],
    class_patterns: &[],
};

pub static SQL: LangDef = LangDef {
    name: "SQL",
    extensions: &["sql", "ddl", "dml"],
    line_comment: Some("--"),
    block_comment: Some(("/*", "*/")),
    doc_comment: None,
    function_patterns: &["CREATE\\s+\\w+", "ALTER\\s+\\w+", "DROP\\s+\\w+", "SELECT\\s+"],
    class_patterns: &["CREATE\\s+TABLE\\s+\\w+"],
};

pub static SWIFT: LangDef = LangDef {
    name: "Swift",
    extensions: &["swift", "swiftinterface", "swiftmodule"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("///"),
    function_patterns: &["func\\s+\\w+", "init\\s*\\(", "deinit"],
    class_patterns: &["class\\s+\\w+", "struct\\s+\\w+", "enum\\s+\\w+"],
};

pub static TCL: LangDef = LangDef {
    name: "Tcl",
    extensions: &["tcl", "tk"],
    line_comment: Some("#"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &["proc\\s+\\w+"],
    class_patterns: &[],
};

pub static TEX: LangDef = LangDef {
    name: "TeX",
    extensions: &["tex", "latex", "sty", "cls", "bib"],
    line_comment: Some("%"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &["\\\\\\w+\\s*\\{"],
    class_patterns: &[],
};

pub static TEXT: LangDef = LangDef {
    name: "Text",
    extensions: &["txt"],
    line_comment: None,
    block_comment: None,
    doc_comment: None,
    function_patterns: &[],
    class_patterns: &[],
};

pub static TOML: LangDef = LangDef {
    name: "TOML",
    extensions: &["toml"],
    line_comment: Some("#"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &[],
    class_patterns: &[],
};

pub static TYPESCRIPT: LangDef = LangDef {
    name: "TypeScript",
    extensions: &["ts", "tsx", "cts", "mts"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: Some("/**"),
    function_patterns: &["function\\s+\\w+", "const\\s+\\w+\\s*=\\s*\\(", "\\w+\\s*:\\s*function"],
    class_patterns: &["class\\s+\\w+", "interface\\s+\\w+", "type\\s+\\w+"],
};

pub static V: LangDef = LangDef {
    name: "V",
    extensions: &["v", "vv", "vsh"],
    line_comment: Some("//"),
    block_comment: Some(("/*", "*/")),
    doc_comment: None,
    function_patterns: &["fn\\s+\\w+", "pub\\s+fn\\s+\\w+"],
    class_patterns: &["struct\\s+\\w+", "enum\\s+\\w+", "const\\s+\\w+", "var\\s+\\w+"],
};

pub static WENYAN: LangDef = LangDef {
    name: "文言",
    extensions: &["wy"],
    line_comment: Some("註"),
    block_comment: Some(("〔", "〕")),
    doc_comment: None,
    function_patterns: &["有"],
    class_patterns: &[],
};

pub static XML: LangDef = LangDef {
    name: "XML",
    extensions: &["xml", "xsl", "xslt", "svg", "wsdl", "wsdd", "xhtml"],
    line_comment: None,
    block_comment: Some(("<!--", "-->")),
    doc_comment: None,
    function_patterns: &["<\\w+", "</\\w+"],
    class_patterns: &["<\\w+\\s+class\\s*=\\s*\""],
};

pub static YAML: LangDef = LangDef {
    name: "YAML",
    extensions: &["yaml", "yml"],
    line_comment: Some("#"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &[],
    class_patterns: &[],
};

pub static ZIG: LangDef = LangDef {
    name: "Zig",
    extensions: &["zig"],
    line_comment: Some("//"),
    block_comment: None,
    doc_comment: None,
    function_patterns: &["fn\\s+\\w+", "pub\\s+fn\\s+\\w+"],
    class_patterns: &["const\\s+\\w+", "var\\s+\\w+"],
};