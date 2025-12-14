use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::report::Report;
use crate::utils::format::OutputFormat;

/// 文件保存器，支持多种格式导出统计结果
pub struct FileSaver;

impl FileSaver {
    /// 将报告保存到指定文件
    pub fn save_report<P: AsRef<Path>>(
        report: &Report,
        path: P,
        format: OutputFormat,
    ) -> Result<(), SaveError> {
        match format {
            OutputFormat::Json => Self::save_as_json(report, path),
            OutputFormat::Csv => Self::save_as_csv(report, path),
            OutputFormat::Text => Err(SaveError::UnsupportedFormat),
        }
    }

    /// 保存为 JSON 格式
    fn save_as_json<P: AsRef<Path>>(report: &Report, path: P) -> Result<(), SaveError> {
        let json_data = Self::report_to_json(report)?;
        let mut file = File::create(path).map_err(SaveError::Io)?;
        file.write_all(json_data.as_bytes()).map_err(SaveError::Io)?;
        Ok(())
    }

    /// 保存为 CSV 格式
    fn save_as_csv<P: AsRef<Path>>(report: &Report, path: P) -> Result<(), SaveError> {
        let csv_data = Self::report_to_csv(report)?;
        let mut file = File::create(path).map_err(SaveError::Io)?;
        file.write_all(csv_data.as_bytes()).map_err(SaveError::Io)?;
        Ok(())
    }

    /// 将报告转换为 JSON 格式字符串
    fn report_to_json(report: &Report) -> Result<String, SaveError> {
        let mut json_data = serde_json::json!({
            "languages": []
        });

        // 收集并排序数据
        let mut items: Vec<_> = report.into_iter().collect();
        items.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));

        let mut languages = Vec::new();
        let mut total_files = 0;
        let mut total_lines = 0;
        let mut total_code = 0;
        let mut total_comments = 0;
        let mut total_blanks = 0;
        let mut total_functions = 0;
        let mut total_classes = 0;

        for (lang, stat) in items {
            let lang_data = serde_json::json!({
                "language": lang.to_string(),
                "files": stat.files,
                "lines": stat.lines,
                "code": stat.code,
                "comments": stat.comments,
                "blanks": stat.blanks,
                "functions": stat.functions,
                "classes": stat.classes
            });
            languages.push(lang_data);

            total_files += stat.files;
            total_lines += stat.lines;
            total_code += stat.code;
            total_comments += stat.comments;
            total_blanks += stat.blanks;
            total_functions += stat.functions;
            total_classes += stat.classes;
        }

        json_data["languages"] = serde_json::Value::Array(languages);
        json_data["total"] = serde_json::json!({
            "files": total_files,
            "lines": total_lines,
            "code": total_code,
            "comments": total_comments,
            "blanks": total_blanks,
            "functions": total_functions,
            "classes": total_classes
        });

        serde_json::to_string_pretty(&json_data).map_err(SaveError::Json)
    }

    /// 将报告转换为 CSV 格式字符串
    fn report_to_csv(report: &Report) -> Result<String, SaveError> {
        let mut csv_data = String::new();

        // CSV 头部
        csv_data.push_str("Language,Files,Lines,Code,Comments,Blanks,Functions,Classes\n");

        // 收集并排序数据
        let mut items: Vec<_> = report.into_iter().collect();
        items.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));

        let mut total_files = 0;
        let mut total_lines = 0;
        let mut total_code = 0;
        let mut total_comments = 0;
        let mut total_blanks = 0;
        let mut total_functions = 0;

        // 写入每种语言的数据
        for (lang, stat) in items {
            csv_data.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                lang.to_string(),
                stat.files,
                stat.lines,
                stat.code,
                stat.comments,
                stat.blanks,
                stat.functions,
            ));

            total_files += stat.files;
            total_lines += stat.lines;
            total_code += stat.code;
            total_comments += stat.comments;
            total_blanks += stat.blanks;
            total_functions += stat.functions;
        }

        // 添加分隔线
        csv_data.push_str(",,,,,,,,\n");

        // 添加总计行
        csv_data.push_str(&format!(
            "Total,{},{},{},{},{},{}\n",
            total_files, total_lines, total_code, total_comments, total_blanks, total_functions
        ));

        Ok(csv_data)
    }
}

/// 保存错误类型
#[derive(Debug)]
pub enum SaveError {
    Io(std::io::Error),
    Json(serde_json::Error),
    UnsupportedFormat,
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Io(e) => write!(f, "IO error: {}", e),
            SaveError::Json(e) => write!(f, "JSON error: {}", e),
            SaveError::UnsupportedFormat => write!(f, "Unsupported output format for saving"),
        }
    }
}

impl std::error::Error for SaveError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::langs::lang_type::LangType;
    use crate::stats::{LangStat, FileStat};
    use std::collections::HashMap;

    fn create_test_report() -> Report {
        let mut report = Report::new();

        let rust_stat = FileStat {
            lang: LangType::Rust,
            path: "test.rs".to_string(),
            name: "test.rs".to_string(),
            lines: 100,
            code: 80,
            comments: 10,
            blanks: 10,
            functions: 5,
            classes: 2,
        };

        let js_stat = FileStat {
            path: "test.js".to_string(),
            name: "test.js".to_string(),
            lang: LangType::Javascript,
            lines: 50,
            code: 40,
            comments: 5,
            blanks: 5,
            functions: 3,
            classes: 1,
        };

        report.add(rust_stat);
        report.add(js_stat);

        report
    }

    #[test]
    fn test_json_conversion() {
        let report = create_test_report();
        let json_str = FileSaver::report_to_json(&report).unwrap();

        // 验证 JSON 包含预期字段
        assert!(json_str.contains("languages"));
        assert!(json_str.contains("total"));
        assert!(json_str.contains("Rust"));
        assert!(json_str.contains("JavaScript"));
    }

    #[test]
    fn test_csv_conversion() {
        let report = create_test_report();
        let csv_str = FileSaver::report_to_csv(&report).unwrap();

        // 验证 CSV 包含头部
        assert!(csv_str.contains("Language,Files"));
        assert!(csv_str.contains("Rust"));
        assert!(csv_str.contains("JavaScript"));
        assert!(csv_str.contains("Total,"));
    }
}