use toukei::config::Config;
use toukei::fc::FileCounter;
use toukei::utils::format::OutputFormat;
use toukei::saver::FileSaver;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let working_dir = std::env::current_dir()?;
    // 创建配置
    let config = Config::new()
        .with_paths(vec![working_dir.display().to_string()])
        .with_num_workers(4);

    // 运行统计
    println!("开始统计代码...");
    let counter = FileCounter::new(config);
    let report = counter.process()?;

    println!("统计完成！");

    // 保存为 JSON
    println!("保存为 JSON 格式...");
    FileSaver::save_report(&report, "stats.json", OutputFormat::Json)?;
    println!("已保存到 stats.json");

    // 保存为 CSV
    println!("保存为 CSV 格式...");
    FileSaver::save_report(&report, "stats.csv", OutputFormat::Csv)?;
    println!("已保存到 stats.csv");

    Ok(())
}