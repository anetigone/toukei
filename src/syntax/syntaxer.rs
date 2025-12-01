use std::io::BufRead;

use crate::stats::FileStat;

pub trait Syntax: Send + Sync {
    fn syntax<R: BufRead>(&self, reader: R) -> Result<FileStat, String>;
}
pub struct GeneralSyntax;

impl Syntax for GeneralSyntax {
    fn syntax<R: BufRead>(&self, reader: R) -> Result<FileStat, String> {
        let mut lines = 0;
        let mut blanks = 0;
        let mut comments = 0;
        
        let stats = FileStat::default();

        todo!()
    }
}