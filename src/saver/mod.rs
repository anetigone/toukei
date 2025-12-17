pub mod exporter;
pub mod save_error;
pub use exporter::{ReportExporter, JsonExporter, CsvExporter};
pub use save_error::SaveError;

use std::fs::File;
use std::path::Path;
use crate::report::Report;
use crate::utils::format::OutputFormat;

/// 文件保存器，支持多种格式导出统计结果
pub struct FileSaver;

impl FileSaver {
    pub fn new() -> Self {
        FileSaver
    }
}

impl FileSaver {
    /// 将报告保存到指定文件
    pub fn save_report<P: AsRef<Path>>(
        report: &Report,
        path: P,
        format: OutputFormat,
    ) -> Result<(), SaveError> {
        let mut file = File::create(path).map_err(SaveError::Io)?;

        match format {
            OutputFormat::Json => {
                let exporter = JsonExporter::new();
                exporter.export(report, &mut file)
            },
            OutputFormat::Csv => {
                let exporter = CsvExporter::new();
                exporter.export(report, &mut file)
            },
            OutputFormat::Text => Err(SaveError::UnsupportedFormat),
        }
    }

    /// 使用自定义导出器保存报告
    pub fn save_report_with_exporter<P: AsRef<Path>>(
        report: &Report,
        path: P,
        exporter: &dyn ReportExporter,
    ) -> Result<(), SaveError> {
        let mut file = File::create(path).map_err(SaveError::Io)?;
        exporter.export(report, &mut file)
    }
}