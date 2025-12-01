use std::any::TypeId;

#[derive(Debug)]
pub enum ParseError {
    FromString(String),
    ArgMismatch{name: String, expected: TypeId, actual: TypeId},
    UnknownArg(String),
    InternalError(String),
    MissingRequired(String),
    UnknownFlag(String),
    InvalidOutputFormat(String),
    NoValue(String),
    BadValue { arg: String, ty: &'static str, msg: String },
    Conflict { a: String, b: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::FromString(msg) => write!(f, "parse error: {}", msg),
            ParseError::ArgMismatch{name, expected, actual} => write!(
                f, 
                "参数 '{}' 类型不匹配，期望 {:?}，实际 {:?}", 
                name, expected, actual
            ),
            ParseError::InternalError(msg) => write!(f, "internal error: {}", msg),
            ParseError::UnknownArg(arg) => write!(f, "unknown argument: {}", arg),
            ParseError::MissingRequired(arg) => write!(f, "missing required argument: {}", arg),
            ParseError::UnknownFlag(flag) => write!(f, "unknown flag: {}", flag),
            ParseError::InvalidOutputFormat(format) => write!(f, "invalid output format: {}", format),
            ParseError::NoValue(arg) => write!(f, "no value provided for argument: {}", arg),
            ParseError::BadValue { arg, ty, msg } => write!(
                f,
                "bad value for argument {}: expected {}, {}",
                arg, ty, msg
            ),
            ParseError::Conflict { a, b } => write!(
                f,
                "argument {} conflicts with argument {}",
                a, b
            ),
        }
    }
}

impl std::error::Error for ParseError {}