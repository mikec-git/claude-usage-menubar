use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    pub cwd: Option<String>,
    pub message: Message,
    #[serde(rename = "costUSD")]
    pub cost_usd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Option<String>,
    pub model: Option<String>,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    #[serde(default)]
    pub input_tokens: u64,
    #[serde(default)]
    pub output_tokens: u64,
    #[serde(default)]
    pub cache_creation_input_tokens: Option<u64>,
    #[serde(default)]
    pub cache_read_input_tokens: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AggregatedTokens {
    #[serde(rename = "inputTokens")]
    pub input_tokens: u64,
    #[serde(rename = "outputTokens")]
    pub output_tokens: u64,
    #[serde(rename = "cacheCreationInputTokens")]
    pub cache_creation_input_tokens: u64,
    #[serde(rename = "cacheReadInputTokens")]
    pub cache_read_input_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsage {
    pub model: String,
    #[serde(rename = "inputTokens")]
    pub input_tokens: u64,
    #[serde(rename = "outputTokens")]
    pub output_tokens: u64,
    #[serde(rename = "cacheCreationInputTokens")]
    pub cache_creation_input_tokens: u64,
    #[serde(rename = "cacheReadInputTokens")]
    pub cache_read_input_tokens: u64,
    #[serde(rename = "costUsd")]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    #[serde(rename = "totalCostUsd")]
    pub total_cost_usd: f64,
    #[serde(rename = "totalTokens")]
    pub total_tokens: AggregatedTokens,
    #[serde(rename = "modelBreakdown")]
    pub model_breakdown: Vec<ModelUsage>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingWindow {
    pub id: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "totalTokens")]
    pub total_tokens: u64,
    #[serde(rename = "costUsd")]
    pub cost_usd: f64,
    #[serde(rename = "remainingMinutes")]
    pub remaining_minutes: i64,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "projectPath")]
    pub project_path: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "messageCount")]
    pub message_count: u32,
    #[serde(rename = "totalCostUsd")]
    pub total_cost_usd: f64,
    pub models: Vec<String>,
}
