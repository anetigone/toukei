use std::io::BufRead;

use regex::RegexSet;

use crate::langs::lang_type::LangType;
use crate::langs::registry::{get_function_regex, get_lang_def};
use crate::stats::FileStat;
use crate::syntax::classifier::{Classifier, DefaultClassifier, PythonClassifier, LineKind};

use super::lex_status::{LineCtx, LexCtx, FnCtx, PyCtx};

pub trait Lexer: Send + Sync {
    
    fn lex(&self, reader: &mut dyn BufRead) -> Result<FileStat, String>;
}

pub struct DefaultLexer<C: Classifier = DefaultClassifier> {
    pub lang_type: LangType,
    classifier: C,
}

impl DefaultLexer {
    pub fn new(lang: LangType) -> Self {
        Self {
            lang_type: lang,
            classifier: DefaultClassifier,
        }
    }
}

impl<C: Classifier> Lexer for DefaultLexer<C> {
    fn lex(&self, reader: &mut dyn BufRead) -> Result<FileStat, String> {
        let def = get_lang_def(&self.lang_type).ok_or("Language not supported")?;
        let function_regexes = get_function_regex(&self.lang_type);

        let mut stat = FileStat::default();
        let mut ctx = LexCtx::default();
        let mut fn_ctx = FnCtx::default();

        for line in reader.lines() {
            let raw = line.map_err(|e| e.to_string())?;
            let trimmed = raw.trim();

            if fn_ctx.in_function && fn_ctx.prev == 0 {
                fn_ctx.in_function = false;
            }

            let lctx = LineCtx::new(&raw, &mut ctx, &def);
            let (kind, pos) = self.classifier.classify(lctx);

            stat.lines += 1;
            match kind {
                LineKind::Blank => stat.blanks += 1,
                LineKind::Comment | LineKind::DocComment => stat.comments += 1,
                LineKind::Code => {
                    stat.code += 1;
                    if let Some(regexes) = &function_regexes {
                        self.update_fn_ctx(trimmed, regexes, &mut fn_ctx);
                    }
                }
                LineKind::Mixed => {
                    stat.code += 1;
                    if let Some((start, end)) = pos {
                        let raw = &trimmed[start..end];
                        if let Some(regexes) = &function_regexes {
                            self.update_fn_ctx(raw, regexes, &mut fn_ctx);
                        }
                    }
                }
            }
            if fn_ctx.in_function {
                stat.functions += 1;
            }
        }

        Ok(stat)
    }
}

impl<C: Classifier> DefaultLexer<C> {
    fn update_fn_ctx(
        &self, 
        raw: &str, 
        regexes: &RegexSet, 
        ctx: &mut FnCtx) {
            if !ctx.in_function {
                if regexes.is_match(raw) {
                    ctx.in_function = true;
                    ctx.depth = 0;
                }
            }
            for ch in raw.chars() {
                if ch == '{' {
                    ctx.depth += 1;
                } else if ch == '}' {
                    ctx.depth = ctx.depth.saturating_sub(1);
                }
            }

            ctx.prev = ctx.depth;
        }
}

pub struct PythonLexer;

impl PythonLexer {
    pub fn new() -> Self { PythonLexer }
}

/// 计算一行真正的缩进空格数（1 tab = 4 space）
fn calc_indent(raw: &str) -> usize {
    raw.chars()
        .take_while(|c| c.is_whitespace())
        .map(|c| if c == '\t' { 4 } else { 1 })
        .sum()
}

impl Lexer for PythonLexer {
    fn lex(&self, reader: &mut dyn BufRead) -> Result<FileStat, String> {
        let def = get_lang_def(&LangType::Python)
                        .ok_or("Python language not supported")?;
        let fn_res = get_function_regex(&LangType::Python);

        let mut stat = FileStat::default();
        let mut ctx  = LexCtx::default();
        let classifier = PythonClassifier::new();
        let mut py = PyCtx::default();

        for line in reader.lines() {
            let raw = line.map_err(|e| e.to_string())?;
            let trimmed = raw.trim();

            /* ---------- 0. 先处理“上一行是函数定义”的遗留标记 ---------- */
            if py.fn_def_line {
                py.fn_def_line = false;
                py.in_fn       = true;
                py.base_indent = py.cur_indent;   // 函数体起始缩进
            }

            /* ---------- 1. 分类本行 ---------- */
            let lctx = LineCtx::new(&raw, &mut ctx, &def);
            let (kind, pos) = classifier.classify(lctx);

            stat.lines += 1;
            match kind {
                LineKind::Blank => stat.blanks += 1,
                LineKind::Comment | LineKind::DocComment => stat.comments += 1,
                LineKind::Code | LineKind::Mixed => {
                    stat.code += 1;

                    // 只在代码段里找函数定义
                    let code_slice = match kind {
                        LineKind::Mixed => {
                            let (s, e) = pos.unwrap();
                            &trimmed[s..e]
                        }
                        _ => trimmed,
                    };

                    if let Some(ref re) = fn_res {
                        if re.is_match(code_slice) {
                            py.fn_def_line  = true; // 延迟到下一行才真正进入函数体
                            stat.functions += 1;
                        }

                    }
                }
            }
        
            /* ---------- 2. 维护缩进 & 函数体范围 ---------- */
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;          // 空行或纯注释不影响缩进逻辑
            }

            let indent = calc_indent(&raw);
            py.cur_indent = indent;

            if py.in_fn {
                // 当前行缩进 ≤ 函数基准缩进  →  退出函数体
                if indent <= py.base_indent {
                    py.in_fn = false;
                }
            }
            if py.in_fn {
                stat.functions += 1;
            }
        }
        Ok(stat)
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
    use std::io::Cursor;

    #[test]
    fn count_simple_c_like() {
        let code = r#"
// 文件注释
#include <stdio.h>

/* 块注释
   第二行
*/
int add(int a, int b) {} // 函数声明

int main() {
    printf("hello\n");  // 行注释
    return 0;
}
"#;
        let mut cursor = Cursor::new(code);
        let stat = DefaultLexer::new(LangType::C)
            .lex(&mut cursor)
            .unwrap();

        // 总行数
        assert_eq!(stat.lines, 13);
        // 空行
        assert_eq!(stat.blanks, 3);
        // 注释行（// 两行 + /* */ 两行）
        assert_eq!(stat.comments, 4);
        // 纯代码行
        assert_eq!(stat.code, 6);
        // 函数数
        assert_eq!(stat.functions, 5);
    }

    #[test]
    fn empty_file() {
        let code = "";
        let mut cursor = Cursor::new(code);
        let stat = DefaultLexer::new(LangType::Rust)
            .lex(&mut cursor)
            .unwrap();
        assert_eq!(stat.lines, 0);
        assert_eq!(stat.blanks, 0);
        assert_eq!(stat.code, 0);
        assert_eq!(stat.functions, 0);
    }

    #[test]
    fn count_simple_python() {
        let code = r#"#!/usr/bin/env python3
# This is a Python script example

import sys
import os

def hello_world():
    """Print hello world message."""
    print("Hello, World!")  # This prints a message

class MyClass:
    """A simple class."""

    def __init__(self, name):
        self.name = name

    def greet(self):
        return f"Hello, {self.name}!"

async def async_function():

    await some_async_call()

    print("Done")

if __name__ == "__main__":
    hello_world()
    obj = MyClass("Alice")
    print(obj.greet())
"#;
        let mut cursor = Cursor::new(code);
        let stat = PythonLexer::new()
            .lex(&mut cursor)
            .unwrap();

        // 总行数
        assert_eq!(stat.lines, 29);
        // 空行
        assert_eq!(stat.blanks, 9);
        // 注释行（including docstrings）
        assert_eq!(stat.comments, 4);
        // 纯代码行
        assert_eq!(stat.code, 16);
        // 函数数 (hello_world, __init__, greet, async_function)
        assert_eq!(stat.functions, 14);
    }
}