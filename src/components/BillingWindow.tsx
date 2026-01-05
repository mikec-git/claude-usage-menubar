import { useState, useEffect } from "react";
import type { BillingWindow as BillingWindowType, PlanType } from "../lib/types";
import { PLAN_CONFIG } from "../lib/types";
import { formatCurrency, formatDuration, formatTime, formatTokens } from "../lib/formatters";

const PLAN_STORAGE_KEY = "claude-usage-plan";

interface Props {
  windows: BillingWindowType[];
}

export default function BillingWindow({ windows }: Props) {
  const [plan, setPlan] = useState<PlanType>("max200");

  useEffect(() => {
    const stored = localStorage.getItem(PLAN_STORAGE_KEY) as PlanType | null;
    if (stored && stored in PLAN_CONFIG) {
      setPlan(stored);
    }
  }, []);

  const handlePlanChange = (newPlan: PlanType) => {
    setPlan(newPlan);
    localStorage.setItem(PLAN_STORAGE_KEY, newPlan);
  };

  const activeWindow = windows.find((w) => w.isActive);

  if (!activeWindow) {
    return null;
  }

  const tokenLimit = PLAN_CONFIG[plan].tokenLimit;

  const timeProgressPercent = Math.max(
    0,
    Math.min(100, ((300 - activeWindow.remainingMinutes) / 300) * 100)
  );

  const tokenProgressPercent = Math.max(
    0,
    Math.min(100, (activeWindow.totalTokens / tokenLimit) * 100)
  );

  return (
    <div className="bg-amber-50 dark:bg-amber-900/20 rounded-lg p-3">
      <div className="flex items-center justify-between mb-2">
        <div className="text-xs text-amber-600 dark:text-amber-400 font-medium">
          5-Hour Billing Window
        </div>
        <div className="text-xs text-amber-500 dark:text-amber-400">
          {formatDuration(activeWindow.remainingMinutes)} remaining
        </div>
      </div>

      <div className="w-full bg-amber-200 dark:bg-amber-800 rounded-full h-2 mb-2">
        <div
          className="bg-amber-500 dark:bg-amber-400 h-2 rounded-full transition-all duration-300"
          style={{ width: `${timeProgressPercent}%` }}
        />
      </div>

      <div className="flex items-center justify-between text-xs text-amber-600 dark:text-amber-400 mb-2">
        <span>Started {formatTime(activeWindow.startTime)}</span>
        <span className="font-medium">{formatCurrency(activeWindow.costUsd)}</span>
      </div>

      <div className="flex items-center justify-between text-xs text-amber-600 dark:text-amber-400 mb-2">
        <span>{formatTokens(activeWindow.totalTokens)} tokens</span>
        <span className="font-medium">{tokenProgressPercent.toFixed(1)}% of limit</span>
      </div>

      <div className="flex items-center justify-between text-xs pt-2 border-t border-amber-200 dark:border-amber-700">
        <span className="text-amber-600 dark:text-amber-400">Plan:</span>
        <select
          value={plan}
          onChange={(e) => handlePlanChange(e.target.value as PlanType)}
          className="text-xs bg-amber-100 dark:bg-amber-800 text-amber-700 dark:text-amber-300 rounded px-1.5 py-0.5 border border-amber-300 dark:border-amber-600 cursor-pointer"
        >
          {(Object.keys(PLAN_CONFIG) as PlanType[]).map((key) => (
            <option key={key} value={key}>
              {PLAN_CONFIG[key].name}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}
