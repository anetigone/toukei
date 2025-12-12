use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap};

use super::arg::{Arg, ArgAction};
use super::parse_error::ParseError;

use crate::config::Config;
use crate::parser::matches::{Matches};
use crate::parser::arg_cursor::ArgCursor;
use crate::{extract_config, value_parser};
use crate::utils::format::OutputFormat;

/// 命令行参数解析器，用于定义和解析命令行参数
///
/// # 结构
///
/// `ArgParser` 包含以下核心组件：
/// - `args`: 存储所有定义的参数，使用有序的 BTreeMap 保持参数的定义顺序
/// - `long_arg`: 长参数名（如 `--help`）到内部参数名的映射
/// - `short_arg`: 短参数名（如 `-h`）到内部参数名的映射
/// - `params`: 存储解析后的参数值
///
/// # 功能
///
/// - 支持长参数（`--long`）和短参数（`-s`）格式
/// - 支持多种参数动作：设置值、追加值、计数、设置布尔值
/// - 类型安全的参数解析和获取
/// - 自动生成帮助信息
/// - 支持参数验证
///
/// # 示例
///
/// ```rust
/// let mut parser = ArgParser::new()
///     .arg(Arg::new("verbose")
///         .short('v')
///         .long("verbose")
///         .help("启用详细输出")
///         .action(ArgAction::SetTrue));
///
/// let matches = parser.build_matches(vec!["-v"])?;
/// let verbose = matches.get_one::<bool>("verbose")?;
/// ```
#[derive(Debug)]
pub struct ArgParser {
    args: BTreeMap<String, Arg>,
    long_arg: HashMap<String, String>,
    short_arg: HashMap<char, String>,

    params: Matches,
}

impl ArgParser {
    pub fn new() -> Self {
        ArgParser {
            args: BTreeMap::new(),
            long_arg: HashMap::new(),
            short_arg: HashMap::new(),
            params: Matches::new(),
        }
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.add_arg(arg);
        self
    }

    pub fn add_arg(&mut self, arg: Arg) {
        if let Some(long_name) = arg.get_long() {
            self.long_arg.insert(long_name.to_string(), arg.get_name().to_string());
        }
        if let Some(short_name) = arg.get_short() {
            self.short_arg.insert(short_name, arg.get_name().to_string());
        }
        self.args.insert(arg.get_name().to_string(), arg);
    }

    pub fn get_arg(&self, name: &str) -> Option<&Arg> {
        self.args.get(name)
    }

    pub fn set_param(&mut self, name: &str, value: String, value_type: TypeId) -> Result<(), ParseError> {
        let arg = self.get_arg(name)
        .ok_or(ParseError::UnknownArg(name.to_string()))?;

        let expected = arg.get_value_type();
        if expected != value_type {
            return Err(ParseError::ArgMismatch { name: name.to_string(), expected, actual: value_type });
        }

        let value = arg.parse(&value)?;

        self.params.set(name, value, value_type)
    }

    pub fn append_param(&mut self, name: &str, value: String, value_type: TypeId) -> Result<(), ParseError> {
        let arg = self.get_arg(name)
            .ok_or(ParseError::UnknownArg(name.to_string()))?;

        let expected = arg.get_value_type();
        if expected != value_type {
            return Err(ParseError::ArgMismatch { name: name.to_string(), expected, actual: value_type });
        }

        let value = arg.parse(&value)?;

        self.params.append(name, value, value_type)
    }

    pub fn get_args(&self) -> &BTreeMap<String, Arg> {
        &self.args
    }

    pub fn get_one<T>(&self, name: &str) -> Result<&T, ParseError> 
    where
        T: Any + Send + Sync + 'static,
    {
        self.params.get_one(name)
    }

    pub fn get_many<T>(&self, name: &str) -> Result<Vec<&T>, ParseError> 
    where
        T: Any + Send + Sync + 'static,
    {
        self.params.get_many(name)
    }
    
}

impl ArgParser {
    pub fn build_matches<I, T>(&mut self, iter: I) -> Result<Matches, ParseError> 
    where 
        I: IntoIterator<Item = T>, 
        T: Into<String>
    { 
        let iter = iter.into_iter().map(|s| s.into());
        let mut cursor= ArgCursor::new(iter);

        let mut matches = Matches::new();

        let mut cur_arg: Option<String> = None;

        while let Some(arg_str) = cursor.next() { 
            if let Some(long_flag) = arg_str.strip_prefix("--") {
                self.handle_long_flag(&long_flag, &mut cursor, &mut matches)?;
                cur_arg = Some(long_flag.to_string());
            }
            else if let Some(short_flag) = arg_str.strip_prefix("-") {
                for c in short_flag.chars() {
                    self.handle_short_flag(c, &mut cursor, &mut matches)?;
                }
                cur_arg = None;
            }
            else {
                if let Some(cur) = cur_arg.as_ref() {
                    self.handle_value(&arg_str, cur, &mut cursor, &mut matches)?;
                }
            }
        }

        Ok(matches)
    }

    pub fn parse_matches(&self, matches: &Matches) -> Result<Config, ParseError> {

        let mut config = Config::new();
    
        extract_config!(matches, config, {
            vecs: [
                paths <- "path",
                types <- "type",
                exclude_files <- "exclude-files"
            ],
            scalars: [
                ignore_blanks <- "ignore-blanks" : bool,
                ignore_comments <- "ignore-comments" : bool,
                output <- "output" : OutputFormat,
                help <- "help" : bool
            ]
        });

        Ok(config)
    }

    fn handle_long_flag<I>(
        &self, 
        key: &str, 
        cursor: &mut ArgCursor<I>, 
        matches: &mut Matches
    ) -> Result<(), ParseError>
    where 
        I: Iterator<Item = String> 
    {
        let arg = self.get_arg_by_long(key)
            .ok_or(ParseError::UnknownFlag(key.to_string()))?;

        self.act_parse(key, arg, cursor, matches)
    }

    fn handle_short_flag<I>(
        &self, 
        key: char,
        cursor: &mut ArgCursor<I>, 
        matches: &mut Matches
    ) -> Result<(), ParseError>
    where 
        I: Iterator<Item = String> 
    {
        let arg = self.get_arg_by_short(key)
            .ok_or(ParseError::UnknownFlag(key.to_string()))?;

        self.act_parse(&key.to_string(), arg, cursor, matches)
    }

    fn handle_value<I>(
        &self, 
        key: &str, 
        cur: &str,
        cursor: &mut ArgCursor<I>, 
        matches: &mut Matches
    ) -> Result<(), ParseError>
    where 
        I: Iterator<Item = String> 
    {
        if let Some(arg) = self.args.get(cur) {
            self.act_parse(key, arg, cursor, matches)
        }
        else {
            Err(ParseError::UnknownArg(cur.to_string()))
        }
    }

    fn act_parse<I>(
        &self, 
        key: &str, 
        arg: &Arg,
        cursor: &mut ArgCursor<I>, 
        matches: &mut Matches
    ) -> Result<(), ParseError>
    where 
        I: Iterator<Item = String> 
    {
        match arg.get_action() {
            ArgAction::Set => {
                let value = cursor.next_if_value()
                    .ok_or(ParseError::NoValue(key.to_string()))?;

                let value = arg.parse(&value)?;
                matches.set(arg.get_name(), value, arg.get_value_type())
            },
            ArgAction::Append => {
                let value = cursor.next_if_value()
                    .ok_or(ParseError::NoValue(key.to_string()))?;

                let value = arg.parse(&value)?;
                matches.append(arg.get_name(), value, arg.get_value_type())
            },
            ArgAction::Count => {
                let cnt = matches.try_get_one::<u8>(arg.get_name())
                    .copied()
                    .unwrap_or(0 as u8);
                let new = cnt.saturating_add(1);

                let value = arg.parse(&new.to_string())?;
                matches.set(arg.get_name(), value, arg.get_value_type())
            },
            ArgAction::SetTrue => {
                let value = arg.parse("true")?;
                matches.set(arg.get_name(), value, arg.get_value_type())
            }
        }
    }

    fn get_arg_by_long(&self, long: &str) -> Option<&Arg> { 
        self.long_arg.get(long).and_then(|name| self.args.get(name))
    }

    fn get_arg_by_short(&self, short: char) -> Option<&Arg> { 
        self.short_arg.get(&short).and_then(|name| self.args.get(name))
    }
}

impl Default for ArgParser {
    fn default() -> Self {
        ArgParser::new()
                .arg(Arg::new("help")
                    .short('h')
                    .long("help")
                    .help("显示帮助信息")
                    .parser(value_parser!(bool))
                    .action(ArgAction::SetTrue))
                .arg(Arg::new("path")
                    .short('p')
                    .long("path")
                    .help("指定要分析的路径")
                    .parser(value_parser!(Vec<String>, |s| {
                        Ok(s.split(',').map(|s| s.trim().to_string()).collect())
                    })))
                .arg(Arg::new("type")
                    .short('t')
                    .long("type")
                    .help("指定要分析的语言类型")
                    .parser(value_parser!(Vec<String>, |s| {
                        Ok(s.split(',').map(|s| s.trim().to_string()).collect())
                    })))
                .arg(Arg::new("exclude-files")
                    .short('e')
                    .long("exclude-files")
                    .help("指定要忽略的文件或目录，多个以逗号分隔")
                    .parser(value_parser!(Vec<String>, |s| {
                        Ok(s.split(',').map(|s| s.trim().to_string()).collect())
                    })))
                .arg(Arg::new("ignore-blanks")
                    .long("ignore-blanks")
                    .help("忽略空白行")
                    .parser(value_parser!(bool))
                    .action(ArgAction::SetTrue))
                .arg(Arg::new("ignore-comments")
                    .long("ignore-comments")
                    .help("忽略注释行")
                    .parser(value_parser!(bool))
                    .action(ArgAction::SetTrue))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .help("指定输出文件")
                    .parser(value_parser!(OutputFormat)))
    }
}

#[macro_export]
macro_rules! extract_config {
    ($matches:ident, $config:ident, {
        // 处理 Vec 类型 (flatten 逻辑)
        vecs: [ $( $v_field:ident <- $v_key:literal ),* ],
        // 处理 单值 类型 (Copy 逻辑)
        scalars: [ $( $s_field:ident <- $s_key:literal : $s_type:ty ),* ]
    }) => {
        $(
            if let Ok(vals) = $matches.get_many::<Vec<String>>($v_key) {
                $config.$v_field = vals.iter().copied().flatten().cloned().collect();
            }
        )*
        $(
            if let Ok(val) = $matches.get_one::<$s_type>($s_key) {
                $config.$s_field = *val;
            }
        )*
    };
}

#[cfg(test)]
mod tests { 
    use super::*;
    use crate::value_parser;

        #[test]
    fn test_long_flag_with_value() {
        let mut parser = ArgParser::new();
        parser.add_arg(Arg::new("path")
            .long("path")
            .help("设置路径"));

        let args = vec!["--path", "/home/user"];
        let result = parser.build_matches(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        assert_eq!(matches.get_one::<String>("path").unwrap(), "/home/user");
    }

    #[test]
    fn test_short_flag_with_value() {
        let mut parser = ArgParser::new();
        parser.add_arg(Arg::new("path")
            .short('p')
            .help("设置路径"));

        let args = vec!["-p", "/home/user"];
        let result = parser.build_matches(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        assert_eq!(matches.get_one::<String>("path").unwrap(), "/home/user");
    }

    #[test]
    fn test_multiple_short_flags() {
        let mut parser = ArgParser::new();
        parser.add_arg(Arg::new("verbose")
            .parser(value_parser!(bool))
            .short('v')
            .action(ArgAction::SetTrue)
            .help("详细输出"));
        parser.add_arg(Arg::new("debug")
            .parser(value_parser!(bool))
            .action(ArgAction::SetTrue)
            .short('d')
            .help("调试模式"));

        let args = vec!["-vd"];
        let result = parser.build_matches(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        assert!(matches.get_one::<bool>("verbose").is_ok());
        assert!(matches.get_one::<bool>("debug").is_ok());
    }

    #[test]
    fn test_unknown_flag() {
        let mut parser = ArgParser::new();
        let args = vec!["--unknown"];
        let result = parser.build_matches(args);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::UnknownFlag(_)));
    }

    #[test]
    fn test_flag_without_value() {
        let mut parser = ArgParser::new();
        parser.add_arg(Arg::new("path")
            .long("path")
            .help("设置路径"));

        let args = vec!["--path"];
        let result = parser.build_matches(args);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::NoValue(_)));
    }

    #[test]
    fn test_append_action() {
        let mut parser = ArgParser::new();
        parser.add_arg(Arg::new("files")
            .long("files")
            .action(ArgAction::Append)
            .help("多个文件"));

        let args = vec!["--files", "file1", "--files", "file2"];
        let result = parser.build_matches(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        let files: Vec<&String> = matches.get_many("files").unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0], "file1");
        assert_eq!(files[1], "file2");
    }

    #[test]
    fn test_count_action() {
        let mut parser = ArgParser::new();
        parser.add_arg(Arg::new("verbose")
            .short('v')
            .parser(value_parser!(u8))
            .action(ArgAction::Count)
            .help("详细级别"));

        let args = vec!["-vvv"];
        let result = parser.build_matches(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        assert_eq!(*matches.get_one::<u8>("verbose").unwrap(), 3);
    }

    #[test]
    fn test_set_true_action() {
        let mut parser = ArgParser::new();
        parser.add_arg(Arg::new("debug")
            .long("debug")
            .action(ArgAction::SetTrue)
            .parser(value_parser!(bool))
            .help("调试模式"));
 
        let args = vec!["--debug"];
        let result = parser.build_matches(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        assert_eq!(*matches.get_one::<bool>("debug").unwrap(), true);
    }

    #[test]
    fn test_matches_and_config() {
        let mut arg_parser =  ArgParser::default();
        
        let args = vec!["--path", "/home/user", "--type", "cpp,rust", "--exclude-files", "file1,file2", "--ignore-blanks", "--ignore-comments", "--output", "json"];
        let result = arg_parser.build_matches(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        let config = arg_parser.parse_matches(&matches);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config, Config {
            paths: vec!["/home/user".to_string()],
            types: vec!["cpp".to_string(), "rust".to_string()],
            ignore_blanks: true,
            ignore_comments: true,
            exclude_files: vec!["file1".to_string(), "file2".to_string()],
            show_stats: false,
            output: OutputFormat::Json,
            help: false
        });
    }
}