use crate::report::Report;
use crate::utils::format::OutputFormat;
use crate::saver::{FileSaver, SaveError};
use crate::saver::exporter::{ReportExporter, JsonExporter, CsvExporter};

/// 便捷函数：将报告保存到指定文件
///
/// # Arguments
/// * `report` - 要保存的报告
/// * `path` - 文件路径
/// * `format` - 输出格式
pub fn save_report<P: AsRef<std::path::Path>>(
    report: &Report,
    path: P,
    format: OutputFormat,
) -> Result<(), SaveError> {
    FileSaver::save_report(report, path, format)
}

/// 便捷函数：将报告导出为 JSON 字符串
pub fn report_to_json(report: &Report) -> Result<String, SaveError> {
    let mut buffer = Vec::new();
    let exporter = JsonExporter::new();
    exporter.export(report, &mut buffer)?;
    Ok(String::from_utf8(buffer).map_err(|e| SaveError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?)
}

/// 便捷函数：将报告导出为 CSV 字符串
pub fn report_to_csv(report: &Report) -> Result<String, SaveError> {
    let mut buffer = Vec::new();
    let exporter = CsvExporter::new();
    exporter.export(report, &mut buffer)?;
    Ok(String::from_utf8(buffer).map_err(|e| SaveError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?)
}

/// 便捷函数：将报告写入到任意实现了 Write 的目标中
///
/// 这个函数允许将报告导出到标准输出、内存缓冲区或任何其他实现了 Write trait 的目标
pub fn export_report<W: std::io::Write>(
    report: &Report,
    writer: &mut W,
    format: OutputFormat,
) -> Result<(), SaveError> {
    match format {
        OutputFormat::Json => {
            let exporter = crate::saver::JsonExporter::new();
            exporter.export(report, writer)
        },
        OutputFormat::Csv => {
            let exporter = crate::saver::CsvExporter::new();
            exporter.export(report, writer)
        },
        OutputFormat::Text => Err(SaveError::UnsupportedFormat),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::langs::lang_type::LangType;
    use crate::stats::FileStat;

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
        let json_str = report_to_json(&report).unwrap();

        // 验证 JSON 包含预期字段
        assert!(json_str.contains("languages"));
        assert!(json_str.contains("total"));
        assert!(json_str.contains("Rust"));
        assert!(json_str.contains("JavaScript"));
    }

    #[test]
    fn test_csv_conversion() {
        let report = create_test_report();
        let csv_str = report_to_csv(&report).unwrap();

        // 验证 CSV 包含头部
        assert!(csv_str.contains("Language,Files"));
        assert!(csv_str.contains("Rust"));
        assert!(csv_str.contains("JavaScript"));
        assert!(csv_str.contains("Total,"));
    }
}