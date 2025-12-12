use std::collections::HashMap;

use crate::langs::lang_type::LangType;
use crate::stats::{LangStat, FileStat};

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub inner: HashMap<LangType, LangStat>,
}

impl Report {
    pub fn new() -> Self {
        Report {
            inner: HashMap::new(),
        }
    }
}

impl Report {
    pub fn get_by_lang(&self, lang: &LangType) -> Option<&LangStat> {
        self.inner.get(lang)
    }

    pub fn add(&mut self, stat: FileStat) {
        let lang = stat.lang;
        let lang_stat = self.inner.entry(lang.clone()).or_insert_with(|| LangStat::new(lang));

        lang_stat.files += 1;
        lang_stat.lines += stat.lines;
        lang_stat.code += stat.code;
        lang_stat.comments += stat.comments;
        lang_stat.blanks += stat.blanks;
        lang_stat.functions += stat.functions;
        lang_stat.classes += stat.classes;
        
        lang_stat.stats.push(stat);
    }

}

impl<'a> IntoIterator for &'a Report {
    type Item = (&'a LangType, &'a LangStat);
    type IntoIter = std::collections::hash_map::Iter<'a, LangType, LangStat>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
