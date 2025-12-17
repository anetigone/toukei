use std::io::Write;
use crate::report::Report;
use super::SaveError;

/// 导出策略 Trait
pub trait ReportExporter {
    /// 将报告导出写入到实现了 Write 的目标中
    fn export(&self, report: &Report, writer: &mut dyn Write) -> Result<(), SaveError>;
}

/// JSON 导出器
pub struct JsonExporter;

impl JsonExporter {
    pub fn new() -> Self {
        JsonExporter
    }
}

impl Default for JsonExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportExporter for JsonExporter {
    fn export(&self, report: &Report, writer: &mut dyn Write) -> Result<(), SaveError> {
        let json_data = self.format_as_json(report)?;
        writer.write_all(json_data.as_bytes()).map_err(SaveError::Io)?;
        Ok(())
    }
}

impl JsonExporter {
    /// 将报告格式化为 JSON 字符串
    fn format_as_json(&self, report: &Report) -> Result<String, SaveError> {
        let mut json_data = serde_json::json!({
            "languages": []
        });

        // 使用 Report 的排序方法
        let items = report.sort_stats(|a, b| b.1.lines.cmp(&a.1.lines));

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
}

/// CSV 导出器
pub struct CsvExporter;

impl CsvExporter {
    pub fn new() -> Self {
        CsvExporter
    }
}

impl Default for CsvExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportExporter for CsvExporter {
    fn export(&self, report: &Report, writer: &mut dyn Write) -> Result<(), SaveError> {
        let csv_data = self.format_as_csv(report)?;
        writer.write_all(csv_data.as_bytes()).map_err(SaveError::Io)?;
        Ok(())
    }
}

impl CsvExporter {
    /// 将报告格式化为 CSV 字符串
    fn format_as_csv(&self, report: &Report) -> Result<String, SaveError> {
        let mut csv_data = String::new();

        // CSV 头部
        csv_data.push_str("Language,Files,Lines,Code,Comments,Blanks,Functions,Classes\n");

        // 使用 Report 的排序方法
        let items = report.sort_stats(|a, b| b.1.lines.cmp(&a.1.lines));

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