use super::parse_error::ParseError;
use super::any_value::AnyValue;
use std::any::{Any, TypeId};

pub trait ParseValue: Send + Sync + 'static {
    type Output: Any + Send + Sync + 'static;
    fn parse(&self, value: &str) -> Result<Self::Output, ParseError>;
}

pub trait ValueParser: Send + Sync + 'static {
    fn parse(&self, value: &str) -> Result<AnyValue, ParseError>;
    fn type_id(&self) -> TypeId;
}

impl<P> ValueParser for P
where
    P: ParseValue + Send + Sync + 'static,
{
    fn parse(&self, value: &str) -> Result<AnyValue, ParseError> {
        let v = self.parse(value)?;
        Ok(AnyValue::new(v))
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<<P as ParseValue>::Output>()
    }
}

// 默认解析器，使用 FromStr 实现
pub struct DefaultParser<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for DefaultParser<T> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> ParseValue for DefaultParser<T>
where
    T: std::str::FromStr + Any + Send + Sync + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    type Output = T;

    fn parse(&self, s: &str) -> Result<Self::Output, ParseError> {
        s.parse::<T>()
            .map_err(|e| ParseError::FromString(e.to_string()))
    }
}

// 自定义解析器
pub struct CustomParser<T, F> {
    logic: F,
    _marker: std::marker::PhantomData<T>,
}

impl<T, F> CustomParser<T, F>
where
    F: Fn(&str) -> Result<T, ParseError> + Send + Sync + 'static,
    T: Any + Send + Sync + 'static,
{
    pub fn new(logic: F) -> Self {
        Self {
            logic,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, F> ParseValue for CustomParser<T, F>
where
    F: Fn(&str) -> Result<T, ParseError> + Send + Sync + 'static,
    T: Any + Send + Sync + 'static,
{
    type Output = T;

    fn parse(&self, s: &str) -> Result<Self::Output, ParseError> {
        (self.logic)(s)
    }
}

// 宏
#[macro_export]
macro_rules! value_parser {
    ($target_type:ty) => {{
        $crate::parser::value_parser::DefaultParser::<$target_type>::default()
    }};

    ($target_type:ty, $custom_logic:expr) => {{
        $crate::parser::value_parser::CustomParser::<$target_type, _>::new($custom_logic)
    }};
}