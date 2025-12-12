use crate::config::Config;
use crate::report::Report;
use crate::stats::{FileStat, LangStat};
use crate::counter::Counter;
use crate::walker::FileReader;

#[derive(Debug)]
pub struct FileCounter {
    config: Config,
    counter: Counter,
    reader: FileReader,
}

impl FileCounter {
    pub fn new(config: Config) -> Self {
        let counter = Counter::new(config.clone());
        let reader = FileReader::new(config.clone());
        FileCounter {
            config,
            counter,
            reader,
        }
    }
}

impl FileCounter {
    pub fn process(&self) -> Result<Report, String> {
        let mut report = Report::new();
        for path in self.config.paths.iter() {
            let files = self.reader.walk_dir(path)
                .map_err(|e| format!("Failed to walk directory: {}", e))?;
            for file_path in files {
                let stat = self.counter.count(&file_path)
                    .map_err(|e| format!("Failed to count file {:?}: {}", file_path, e))?;
                report.add(stat);
            }
        }

        Ok(report)
    }
}