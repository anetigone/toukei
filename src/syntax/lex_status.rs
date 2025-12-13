use crate::langs::lang_def::LangDef;

#[derive(Debug, Default, Clone, Copy)]
pub struct LexCtx {
    pub in_block_comment: bool,
    pub in_string: bool,
}

#[derive(Debug)]
pub struct LineCtx<'a> {
    raw: &'a str,
    trimmed: &'a str,
    ctx: &'a mut LexCtx,
    lang: &'a LangDef,
}

impl<'a> LineCtx<'a> {
    pub fn new(raw: &'a str, ctx: &'a mut LexCtx, lang: &'a LangDef) -> Self {
        Self {
            raw,
            trimmed: raw.trim(),
            ctx,
            lang,
        }
    }

    pub fn raw(&self) -> &str {
        self.raw
    }
    pub fn trimmed(&self) -> &str {
        self.trimmed
    }
    pub fn ctx(&mut self) -> &mut LexCtx {
        self.ctx
    }
    pub fn lang(&self) -> &LangDef {
        self.lang
    }
}

#[derive(Debug, Default, Clone)]
pub struct FnCtx {
    pub in_function: bool,
    pub prev: isize,
    pub depth: isize,
}

#[derive(Debug, Default, Clone)]
pub struct PyCtx {
    pub in_fn:       bool,          // 是否正位于函数体内部
    pub fn_def_line: bool,          // 刚识别到函数定义的那一行
    pub base_indent: usize,         // 函数定义所在行的缩进
    pub cur_indent:  usize,         // 当前行缩进
}