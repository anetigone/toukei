use toukei::config::Config;
use toukei::fc::{FileCounter, AsyncFileCounter};

// 统计结果
fn calculate_totals(report: &toukei::report::Report) -> (usize, usize, usize, usize, usize) {
    let mut total_files = 0;
    let mut total_lines = 0;
    let mut total_code = 0;
    let mut total_comments = 0;
    let mut total_blanks = 0;

    for lang_stat in report.inner.values() {
        total_files += lang_stat.files;
        total_lines += lang_stat.lines;
        total_code += lang_stat.code;
        total_comments += lang_stat.comments;
        total_blanks += lang_stat.blanks;
    }

    (total_files, total_lines, total_code, total_comments, total_blanks)
}

#[tokio::test]
async fn test_async_file_counter() {
    let mut config = Config::new();
    // Use the current directory's absolute path
    let current_dir = std::env::current_dir().unwrap();
    config.paths.push(current_dir.to_str().unwrap().to_string());

    // 测试同步与异步统计结果一致性
    let sync_counter = FileCounter::new(config.clone());
    let sync_report = sync_counter.process().unwrap();

    let async_counter = AsyncFileCounter::new(config);
    let async_report = async_counter.process().await.unwrap();

    let (sync_files, sync_lines, sync_code, sync_comments, sync_blanks) = calculate_totals(&sync_report);
    let (async_files, async_lines, async_code, async_comments, async_blanks) = calculate_totals(&async_report);

    assert!(sync_files > 0);
    assert_eq!(sync_files, async_files);

    // 比较各项统计数据是否一致
    assert_eq!(sync_lines, async_lines);
    assert_eq!(sync_code, async_code);
    assert_eq!(sync_comments, async_comments);
    assert_eq!(sync_blanks, async_blanks);
}

#[tokio::test]
async fn test_async_counter_with_custom_workers() {
    let mut config = Config::new();
    // Use the current directory's absolute path
    let current_dir = std::env::current_dir().unwrap();
    config.paths.push(current_dir.to_str().unwrap().to_string());

    // 使用自定义工作线程数的异步文件统计器
    let async_counter = AsyncFileCounter::new(config).with_workers(2);
    let report = async_counter.process().await.unwrap();

    let (total_files, _, _, _, _) = calculate_totals(&report);
    assert!(total_files > 0);
}

#[tokio::test]
async fn test_async_vs_sync_performance() {
    use std::time::Instant;

    let mut config = Config::new();
    // Use the current directory's absolute path
    let current_dir = std::env::current_dir().unwrap();
    config.paths.push(current_dir.to_str().unwrap().to_string());

    // 测试同步版本的性能
    let sync_start = Instant::now();
    let sync_counter = FileCounter::new(config.clone());
    let sync_report = sync_counter.process().unwrap();
    let sync_duration = sync_start.elapsed();

    // 测试异步版本的性能
    let async_start = Instant::now();
    let async_counter = AsyncFileCounter::new(config);
    let async_report = async_counter.process().await.unwrap();
    let async_duration = async_start.elapsed();

    println!("Sync duration: {:?}", sync_duration);
    println!("Async duration: {:?}", async_duration);

    let (_, sync_lines, _, _, _) = calculate_totals(&sync_report);
    let (_, async_lines, _, _, _) = calculate_totals(&async_report);
    assert_eq!(sync_lines, async_lines);

    println!("Performance ratio (async/sync): {:.2}",
             async_duration.as_millis() as f64 / sync_duration.as_millis() as f64);
}