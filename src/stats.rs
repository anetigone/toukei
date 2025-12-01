use std::ops::AddAssign;

#[derive(Debug, Default, Clone)]
pub struct FileStat {
    pub path: String,
    pub name: String,
    pub size: u64,
    
    pub lines: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,

    pub functions: usize,
    pub classes: usize,
}

impl FileStat {
    pub fn new(path: String, name: String) -> Self {
        Self {
            path,
            name,
            ..Default::default()
        }
    }
}

impl AddAssign for FileStat {
    fn add_assign(&mut self, other: Self) {
        self.lines += other.lines;
        self.code += other.code;
        self.comments += other.comments;
        self.blanks += other.blanks;
        self.functions += other.functions;
        self.classes += other.classes;
        self.size += other.size;
    }
}

#[derive(Debug, Default, Clone)]
pub struct LangStat {
    pub lang: String,
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
    pub fn new(lang: String) -> Self {
        LangStat {
            lang,
            ..Default::default()
        }
    }

    pub fn add(&mut self, other: &LangStat) {
        self.files += other.files;
        self.lines += other.lines;
        self.code += other.code;
        self.comments += other.comments;
        self.blanks += other.blanks;
        self.functions += other.functions;

        self.stats.extend_from_slice(&other.stats);
    }
}