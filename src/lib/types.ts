export interface TokenUsage {
  inputTokens: number;
  outputTokens: number;
  cacheCreationInputTokens: number;
  cacheReadInputTokens: number;
}

export interface ModelUsage extends TokenUsage {
  model: string;
  costUsd: number;
}

export interface UsageData {
  totalCostUsd: number;
  totalTokens: TokenUsage;
  modelBreakdown: ModelUsage[];
  lastUpdated: string;
}

export interface BillingWindow {
  id: string;
  startTime: string;
  endTime: string;
  totalTokens: number;
  costUsd: number;
  remainingMinutes: number;
  isActive: boolean;
}

export interface SessionSummary {
  sessionId: string;
  projectPath: string;
  startTime: string;
  endTime: string;
  messageCount: number;
  totalCostUsd: number;
  models: string[];
}

export interface DashboardData {
  today: UsageData;
  month: UsageData;
  billingWindows: BillingWindow[];
  sessions: SessionSummary[];
}
