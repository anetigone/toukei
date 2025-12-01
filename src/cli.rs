
use crate::parser::args_parser::ArgParser;

use crate::config::Config;


pub struct Cli{
    arg_parser: ArgParser,

    config: Config,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            arg_parser: ArgParser::default(),
            config: Config::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let args = std::env::args().skip(1);

        Ok(())
    }


}

#[cfg(test)]
mod tests { 
    use crate::utils::format::OutputFormat;
    use crate::parser::value_parser::{ParseValue};
    use crate::value_parser;

    #[test]
    fn test_cli() {
        let parser = value_parser!(Vec<String>, |s| {
            Ok(s.split(',').map(|s| s.trim().to_string()).collect())
        });

        assert_eq!(parser.parse("a,b,c").unwrap(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn test_parser() {
        let parser = value_parser!(OutputFormat);

        assert_eq!(parser.parse("text").unwrap(), OutputFormat::Text);
    }
}