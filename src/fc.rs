use crate::config::Config;
use crate::report::Report;
use crate::stats::{FileStat, LangStat};
use crate::counter::Counter;
use crate::walker::FileReader;

use log::warn;

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
                match self.counter.count(&file_path) {
                    Ok(stat) => report.add(stat),
                    Err(crate::counter::CounterError::BinaryFile) => {
                        // Skip binary files silently
                        warn!("Skipping binary file: {}", file_path.display());
                        continue;
                    }
                    Err(e) => return Err(format!("Failed to count file {:?}: {}", file_path, e)),
                }
            }
        }

        Ok(report)
    }
}