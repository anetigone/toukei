use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic;

use serde::{Deserialize, Serialize};
use toukei::config::Config;
use toukei::fc::FileCounter;
use toukei::fc::AsyncFileCounter;
use toukei::report::Report;
use toukei::utils::format::OutputFormat;

/// FFI input configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct FfiConfig {

    pub paths: Vec<String>,
    pub types: Option<Vec<String>>,
    pub exclude_files: Option<Vec<String>>,
    pub ignore_blanks: Option<bool>,
    pub ignore_comments: Option<bool>,
    pub enable_async: Option<bool>,
    pub num_workers: Option<usize>,
}

/// FFI output result structure
#[derive(Debug, Serialize)]
pub struct FfiResult {

    pub success: bool,
    pub error: Option<String>,
    pub languages: Vec<LanguageStat>,
    pub total: Totals,
}

/// Individual language statistics
#[derive(Debug, Serialize)]
pub struct LanguageStat {

    pub language: String,
    pub files: usize,
    pub lines: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
    pub functions: usize,
    pub classes: usize,
}

/// Total statistics across all languages
#[derive(Debug, Serialize)]
pub struct Totals {

    pub files: usize,
    pub lines: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
    pub functions: usize,
    pub classes: usize,
}

/// Convert FfiConfig to internal Config
impl From<FfiConfig> for Config {
    fn from(ffi_config: FfiConfig) -> Self {
        let mut config = Config::new();

        config.paths = ffi_config.paths;

        if let Some(types) = ffi_config.types {
            config.types = types;
        }

        if let Some(exclude_files) = ffi_config.exclude_files {
            config.exclude_files = exclude_files;
        }

        if let Some(ignore_blanks) = ffi_config.ignore_blanks {
            config.ignore_blanks = ignore_blanks;
        }

        if let Some(ignore_comments) = ffi_config.ignore_comments {
            config.ignore_comments = ignore_comments;
        }

        if let Some(enable_async) = ffi_config.enable_async {
            config.enable_async = enable_async;
        }

        if let Some(num_workers) = ffi_config.num_workers {
            config.num_workers = num_workers;
        }

        config
    }
}

/// Convert Report to FfiResult
impl From<Report> for FfiResult {
    fn from(report: Report) -> Self {
        let mut languages = Vec::new();
        let mut totals = Totals {
            files: 0,
            lines: 0,
            code: 0,
            comments: 0,
            blanks: 0,
            functions: 0,
            classes: 0,
        };

        for (lang_type, lang_stat) in &report {
            let lang_stat = LanguageStat {
                language: lang_type.to_string(),
                files: lang_stat.files,
                lines: lang_stat.lines,
                code: lang_stat.code,
                comments: lang_stat.comments,
                blanks: lang_stat.blanks,
                functions: lang_stat.functions,
                classes: lang_stat.classes,
            };

            totals.files += lang_stat.files;
            totals.lines += lang_stat.lines;
            totals.code += lang_stat.code;
            totals.comments += lang_stat.comments;
            totals.blanks += lang_stat.blanks;
            totals.functions += lang_stat.functions;
            totals.classes += lang_stat.classes;

            languages.push(lang_stat);
        }

        // Sort by lines descending
        languages.sort_by(|a, b| b.lines.cmp(&a.lines));

        FfiResult {
            success: true,
            error: None,
            languages,
            total: totals,
        }
    }
}

/// FFI 接口，接受一个 JSON 格式的配置字符串，返回一个 JSON 格式的统计结果字符串
///
/// # 安全性
///
/// 此函数是 FFI 接口，因此需要确保传入的字符串是有效的 UTF-8
/// 并且在使用后正确释放内存
#[no_mangle]
pub unsafe extern "C" fn analyze_code(json_config: *const c_char) -> *mut c_char {
    let result = panic::catch_unwind(|| {
        if json_config.is_null() {
            return create_error_response("Input config is null");
        }

        let c_str = match CStr::from_ptr(json_config).to_str() {
            Ok(s) => s,
            Err(_) => return create_error_response("Invalid UTF-8 in input"),
        };

        let ffi_config: FfiConfig = match serde_json::from_str(c_str) {
            Ok(config) => config,
            Err(e) => return create_error_response(&format!("Failed to parse JSON: {}", e)),
        };

        let config = Config::from(ffi_config);

        let report = if config.enable_async {
            use tokio::runtime::Runtime;
            let rt = match Runtime::new() {
                Ok(rt) => rt,
                Err(e) => return create_error_response(&format!("Failed to create async runtime: {}", e)),
            };

            match rt.block_on(run_async_analysis(config)) {
                Ok(report) => report,
                Err(e) => return create_error_response(&format!("Async analysis failed: {}", e)),
            }
        } else {
            match run_sync_analysis(config) {
                Ok(report) => report,
                Err(e) => return create_error_response(&format!("Sync analysis failed: {}", e)),
            }
        };

        let ffi_result: FfiResult = FfiResult::from(report);

        match serde_json::to_string(&ffi_result) {
            Ok(json) => {
                match CString::new(json) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => create_error_response("Failed to create result string"),
                }
            }
            Err(e) => create_error_response(&format!("Failed to serialize result: {}", e)),
        }
    });

    match result {
        Ok(ptr) => ptr,
        Err(_) => create_error_response("Panic occurred during analysis"),
    }
}

/// 释放结果字符串
///
/// # 安全性
///
/// 指针必须要释放
#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}

/// 创建错误响应
fn create_error_response(error_msg: &str) -> *mut c_char {
    let error_result = FfiResult {
        success: false,
        error: Some(error_msg.to_string()),
        languages: Vec::new(),
        total: Totals {
            files: 0,
            lines: 0,
            code: 0,
            comments: 0,
            blanks: 0,
            functions: 0,
            classes: 0,
        },
    };

    match serde_json::to_string(&error_result) {
        Ok(json) => {
            match CString::new(json) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => {
                    // Fallback: create a simple error message
                    let error_json = format!(r#"{{"success":false,"error":"{}"}}"#, error_msg);
                    CString::new(error_json).unwrap().into_raw()
                }
            }
        }
        Err(_) => {
            // Ultimate fallback
            CString::new(r#"{"success":false,"error":"Failed to create error response"}"#).unwrap().into_raw()
        }
    }
}

/// 运行同步模式分析
fn run_sync_analysis(config: Config) -> Result<Report, String> {
    let counter = FileCounter::new(config);
    counter.process().map_err(|e| format!("Processing failed: {}", e))
}

/// 运行异步模式分析
async fn run_async_analysis(config: Config) -> Result<Report, String> {
    let mut async_counter = AsyncFileCounter::new(config.clone());

    if config.num_workers > 0 {
        async_counter = async_counter.with_workers(config.num_workers);
    }

    async_counter.process()
        .await
        .map_err(|e| format!("Async processing failed: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_config_to_config() {
        let ffi_config = FfiConfig {
            paths: vec!["src".to_string()],
            types: Some(vec!["rs".to_string()]),
            ignore_blanks: Some(true),
            ignore_comments: Some(false),
            enable_async: Some(true),
            num_workers: Some(4),
            exclude_files: None,
        };

        let config: Config = ffi_config.into();

        assert_eq!(config.paths, vec!["src"]);
        assert_eq!(config.types, vec!["rs"]);
        assert_eq!(config.ignore_blanks, true);
        assert_eq!(config.ignore_comments, false);
        assert_eq!(config.enable_async, true);
        assert_eq!(config.num_workers, 4);
    }

    #[test]
    fn test_error_response() {
        let ptr = create_error_response("Test error");
        assert!(!ptr.is_null());

        unsafe {
            let c_str = CStr::from_ptr(ptr);
            let json_str = c_str.to_str().unwrap();
            assert!(json_str.contains("Test error"));
            assert!(json_str.contains("\"success\":false"));

            free_string(ptr);
        }
    }
}