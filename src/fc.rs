use crate::config::Config;
use crate::report::Report;
use crate::stats::FileStat;
use crate::counter::Counter;
use crate::walker::FileReader;
use crate::counter::CounterError;

use log::warn;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use futures::future::join_all;

#[derive(Debug)]
pub struct FileCounter {
    config: Config,
    reader: FileReader,
}

impl FileCounter {
    pub fn new(config: Config) -> Self {
        let reader = FileReader::new(config.clone());
        FileCounter {
            config,
            reader,
        }
    }
}

impl FileCounter {
    pub fn process(&self) -> Result<Report, String> {
        let mut report = Report::new();
        // 先收集所有文件（单线程）
        let mut all_files = Vec::new();
        for path in self.config.paths.iter() {
            let files = self.reader.walk_dir(path)
                .map_err(|e| format!("Failed to walk directory: {}", e))?;
            all_files.extend(files);
        }

        // 创建线程池
        let num_threads = if self.config.num_workers > 0 {
            self.config.num_workers
        } else {
            num_cpus::get()
        };
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .map_err(|e| format!("Failed to build thread pool: {}", e))?;

        // 克隆 config 以供并行任务使用
        let config = self.config.clone();

        // 并行计数，收集每个文件的结果（跳过二进制文件，其他错误立刻返回）
        let results: Vec<Result<Option<FileStat>, String>> = thread_pool.install(|| {
            all_files
                .par_iter()
                .map(|file_path| {
                    // 每个任务创建自己的 Counter
                    let counter = Counter::new(config.clone());
                    match counter.count(file_path) {
                        Ok(stat) => Ok(Some(stat)),
                        Err(CounterError::BinaryFile) => {
                            warn!("Skipping binary file: {}", file_path.display());
                            Ok(None)
                        }
                        Err(e) => Err(format!("Failed to count file {:?}: {}", file_path, e)),
                    }
                })
                .collect()
        });

        for res in results {
            match res {
                Ok(Some(stat)) => report.add(stat),
                Ok(None) => (), // 二进制文件已跳过
                Err(e) => return Err(e),
            }
        }

        Ok(report)
    }
}

/// 异步版本的文件统计器
#[derive(Debug)]
pub struct AsyncFileCounter {
    config: Config,
    counter: Arc<Counter>,
    reader: Arc<FileReader>,
    num_workers: usize,
}

impl AsyncFileCounter {
    /// 创建异步文件统计器
    /// num_workers: 工作线程数，默认为CPU核心数
    pub fn new(config: Config) -> Self {
        let num_workers = if config.num_workers > 0 {
            config.num_workers
        } else {
            num_cpus::get()
        };

        let counter = Arc::new(Counter::new(config.clone()));
        let reader = Arc::new(FileReader::new(config.clone()));

        Self {
            config,
            counter,
            reader,
            num_workers,
        }
    }

    /// 设置工作线程数
    pub fn with_workers(mut self, num_workers: usize) -> Self {
        self.num_workers = num_workers;
        self
    }

    /// 异步处理文件
    pub async fn process(&self) -> Result<Report, String> {
        let (tx, rx) = mpsc::channel::<PathBuf>(self.num_workers * 2); // Buffer size = 2x workers
        let report = Arc::new(tokio::sync::Mutex::new(Report::new()));

        // 生产者任务
        let mut producer_handles = vec![];
        for path in self.config.paths.iter().cloned() {
            let tx_clone = tx.clone();
            let reader_clone = Arc::clone(&self.reader);

            let handle = tokio::spawn(async move {
                if let Err(e) = Self::produce_files(&path, reader_clone, tx_clone).await {
                    log::error!("Producer error for path {}: {}", path, e);
                }
            });
            producer_handles.push(handle);
        }

        // 丢弃原始发送者以关闭通道
        drop(tx);

        // 消费者任务
        let counter_clone = Arc::clone(&self.counter);
        let report_clone = Arc::clone(&report);
        let num_workers = self.num_workers;

        let consumer_handle = tokio::spawn(async move {
            let mut stream = rx;
            let semaphore = Arc::new(Semaphore::new(num_workers));
            let mut handles = vec![];

            while let Some(file_path) = stream.recv().await {
                let counter = Arc::clone(&counter_clone);
                let report = Arc::clone(&report_clone);
                let permit = Arc::clone(&semaphore);

                let handle = tokio::spawn(async move {
                    let _permit = permit.acquire().await.unwrap();
                    match counter.count_async(&file_path).await {
                        Ok(stat) => {
                            let mut report_guard = report.lock().await;
                            report_guard.add(stat);
                        }
                        Err(CounterError::BinaryFile) => {
                            warn!("Skipping binary file: {}", file_path.display());
                        }
                        Err(e) => {
                            log::error!("Failed to count file {:?}: {}", file_path, e);
                        }
                    }
                });
                handles.push(handle);
            }

            // 等待所有任务完成
            join_all(handles).await;
        });

        // 等待所有生产者完成
        join_all(producer_handles).await;
        consumer_handle.await.map_err(|e| format!("Consumer task failed: {}", e))?;

        // 获取最终报告
        let final_report = Arc::try_unwrap(report)
            .map_err(|_| "Failed to unwrap Arc: still multiple references".to_string())?
            .into_inner();

        Ok(final_report)
    }

    /// 生产者函数，遍历目录并发送文件路径到通道
    async fn produce_files(
        path: &str,
        reader: Arc<FileReader>,
        tx: mpsc::Sender<PathBuf>,
    ) -> Result<(), String> {
        // 保持walker为同步，使用tokio的spawn_blocking
        let path_owned = path.to_owned();
        let files = tokio::task::spawn_blocking(move || {
            reader.walk_dir(&path_owned)
        }).await
        .map_err(|e| format!("Failed to join blocking task: {}", e))?
        .map_err(|e| format!("Failed to walk directory: {}", e))?;

        // 发送文件路径到通道
        for file_path in files {
            if tx.send(file_path).await.is_err() {
                log::warn!("Channel closed, stopping producer");
                break;
            }
        }

        Ok(())
    }
}