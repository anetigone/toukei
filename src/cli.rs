
use crate::parser::args_parser::{self, ArgParser};

use crate::config::{self, Config};
use crate::report::Report;
use crate::fc::FileCounter;

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
        let matches = self.arg_parser
            .build_matches(args)
            .map_err(|e| e.to_string())?;
        let config = self.arg_parser
            .parse_matches(&matches)
            .map_err(|e| e.to_string())?;

        let counter = FileCounter::new(config.clone());
        let report = counter.process()?;
        if config.help {
            self.print_help();
        }
        else {
            self.print(&report);
        }
        Ok(())
    }
}

impl Cli {
    pub fn print(&self, report: &Report) {
        self.print_divider();
        println!(
        "{:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
        "Language", "Files", "Lines", "Blanks", "Comments", "Functions"
        );
        self.print_divider();

        for (lang, stat) in report.into_iter() {
            println!(
            "{:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
            lang.to_string(),
            stat.files,
            stat.lines,
            stat.blanks,
            stat.comments,
            stat.functions
            );
        }
    }

    fn print_divider(&self) {
        println!("{}", "-".repeat(80));
    }

    fn print_help(&self) {
        
        let args = self.arg_parser.get_args();
        self.print_divider();
        println!("{:<20} {:<8} {:<20} {}", "Name", "Short", "Long", "Help");
        self.print_divider();
        for (name, arg) in args.iter() {
            let short = arg
                .get_short()
                .map(|c| format!("-{}", c))
                .unwrap_or_else(|| "".to_string());
            let long = arg
                .get_long()
                .as_ref()
                .map(|s| format!("--{}", s))
                .unwrap_or_else(|| "".to_string());

            println!(
                "{:<20} {:<8} {:<20} {}",
                name,
                short,
                long,
                arg.get_help()
            );
        }
        self.print_divider();
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