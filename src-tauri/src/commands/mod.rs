use chrono::{DateTime, Datelike, Duration, Local, NaiveDate};
use tauri::State;

use crate::cache::EntryCache;
use crate::parser::{
    aggregate_usage, calculate_billing_windows, get_session_breakdown,
    types::{BillingWindow, LogEntry, SessionSummary, UsageData},
};

/// Filter entries by time range
fn filter_entries_by_time_range(entries: &[LogEntry], time_range: &str) -> Vec<LogEntry> {
    let now = Local::now();
    let filter_date = match time_range {
        "today" => now.date_naive(),
        "week" => (now - Duration::days(7)).date_naive(),
        "month" => {
            NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap_or(now.date_naive())
        }
        _ => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
    };

    entries
        .iter()
        .filter(|entry| {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&entry.timestamp) {
                let entry_date = dt.with_timezone(&Local).date_naive();
                if time_range == "today" {
                    entry_date == filter_date
                } else {
                    entry_date >= filter_date
                }
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

#[tauri::command]
pub fn get_usage_data(time_range: String, cache: State<'_, EntryCache>) -> Result<UsageData, String> {
    let all_entries = cache.get_all_entries();
    let filtered = filter_entries_by_time_range(&all_entries, &time_range);
    Ok(aggregate_usage(&filtered))
}

#[tauri::command]
pub fn get_billing_windows(cache: State<'_, EntryCache>) -> Result<Vec<BillingWindow>, String> {
    let all_entries = cache.get_all_entries();
    let filtered = filter_entries_by_time_range(&all_entries, "today");
    Ok(calculate_billing_windows(&filtered))
}

#[tauri::command]
pub fn get_session_breakdown_cmd(cache: State<'_, EntryCache>) -> Result<Vec<SessionSummary>, String> {
    let all_entries = cache.get_all_entries();
    let filtered = filter_entries_by_time_range(&all_entries, "today");
    Ok(get_session_breakdown(&filtered))
}
