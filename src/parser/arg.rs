use strum_macros::EnumString;

use super::value_parser::ValueParser;
use super::parse_error::ParseError;

use crate::parser::any_value::AnyValue;
use crate::value_parser;

use std::any::{TypeId, Any};
use std::fmt::Debug;

#[derive(EnumString, Clone, Copy)]
pub enum ArgAction {
    Set,
    Append,
    Count,
    SetTrue,
}

pub struct Arg {
    name: String,
    short: Option<char>,
    long: Option<String>,
    help: String,
    action: ArgAction,
    required: bool,
    pub parser: Box<dyn ValueParser>,
    value_type: TypeId,
    conflicts: Vec<String>,
}

impl Arg {
    pub fn new(name: &str) -> Self {
        let default_parser = value_parser!(String);

        Arg {
            name: name.to_string(),
            short: None,
            long: None,
            help: "".to_string(),
            action: ArgAction::Set,
            required: false,
            parser: Box::new(default_parser),
            value_type: TypeId::of::<String>(),
            conflicts: vec![],
        }
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub fn long(mut self, long: &str) -> Self {
        self.long = Some(long.to_string());
        self
    }

    pub fn help(mut self, help: &str) -> Self {
        self.help = help.to_string();
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn action(mut self, action: ArgAction) -> Self {
        self.action = action;
        self
    }

    pub fn value_type(mut self, value_type: TypeId) -> Self {
        self.value_type = value_type;
        self
    }

    pub fn parser(mut self, parser: impl ValueParser + 'static) -> Self {
        self.value_type = parser.type_id();
        self.parser = Box::new(parser);
        self
    }

    pub fn default_parser<T>(mut self) -> Self
    where
        T: std::str::FromStr + Any + Send + Sync + 'static,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        let default_parser = value_parser!(T);
        self.value_type = TypeId::of::<T>();
        self.parser = Box::new(default_parser);
        self
    }

    pub fn custom_parser<T, F>(mut self, logic: F) -> Self
    where
        F: Fn(&str) -> Result<T, ParseError> + Send + Sync + 'static,
        T: Any + Send + Sync + 'static,
    {
        let custom_parser = value_parser!(T, logic);
        self.value_type = TypeId::of::<T>();
        self.parser = Box::new(custom_parser);
        self
    }

    pub fn conflicts_with(mut self, arg_name: &str) -> Self {
        self.conflicts.push(arg_name.to_string());
        self
    }

    pub fn parse(&self, value: &str) -> Result<AnyValue, ParseError> {
        self.parser.parse(value)
    }

}

impl Arg {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_short(&self) -> Option<char> {
        self.short
    }

    pub fn get_long(&self) -> Option<&str> {
        self.long.as_deref()
    }

    pub fn get_help(&self) -> &str {
        &self.help
    }

    pub fn get_action(&self) -> ArgAction {
        self.action
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn get_value_type(&self) -> TypeId {
        self.value_type
    }

    pub fn get_conflicts(&self) -> &[String] {
        &self.conflicts
    }

    pub fn is_conflict_with(&self, arg_name: &str) -> bool {
        self.conflicts.contains(&arg_name.to_string())
    }

}

impl Debug for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arg")
            .field("name", &self.name)
            .field("short", &self.short)
            .field("long", &self.long)
            .field("help", &self.help)
            .field("required", &self.required)
            .field("value_type", &self.value_type)
            .field("conflicts", &self.conflicts)
            .finish()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_arg() {
        let arg = Arg::new("test");
        assert_eq!(arg.get_name(), "test");
        assert_eq!(arg.get_short(), None);
        assert_eq!(arg.get_long(), None);
        assert_eq!(arg.get_help(), "");
        assert!(!arg.is_required());
        assert_eq!(arg.get_conflicts().len(), 0);
    }

    #[test]
    fn test_arg_with_short() {
        let arg = Arg::new("test").short('t');
        assert_eq!(arg.get_short(), Some('t'));
    }

    #[test]
    fn test_arg_with_long() {
        let arg = Arg::new("test").long("test-long");
        assert_eq!(arg.get_long(), Some("test-long"));
    }

    #[test]
    fn test_arg_with_help() {
        let arg = Arg::new("test").help("Test help message");
        assert_eq!(arg.get_help(), "Test help message");
    }

    #[test]
    fn test_required_arg() {
        let arg = Arg::new("test").required();
        assert!(arg.is_required());
    }

    #[test]
    fn test_arg_with_conflicts() {
        let arg = Arg::new("test").conflicts_with("other");
        assert!(arg.is_conflict_with("other"));
        assert!(!arg.is_conflict_with("different"));
    }

    #[test]
    fn test_default_parser() {
        let arg = Arg::new("test").default_parser::<i32>();
        assert_eq!(arg.get_value_type(), TypeId::of::<i32>());
        
        // Test parsing
        let result = arg.parse("42");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value.downcast_ref::<i32>(), Some(&42));
    }

    #[test]
    fn test_custom_parser() {
        let arg = Arg::new("test").custom_parser(|s| {
            Ok(s.trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>())
        });

        let result = arg.parse("a,b,c");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value.downcast_ref::<Vec<String>>(), Some(&vec!["a".to_string(), "b".to_string(), "c".to_string()]));
    }

    #[test]
    fn test_parser_error() {
        let arg = Arg::new("test").default_parser::<i32>();
        let result = arg.parse("not_a_number");
        assert!(result.is_err());
    }
}
