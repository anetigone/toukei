use lazy_static::lazy_static;
use strum::VariantNames;
use regex::{Regex, RegexSet};

use std::collections::HashMap;

use super::lang_type::LangType;
use super::lang_def::LangDef;
use super::definitions::*;

lazy_static! {
    pub static ref SUPPORTED_LANGUAGES: Vec<&'static str> = LangType::VARIANTS.to_vec();

    pub static ref LANGUAGE_DEFINITIONS: HashMap<LangType, &'static LangDef> = {
        let mut map = HashMap::new();
        
        map.insert(LangType::Asciidoc, &ASCIIDOC);
        map.insert(LangType::Astro, &ASTRO);
        map.insert(LangType::C, &C);
        map.insert(LangType::Clojure, &CLOJURE);
        map.insert(LangType::Cpp, &CPP);
        map.insert(LangType::Csharp, &CSHARP);
        map.insert(LangType::Css, &CSS);
        map.insert(LangType::D, &D);
        map.insert(LangType::Dart, &DART);
        map.insert(LangType::Elm, &ELM);
        map.insert(LangType::Erlang, &ERLANG);
        map.insert(LangType::Fsharp, &FSHARP);
        map.insert(LangType::Go, &GO);
        map.insert(LangType::Graphql, &GRAPHQL);
        map.insert(LangType::H, &C);
        map.insert(LangType::Hpp, &CPP);
        map.insert(LangType::Haskell, &HASKELL);
        map.insert(LangType::Html, &HTML);
        map.insert(LangType::Java, &JAVA);
        map.insert(LangType::Javascript, &JAVASCRIPT);
        map.insert(LangType::Json, &JSON);
        map.insert(LangType::Jsonnet, &JSONNET);
        map.insert(LangType::Julia, &JULIA);
        map.insert(LangType::Kotlin, &KOTLIN);
        map.insert(LangType::Lua, &LUA);
        map.insert(LangType::Markdown, &MARKDOWN);
        map.insert(LangType::Nix, &NIX);
        map.insert(LangType::Ocaml, &OCAML);
        map.insert(LangType::Php, &PHP);
        map.insert(LangType::Python, &PYTHON);
        map.insert(LangType::Qcl, &QCL);
        map.insert(LangType::Qsharp, &QSHARP);
        map.insert(LangType::R, &R);
        map.insert(LangType::Regex, &REGEX);
        map.insert(LangType::Ruby, &RUBY);
        map.insert(LangType::Rust, &RUST);
        map.insert(LangType::Sass, &SASS);
        map.insert(LangType::Scala, &SCALA);
        map.insert(LangType::Sql, &SQL);
        map.insert(LangType::Swift, &SWIFT);
        map.insert(LangType::Tcl, &TCL);
        map.insert(LangType::Tex, &TEX);
        map.insert(LangType::Toml, &TOML);
        map.insert(LangType::Typescript, &TYPESCRIPT);
        map.insert(LangType::V, &V);
        map.insert(LangType::WenYan, &WENYAN);
        map.insert(LangType::Xml, &XML);
        map.insert(LangType::Yaml, &YAML);
        map.insert(LangType::Zig, &ZIG);
        map.insert(LangType::Shell, &SHELL); 
        map.insert(LangType::Perl, &PERL);
        map.insert(LangType::Text, &TEXT);
        map
    };

    pub static ref FUNCTION_REGEX_MAP: HashMap<LangType, RegexSet> = {
        let mut map = HashMap::new();

        for (k, v) in LANGUAGE_DEFINITIONS.iter() {
            let set = RegexSet::new(v.function_patterns).unwrap();
            map.insert(*k, set);
        }

        map
    };

    pub static ref CLASS_REGEX_MAP: HashMap<LangType, RegexSet> = {
        let mut map = HashMap::new();

        for (k, v) in LANGUAGE_DEFINITIONS.iter() {
            let set = RegexSet::new(v.class_patterns).unwrap();
            map.insert(*k, set);
        }

        map
    };

    pub static ref EXT_LANG_MAP: HashMap<String, LangType> = {
        let mut map = HashMap::new();

        for (k, v) in LANGUAGE_DEFINITIONS.iter() {
            for ext in v.extensions.iter() {
                map.insert(ext.to_string(), *k);
            }
        }

        map
    };
}

pub fn get_lang_def(lang_type: &LangType) -> Option<&'static LangDef> {
    LANGUAGE_DEFINITIONS.get(lang_type).copied()
}

pub fn get_function_regex(lang_type: &LangType) -> Option<&RegexSet> {
    FUNCTION_REGEX_MAP.get(lang_type)
}

pub fn get_class_regex(lang_type: &LangType) -> Option<&RegexSet> {
    CLASS_REGEX_MAP.get(lang_type)
}

pub fn get_type_from_ext(ext: &str) -> Option<LangType> {
    EXT_LANG_MAP.get(ext).copied()
}