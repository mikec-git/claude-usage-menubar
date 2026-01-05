use crate::parser::{
    aggregate_usage, calculate_billing_windows, get_session_breakdown, load_all_entries,
    types::{BillingWindow, SessionSummary, UsageData},
};

#[tauri::command]
pub fn get_usage_data(time_range: String) -> Result<UsageData, String> {
    let entries = load_all_entries(&time_range);
    Ok(aggregate_usage(&entries))
}

#[tauri::command]
pub fn get_billing_windows() -> Result<Vec<BillingWindow>, String> {
    let entries = load_all_entries("today");
    Ok(calculate_billing_windows(&entries))
}

#[tauri::command]
pub fn get_session_breakdown_cmd() -> Result<Vec<SessionSummary>, String> {
    let entries = load_all_entries("today");
    Ok(get_session_breakdown(&entries))
}
