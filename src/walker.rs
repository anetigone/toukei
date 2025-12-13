use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::{config::Config, langs::{lang_type::LangType, registry::get_type_from_ext}};

#[derive(Debug)]
pub struct FileReader {
    
    config: Config
}

impl FileReader {
    pub fn new(config: Config) -> Self {
        FileReader {
            config
        }
    }

    pub fn walk_dir<P>(&self, path: P) -> Result<Vec<PathBuf>, std::io::Error>
    where
        P: AsRef<Path>,
    { 
        let root = path.as_ref().to_path_buf();
        let files = WalkDir::new(&root)
            .into_iter()
            .filter_entry(|entry| {
                let p = entry.path();
                if p == root.as_path() {
                    return true;
                }

                if entry.file_type().is_dir() {
                    if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with(".") {
                            return false;
                        }
                    }

                    for excl in &self.config.exclude_files {
                        if excl.is_empty() {
                            continue;
                        }
                        let excl_path = Path::new(excl);
                        if p.ends_with(excl_path) {
                            return false;
                        }
                    }
                }
                true
            })
            .filter_map(|e| e.ok())
            .filter(|entry| self.include_entry(entry))
            .map(|entry| entry.path().to_path_buf())
            .collect::<Vec<PathBuf>>();

        Ok(files)
    }

    fn include_entry(&self, entry: &DirEntry) -> bool {
        let path = entry.path();

        // 只处理文件
        if entry.file_type().is_dir() {
            return false;
        }

        // 排除任何路径组件以 '.' 开头的（隐藏文件或位于隐藏目录下）
        for comp in path.components() {
            if let Some(s) = comp.as_os_str().to_str() {
                if s.starts_with('.') {
                    return false;
                }
            }
        }

        // 排除配置中指定的文件或目录（支持相对/绝对路径片段或名字）
        let path_str = path.to_string_lossy().to_lowercase();
        for excl in &self.config.exclude_files {
            if excl.is_empty() {
                continue;
            }
            let excl_lower = excl.to_lowercase();
            let excl_path = Path::new(excl);
            if path.ends_with(excl_path) || path_str.contains(&excl_lower) {
                return false;
            }
        }

        // 仅包含指定类型：根据扩展名判定语言类型，然后与配置 types 比较
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_str = ext.to_lowercase();
            let lang = get_type_from_ext(&ext_str).unwrap_or(LangType::Unknown);
            match lang {
                LangType::Unknown => return false,
                _ => {}
            }
            let types: &Vec<String> = &self.config.types;
            
            return types.contains(&lang.to_string().to_lowercase());
        }

        // 无扩展名则排除
        false
    }
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_walk_dir() {
        let reader = FileReader::new(Config::new());
        let files = reader.walk_dir(r"G:\Documents\GitHub\toukei").unwrap();

        assert!(files.len() > 0);
    }
}