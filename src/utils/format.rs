use std::str::FromStr;
use strum_macros::Display;

#[derive(Debug, Clone, Copy, Display)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Text
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}

impl PartialEq<Self> for OutputFormat {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text, Self::Text) => true,
            (Self::Json, Self::Json) => true,
            (Self::Csv, Self::Csv) => true,
            _ => false,
        }
    }
}

impl Eq for OutputFormat {}

impl std::hash::Hash for OutputFormat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}