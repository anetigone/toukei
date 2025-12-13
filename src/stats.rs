use std::ops::AddAssign;

use crate::langs::lang_type::LangType;

#[derive(Debug, Default, Clone)]
pub struct FileStat {
    pub lang: LangType,
    pub path: String,
    pub name: String,
    
    pub lines: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,

    pub functions: usize,
    pub classes: usize,
}

impl FileStat {
    pub fn new(lang: LangType, path: String, name: String) -> Self {
        Self {
            lang,
            path,
            name,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LangStat {
    pub lang: LangType,
    pub files: usize,
    pub lines: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,

    pub functions: usize,
    pub classes: usize,

    pub stats: Vec<FileStat>,
}

impl LangStat {
    pub fn new(lang: LangType) -> Self {
        LangStat {
            lang,
            ..Default::default()
        }
    }
}

impl AddAssign for LangStat {
    fn add_assign(&mut self, other: Self) {
        self.files += other.files;
        self.lines += other.lines;
        self.code += other.code;
        self.comments += other.comments;
        self.blanks += other.blanks;
        self.functions += other.functions;
        self.classes += other.classes;
        
        self.stats.extend_from_slice(&other.stats);
    }
}