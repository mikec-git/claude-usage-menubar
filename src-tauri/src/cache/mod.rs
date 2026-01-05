use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;

use crate::parser::{find_jsonl_files, get_claude_paths, parse_jsonl_file, types::LogEntry};

#[derive(Debug, Clone)]
pub struct CachedFile {
    pub path: PathBuf,
    pub modified_time: SystemTime,
    pub entries: Vec<LogEntry>,
}

pub struct EntryCache {
    files: RwLock<HashMap<PathBuf, CachedFile>>,
}

impl Default for EntryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl EntryCache {
    pub fn new() -> Self {
        Self {
            files: RwLock::new(HashMap::new()),
        }
    }

    /// Initialize cache by scanning and parsing all JSONL files
    pub fn initialize(&self) {
        let paths = get_claude_paths();
        let files = find_jsonl_files(&paths);

        let mut cache = self.files.write().unwrap();
        cache.clear();

        for file_path in files {
            if let Ok(metadata) = fs::metadata(&file_path) {
                if let Ok(modified) = metadata.modified() {
                    let entries = parse_jsonl_file(&file_path);
                    cache.insert(
                        file_path.clone(),
                        CachedFile {
                            path: file_path,
                            modified_time: modified,
                            entries,
                        },
                    );
                }
            }
        }
    }

    /// Get all cached entries from all files
    pub fn get_all_entries(&self) -> Vec<LogEntry> {
        let cache = self.files.read().unwrap();
        cache
            .values()
            .flat_map(|cf| cf.entries.clone())
            .collect()
    }

    /// Invalidate and refresh specific files (called by watcher)
    pub fn invalidate_paths(&self, changed_paths: &[PathBuf]) {
        let mut cache = self.files.write().unwrap();

        for path in changed_paths {
            if path.exists() {
                if let Ok(metadata) = fs::metadata(path) {
                    if let Ok(modified) = metadata.modified() {
                        let entries = parse_jsonl_file(path);
                        cache.insert(
                            path.clone(),
                            CachedFile {
                                path: path.clone(),
                                modified_time: modified,
                                entries,
                            },
                        );
                    }
                }
            } else {
                // File was deleted
                cache.remove(path);
            }
        }
    }
}
