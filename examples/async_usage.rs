//! Async FileCounter 使用示例
//!
//! 这个示例展示了如何使用异步版本的 FileCounter 来并行处理文件

use std::env;
use toukei::config::Config;
use toukei::fc::AsyncFileCounter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置
    let mut config = Config::new();

    // 从命令行参数获取路径，如果没有则使用当前目录
    let args: Vec<String> = env::args().skip(1).collect();
    if !args.is_empty() {
        config.paths = args;
    } else {
        config.paths = vec![env::current_dir()?.to_str().unwrap().to_string()];
    }

    println!("正在扫描目录: {:?}", config.paths);
    println!("支持的语言类型: {:?}", config.types);

    // 创建异步文件计数器
    // 默认工作线程数 = CPU 核心数
    // 也可以通过环境变量 TOUKEI_WORKERS 设置
    let async_counter = AsyncFileCounter::new(config);

    // 或者使用自定义工作线程数
    // let async_counter = AsyncFileCounter::new(config).with_workers(4);

    // 执行异步统计
    let report = async_counter.process().await?;

    // 输出统计结果
    println!("\n统计结果:");
    println!("{:<15} {:<10} {:<10} {:<10} {:<10} {:<10}",
             "语言", "文件数", "总行数", "代码行", "注释行", "空白行");
    println!("{}", "-".repeat(80));

    let mut total_files = 0;
    let mut total_lines = 0;
    let mut total_code = 0;
    let mut total_comments = 0;
    let mut total_blanks = 0;

    for (lang, stat) in report.inner.iter() {
        println!("{:<15} {:<10} {:<10} {:<10} {:<10} {:<10}",
                 format!("{:?}", lang),
                 stat.files,
                 stat.lines,
                 stat.code,
                 stat.comments,
                 stat.blanks);

        total_files += stat.files;
        total_lines += stat.lines;
        total_code += stat.code;
        total_comments += stat.comments;
        total_blanks += stat.blanks;
    }

    println!("{}", "-".repeat(80));
    println!("{:<15} {:<10} {:<10} {:<10} {:<10} {:<10}",
             "总计", total_files, total_lines, total_code, total_comments, total_blanks);

    Ok(())
}