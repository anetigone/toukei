use std::collections::HashMap;
use std::any::{TypeId, Any};

use crate::parser::any_value::AnyValue;
use crate::parser::parse_error::ParseError;
use crate::utils::format::OutputFormat;

#[derive(Debug, Clone)]
pub struct MatchedArg {
    values: Vec<AnyValue>,
}

impl MatchedArg {
    pub fn new() -> Self {
        Self {
            values: vec![],
        }
    }

    pub fn from_vec(vec: Vec<AnyValue>) -> Self { 
        Self {
            values: vec,
        }
    }

    pub fn push(&mut self, value: AnyValue) { 
        self.values.push(value);
    }
}


#[derive(Debug, Default)]
pub struct Matches {

    values: HashMap<String, MatchedArg>,
    types: HashMap<String, TypeId>,
}

impl Matches {

    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn  set(&mut self, name: &str, value: AnyValue, value_type: TypeId) -> Result<(), ParseError> {
        let matched = MatchedArg::from_vec(vec![value]);
        self.values.insert(name.to_string(), matched);
        self.types.insert(name.to_string(), value_type);

        Ok(())
    }

    pub fn append(&mut self, name: &str, value: AnyValue, value_type: TypeId) -> Result<(), ParseError> {
        if let Some(&stored_type) = self.types.get(name) {
            if stored_type != value_type {
                return Err(ParseError::ArgMismatch { 
                    name: name.to_string(), 
                    expected: value_type, 
                    actual: stored_type 
                });
            }
        } else {
            self.types.insert(name.to_string(), value_type);
        }

        self.values.entry(name.to_string()).or_insert_with(MatchedArg::new).push(value);

        Ok(())
    }

    pub fn get_one<T>(&self, name: &str) -> Result<&T, ParseError> 
    where
        T: Any + Send + Sync + 'static,
    {   
        let expected = TypeId::of::<T>();
        let actual = self.types.get(name)
            .ok_or_else(|| ParseError::UnknownArg(name.to_string()))?;

        if *actual != expected {
            return Err(ParseError::ArgMismatch { 
                name: name.to_string(), 
                expected, 
                actual: *actual 
            });
        }

        self.values.get(name)
            .and_then(|matched| matched.values.last())
            .and_then(|value| value.downcast_ref::<T>())
            .ok_or_else(|| ParseError::InternalError(std::fmt::format(format_args!("参数{}解析时出现内部错误，类型检查通过但是转换错误", name))))
    }

    pub fn get_many<T>(&self, name: &str) -> Result<Vec<&T>, ParseError> 
    where
        T: Any + Send + Sync + 'static,
    {
        let expected = TypeId::of::<T>();
        let actual = self.types.get(name)
            .ok_or_else(|| ParseError::UnknownArg(name.to_string()))?;

        if *actual != expected {
            return Err(ParseError::ArgMismatch { 
                name: name.to_string(), 
                expected, 
                actual: *actual 
            });
        }

        self.values.get(name)
            .and_then(|matched| 
                matched.values.iter()
                    .map(|value| value.downcast_ref::<T>())
                    .collect::<Option<Vec<_>>>())
            .ok_or_else(|| ParseError::InternalError(std::fmt::format(format_args!("参数{}解析时出现内部错误，类型检查通过但是转换错误", name))))
    } 

    pub fn try_get_one<T>(&self, name: &str) -> Option<&T>
    where
        T: Any + Send + Sync + 'static,
    {
        self.get_one(name).ok()
    }

    pub fn try_get_many<T>(&self, name: &str) -> Option<Vec<&T>>
    where
        T: Any + Send + Sync + 'static,
    {
        self.get_many(name).ok()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}

pub trait ExtractFromMatches: Sized {
    fn extract(matches: &Matches, name: &str) -> Option<Self>;
}

impl ExtractFromMatches for bool {
    fn extract(matches: &Matches, name: &str) -> Option<Self> {
        matches.try_get_one::<bool>(name).copied()
    }
}

// 为 String 实现
impl ExtractFromMatches for String {
    fn extract(matches: &Matches, name: &str) -> Option<Self> {
        matches.try_get_one::<String>(name).cloned()
    }
}

// 为 Vec<String> 实现 (处理 flatten 逻辑)
impl ExtractFromMatches for Vec<String> {
    fn extract(matches: &Matches, name: &str) -> Option<Self> {
        matches.try_get_many::<Vec<String>>(name)
            .map(|v| v.iter().flat_map(|inner| inner.iter()).cloned().collect())
    }
}

// 为 OutputFormat 实现 (Copy 类型)
impl ExtractFromMatches for OutputFormat {
    fn extract(matches: &Matches, name: &str) -> Option<Self> {
        matches.try_get_one::<OutputFormat>(name).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn test_matched_arg_new() {
        let arg = MatchedArg::new();
        assert!(arg.values.is_empty());
    }

    #[test]
    fn test_matched_arg_from_vec() {
        let vec = vec![AnyValue::new(42i32)];
        let arg = MatchedArg::from_vec(vec);
        assert_eq!(arg.values.len(), 1);
    }

    #[test]
    fn test_matched_arg_push() {
        let mut arg = MatchedArg::new();
        arg.push(AnyValue::new(42i32));
        assert_eq!(arg.values.len(), 1);
    }

    #[test]
    fn test_matches_new() {
        let matches = Matches::new();
        assert!(matches.values.is_empty());
        assert!(matches.types.is_empty());
    }

    #[test]
    fn test_set_and_get_one() {
        let mut matches = Matches::new();
        matches.set("number", AnyValue::new(42i32), TypeId::of::<i32>()).unwrap();
        
        assert!(matches.contains("number"));
        let value = matches.get_one::<i32>("number").unwrap();
        assert_eq!(*value, 42);
    }

    #[test]
    fn test_append_and_get_many() {
        let mut matches = Matches::new();
        matches.append("numbers", AnyValue::new(1i32), TypeId::of::<i32>()).unwrap();
        matches.append("numbers", AnyValue::new(2i32), TypeId::of::<i32>()).unwrap();
        
        let values = matches.get_many::<i32>("numbers").unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], &1);
        assert_eq!(values[1], &2);
    }

    #[test]
    fn test_type_mismatch() {
        let mut matches = Matches::new();
        matches.set("value", AnyValue::new(42i32), TypeId::of::<i32>()).unwrap();
        
        let result = matches.get_one::<String>("value");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::ArgMismatch { .. }));
    }

    #[test]
    fn test_try_get() {
        let mut matches = Matches::new();
        
        // 测试不存在的值
        assert!(matches.try_get_one::<i32>("nonexistent").is_none());
        assert!(matches.try_get_many::<i32>("nonexistent").is_none());
        
        // 测试存在的值
        matches.set("number", AnyValue::new(42i32), TypeId::of::<i32>()).unwrap();
        assert_eq!(*matches.try_get_one::<i32>("number").unwrap(), 42);
        
        matches.append("numbers", AnyValue::new(1i32), TypeId::of::<i32>()).unwrap();
        matches.append("numbers", AnyValue::new(2i32), TypeId::of::<i32>()).unwrap();
        let values = matches.try_get_many::<i32>("numbers").unwrap();
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_contains() {
        let mut matches = Matches::new();
        
        assert!(!matches.contains("test"));
        
        matches.set("test", AnyValue::new(42i32), TypeId::of::<i32>()).unwrap();
        assert!(matches.contains("test"));
    }

    #[test]
    fn test_complex_types() {
        let mut matches = Matches::new();
        let vec_value = vec![1, 2, 3];
        matches.set("vector", AnyValue::new(vec_value.clone()), TypeId::of::<Vec<i32>>()).unwrap();
        
        let retrieved = matches.get_one::<Vec<i32>>("vector").unwrap();
        assert_eq!(*retrieved, vec_value);
    }

    #[test]
    fn test_multiple_values() {
        let mut matches = Matches::new();
        
        matches.set("number", AnyValue::new(42i32), TypeId::of::<i32>()).unwrap();
        matches.set("text", AnyValue::new("hello".to_string()), TypeId::of::<String>()).unwrap();
        matches.set("flag", AnyValue::new(true), TypeId::of::<bool>()).unwrap();
        
        assert_eq!(*matches.get_one::<i32>("number").unwrap(), 42);
        assert_eq!(matches.get_one::<String>("text").unwrap(), "hello");
        assert_eq!(*matches.get_one::<bool>("flag").unwrap(), true);
    }
}
