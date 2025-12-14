
use crate::parser::args_parser::ArgParser;

use crate::config::Config;
use crate::report::Report;
use crate::fc::FileCounter;
use crate::fc::AsyncFileCounter;

pub struct Cli{
    arg_parser: ArgParser,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            arg_parser: ArgParser::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        use tokio::runtime::Runtime;

        let args = std::env::args().skip(1);
        let matches = self.arg_parser
            .build_matches(args)
            .map_err(|e| e.to_string())?;
        let config = self.arg_parser
            .parse_matches(&matches)
            .map_err(|e| e.to_string())?;

        if config.help {
            self.print_help();
            return Ok(());
        }

        if config.enable_async {
            // Async mode
            let rt = Runtime::new()
                .map_err(|e| format!("Failed to create async runtime: {}", e))?;
            let report = rt.block_on(self.run_async(config))?;
            self.print(&report);
        } else {
            // Sync mode
            let counter = FileCounter::new(config.clone());
            let report = counter.process()?;
            self.print(&report);
        }
        Ok(())
    }

    /// 异步辅助函数
    async fn run_async(&self, config: Config) -> Result<Report, String> {
        let mut async_counter = AsyncFileCounter::new(config.clone());

        // Set custom number of workers if specified
        if config.num_workers > 0 {
            async_counter = async_counter.with_workers(config.num_workers);
        }

        async_counter.process()
            .await
            .map_err(|e| format!("Async processing failed: {}", e))
    }
}

impl Cli {
    pub fn print(&self, report: &Report) {
        self.print_divider();

        // 使用更宽的列宽和对齐方式
        println!(
            "{:<12} {:<8} {:<10} {:<10} {:<10} {:<10} {:<10}",
            "Language", "Files", "Lines", "Code", "Comments", "Blanks", "Functions"
        );
        self.print_divider();

        // 收集所有数据并按行数排序
        let mut items: Vec<_> = report.into_iter().collect();
        items.sort_by(|a, b| b.1.lines.cmp(&a.1.lines)); // 按行数降序排序

        for (lang, stat) in items {
            println!(
                "{:<12} {:<8} {:<10} {:<10} {:<10} {:<10} {:<10}",
                lang.to_string(),
                stat.files,
                stat.lines,
                stat.code,
                stat.comments,
                stat.blanks,
                stat.functions
            );
        }

        self.print_divider();

        // 添加总计行
        let total_files: usize = report.into_iter().map(|(_, s)| s.files).sum();
        let total_lines: usize = report.into_iter().map(|(_, s)| s.lines).sum();
        let total_code: usize = report.into_iter().map(|(_, s)| s.code).sum();
        let total_comments: usize = report.into_iter().map(|(_, s)| s.comments).sum();
        let total_blanks: usize = report.into_iter().map(|(_, s)| s.blanks).sum();
        let total_functions: usize = report.into_iter().map(|(_, s)| s.functions).sum();

        println!(
            "{:<12} {:<8} {:<10} {:<10} {:<10} {:<10} {:<10}",
            "Total", total_files, total_lines, total_code, total_comments, total_blanks, total_functions
        );
        self.print_divider();
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