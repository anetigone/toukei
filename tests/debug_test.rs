#[test]
fn test_debug_file_counting() {
    use toukei::config::Config;
    use toukei::fc::FileCounter;
    use toukei::walker::FileReader;

    let config = Config::new();
    println!("Config paths: {:?}", config.paths);
    println!("Config types: {:?}", config.types);
    println!("Config exclude_files: {:?}", config.exclude_files);

    let reader = FileReader::new(config.clone());

    // Try to walk the directory
    match reader.walk_dir(".") {
        Ok(files) => {
            println!("Found {} files", files.len());
            if files.len() > 0 {
                println!("First few files:");
                for (i, file) in files.iter().take(5).enumerate() {
                    println!("  {}: {:?}", i, file);
                }
            }
        }
        Err(e) => {
            println!("Error walking directory: {}", e);
        }
    }

    // Try with full path
    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    match reader.walk_dir(current_dir.to_str().unwrap()) {
        Ok(files) => {
            println!("Found {} files with full path", files.len());
        }
        Err(e) => {
            println!("Error walking directory with full path: {}", e);
        }
    }

    // Try the full counter
    let counter = FileCounter::new(config);
    match counter.process() {
        Ok(report) => {
            println!("Counter processed successfully!");
            let mut total_files = 0;
            for (lang, stat) in report.inner.iter() {
                println!("  {:?}: {} files", lang, stat.files);
                total_files += stat.files;
            }
            println!("Total files: {}", total_files);
        }
        Err(e) => {
            println!("Counter error: {}", e);
        }
    }
}