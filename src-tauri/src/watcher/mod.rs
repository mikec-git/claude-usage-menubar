use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventResult};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

use crate::cache::EntryCache;
use crate::parser::get_claude_paths;

const DEBOUNCE_DURATION_MS: u64 = 500;

pub fn start_file_watcher(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let paths = get_claude_paths();

        if paths.is_empty() {
            eprintln!("No Claude project paths found to watch");
            return;
        }

        let (tx, rx) = mpsc::channel::<Vec<PathBuf>>();

        let mut debouncer = match new_debouncer(
            Duration::from_millis(DEBOUNCE_DURATION_MS),
            move |result: DebounceEventResult| {
                if let Ok(events) = result {
                    let jsonl_paths: Vec<PathBuf> = events
                        .iter()
                        .filter(|e| e.path.extension().is_some_and(|ext| ext == "jsonl"))
                        .map(|e| e.path.clone())
                        .collect();

                    if !jsonl_paths.is_empty() {
                        let _ = tx.send(jsonl_paths);
                    }
                }
            },
        ) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to create file watcher: {:?}", e);
                return;
            }
        };

        for path in &paths {
            if let Err(e) = debouncer.watcher().watch(path, RecursiveMode::Recursive) {
                eprintln!("Failed to watch path {:?}: {:?}", path, e);
            } else {
                println!("Watching: {:?}", path);
            }
        }

        loop {
            if let Ok(changed_paths) = rx.recv() {
                // Refresh cache for changed files only
                let cache = app_handle.state::<EntryCache>();
                cache.invalidate_paths(&changed_paths);

                // Then notify frontend
                if let Some(window) = app_handle.get_webview_window("dashboard") {
                    let _ = window.emit("files-changed", ());
                }
            }
        }
    });
}
