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

export type PlanType = "pro" | "max100" | "max200";

export const PLAN_CONFIG: Record<PlanType, { name: string; tokenLimit: number }> = {
  pro: { name: "Pro ($20)", tokenLimit: 45_000_000 },
  max100: { name: "Max ($100)", tokenLimit: 225_000_000 },
  max200: { name: "Max ($200)", tokenLimit: 900_000_000 },
};
