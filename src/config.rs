use std::fmt::Display;

use crate::langs::registry::SUPPORTED_LANGUAGES;
use crate::utils::format::OutputFormat;

#[derive(Debug, Default, Clone)]
pub struct Config {
    
    pub paths: Vec<String>,
    pub types: Vec<String>,

    pub ignore_blanks: bool,
    pub ignore_comments: bool,
    
    pub exclude_files: Vec<String>,

    pub show_stats: bool,
    pub output: OutputFormat,
    pub help: bool,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config {{ paths: {:?}, types: {:?}, 
            ignore_blanks: {}, ignore_comments: {}, exclude_files: {:?}, 
            show_stats: {}, output: {:?}, help: {} }}",
            self.paths,self.types,
            self.ignore_blanks,self.ignore_comments,self.exclude_files,
            self.show_stats,self.output,self.help
        )
    }   
}

impl PartialEq<Self> for Config {
    fn eq(&self, other: &Self) -> bool {
        self.paths == other.paths &&
        self.types == other.types &&
        self.ignore_blanks == other.ignore_blanks &&
        self.ignore_comments == other.ignore_comments &&
        self.exclude_files == other.exclude_files &&
        self.show_stats == other.show_stats &&
        self.output == other.output &&
        self.help == other.help
    }
}

impl Config {
    pub fn new() -> Self {

        let paths = vec![".".to_string()];
        let types = SUPPORTED_LANGUAGES.iter().map(|s| s.to_string()).collect();

        Config {
            paths,
            types,
            ignore_blanks: false,
            ignore_comments: false,
            exclude_files: Vec::new(),
            show_stats: false,
            output: OutputFormat::Text,
            help: false,
        }
    }
}