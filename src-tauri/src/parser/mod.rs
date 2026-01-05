pub mod types;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use chrono::{DateTime, Duration, Utc};
use walkdir::WalkDir;

use crate::pricing::calculate_cost;
use types::*;

pub fn get_claude_paths() -> Vec<PathBuf> {
    let home = dirs::home_dir().expect("Could not find home directory");
    let mut paths = Vec::new();

    let claude_path = home.join(".claude/projects");
    if claude_path.exists() {
        paths.push(claude_path);
    }

    let config_path = home.join(".config/claude/projects");
    if config_path.exists() {
        paths.push(config_path);
    }

    paths
}

pub fn find_jsonl_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for base_path in paths {
        for entry in WalkDir::new(base_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "jsonl") {
                files.push(path.to_path_buf());
            }
        }
    }

    files
}

pub fn parse_jsonl_file(path: &PathBuf) -> Vec<LogEntry> {
    let mut entries = Vec::new();

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return entries,
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line_content) = line {
            if let Ok(entry) = serde_json::from_str::<LogEntry>(&line_content) {
                if entry.message.usage.is_some() {
                    entries.push(entry);
                }
            }
        }
    }

    entries
}


pub fn aggregate_usage(entries: &[LogEntry]) -> UsageData {
    let mut total_cost = 0.0;
    let mut total_tokens = AggregatedTokens::default();
    let mut model_map: HashMap<String, ModelUsage> = HashMap::new();

    for entry in entries {
        if let Some(ref usage) = entry.message.usage {
            let model = entry
                .message
                .model
                .clone()
                .unwrap_or_else(|| "unknown".to_string());

            let cost = entry.cost_usd.unwrap_or_else(|| calculate_cost(&model, usage));
            total_cost += cost;

            total_tokens.input_tokens += usage.input_tokens;
            total_tokens.output_tokens += usage.output_tokens;
            total_tokens.cache_creation_input_tokens +=
                usage.cache_creation_input_tokens.unwrap_or(0);
            total_tokens.cache_read_input_tokens += usage.cache_read_input_tokens.unwrap_or(0);

            let model_usage = model_map.entry(model.clone()).or_insert(ModelUsage {
                model: model.clone(),
                input_tokens: 0,
                output_tokens: 0,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
                cost_usd: 0.0,
            });

            model_usage.input_tokens += usage.input_tokens;
            model_usage.output_tokens += usage.output_tokens;
            model_usage.cache_creation_input_tokens +=
                usage.cache_creation_input_tokens.unwrap_or(0);
            model_usage.cache_read_input_tokens += usage.cache_read_input_tokens.unwrap_or(0);
            model_usage.cost_usd += cost;
        }
    }

    let model_breakdown: Vec<ModelUsage> = model_map.into_values().collect();

    UsageData {
        total_cost_usd: total_cost,
        total_tokens,
        model_breakdown,
        last_updated: Utc::now().to_rfc3339(),
    }
}

pub fn calculate_billing_windows(entries: &[LogEntry]) -> Vec<BillingWindow> {
    if entries.is_empty() {
        return Vec::new();
    }

    let mut windows: Vec<BillingWindow> = Vec::new();
    let window_duration = Duration::hours(5);
    let now = Utc::now();

    let mut current_window_start: Option<DateTime<Utc>> = None;
    let mut window_entries: Vec<&LogEntry> = Vec::new();

    for entry in entries {
        if let Ok(dt) = DateTime::parse_from_rfc3339(&entry.timestamp) {
            let entry_time = dt.with_timezone(&Utc);

            match current_window_start {
                None => {
                    current_window_start = Some(entry_time);
                    window_entries.push(entry);
                }
                Some(start) => {
                    if entry_time - start <= window_duration {
                        window_entries.push(entry);
                    } else {
                        let end_time = start + window_duration;
                        let remaining = (end_time - now).num_minutes().max(0);
                        let is_active = now < end_time && now >= start;

                        let mut total_tokens = 0u64;
                        let mut total_cost = 0.0;

                        for e in &window_entries {
                            if let Some(ref usage) = e.message.usage {
                                total_tokens += usage.input_tokens
                                    + usage.output_tokens
                                    + usage.cache_creation_input_tokens.unwrap_or(0)
                                    + usage.cache_read_input_tokens.unwrap_or(0);

                                let model = e
                                    .message
                                    .model
                                    .clone()
                                    .unwrap_or_else(|| "unknown".to_string());
                                total_cost +=
                                    e.cost_usd.unwrap_or_else(|| calculate_cost(&model, usage));
                            }
                        }

                        windows.push(BillingWindow {
                            id: start.to_rfc3339(),
                            start_time: start.to_rfc3339(),
                            end_time: end_time.to_rfc3339(),
                            total_tokens,
                            cost_usd: total_cost,
                            remaining_minutes: remaining,
                            is_active,
                        });

                        current_window_start = Some(entry_time);
                        window_entries.clear();
                        window_entries.push(entry);
                    }
                }
            }
        }
    }

    if let Some(start) = current_window_start {
        if !window_entries.is_empty() {
            let end_time = start + window_duration;
            let remaining = (end_time - now).num_minutes().max(0);
            let is_active = now < end_time && now >= start;

            let mut total_tokens = 0u64;
            let mut total_cost = 0.0;

            for e in &window_entries {
                if let Some(ref usage) = e.message.usage {
                    total_tokens += usage.input_tokens
                        + usage.output_tokens
                        + usage.cache_creation_input_tokens.unwrap_or(0)
                        + usage.cache_read_input_tokens.unwrap_or(0);

                    let model = e
                        .message
                        .model
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string());
                    total_cost += e.cost_usd.unwrap_or_else(|| calculate_cost(&model, usage));
                }
            }

            windows.push(BillingWindow {
                id: start.to_rfc3339(),
                start_time: start.to_rfc3339(),
                end_time: end_time.to_rfc3339(),
                total_tokens,
                cost_usd: total_cost,
                remaining_minutes: remaining,
                is_active,
            });
        }
    }

    windows
}

pub fn get_session_breakdown(entries: &[LogEntry]) -> Vec<SessionSummary> {
    let mut session_map: HashMap<String, SessionSummary> = HashMap::new();

    for entry in entries {
        let session_id = entry
            .session_id
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let project_path = entry.cwd.clone().unwrap_or_else(|| "unknown".to_string());

        let model = entry
            .message
            .model
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        let cost = if let Some(ref usage) = entry.message.usage {
            entry
                .cost_usd
                .unwrap_or_else(|| calculate_cost(&model, usage))
        } else {
            0.0
        };

        let session = session_map
            .entry(session_id.clone())
            .or_insert(SessionSummary {
                session_id: session_id.clone(),
                project_path: project_path.clone(),
                start_time: entry.timestamp.clone(),
                end_time: entry.timestamp.clone(),
                message_count: 0,
                total_cost_usd: 0.0,
                models: Vec::new(),
            });

        session.end_time = entry.timestamp.clone();
        session.message_count += 1;
        session.total_cost_usd += cost;

        if !session.models.contains(&model) {
            session.models.push(model);
        }
    }

    let mut sessions: Vec<SessionSummary> = session_map.into_values().collect();
    sessions.sort_by(|a, b| b.end_time.cmp(&a.end_time));
    sessions
}
