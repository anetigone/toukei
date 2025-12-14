#[test]
fn test_debug_file_processing() {
    use toukei::config::Config;
    use toukei::fc::FileCounter;
    use toukei::walker::FileReader;
    use toukei::langs::registry::get_type_from_ext;
    use toukei::langs::lang_type::LangType;

    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    let mut config = Config::new();
    config.paths = vec![current_dir.to_str().unwrap().to_string()];

    let reader = FileReader::new(config.clone());

    // Try to walk the directory
    match reader.walk_dir(&config.paths[0]) {
        Ok(files) => {
            println!("Found {} files", files.len());
            if files.len() > 0 {
                println!("First few files with extensions:");
                for (i, file) in files.iter().take(10).enumerate() {
                    let ext = file.extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("no_ext");
                    let lang = get_type_from_ext(ext).unwrap_or(LangType::Unknown);
                    println!("  {}: {:?} -> ext: {}, lang: {:?}", i, file, ext, lang);
                }
            }
        }
        Err(e) => {
            println!("Error walking directory: {}", e);
        }
    }

    // Try the full counter
    let counter = FileCounter::new(config);
    match counter.process() {
        Ok(report) => {
            println!("Counter processed successfully!");
            println!("Report inner has {} entries", report.inner.len());
            for (lang, stat) in report.inner.iter() {
                println!("  {:?}: {} files, {} lines", lang, stat.files, stat.lines);
            }
        }
        Err(e) => {
            println!("Counter error: {}", e);
        }
    }
}