import { useState } from "react";
import type { ModelUsage } from "../lib/types";
import { formatCurrency, formatTokens, getModelDisplayName } from "../lib/formatters";

type TimeRange = "today" | "week";

interface Props {
  todayModels: ModelUsage[];
  weekModels: ModelUsage[];
}

function filterAndSortModels(models: ModelUsage[]): ModelUsage[] {
  return [...models]
    .filter((model) => {
      const totalTokens =
        model.inputTokens +
        model.outputTokens +
        model.cacheCreationInputTokens +
        model.cacheReadInputTokens;
      return totalTokens > 0;
    })
    .sort((a, b) => b.costUsd - a.costUsd);
}

export default function ModelBreakdown({ todayModels, weekModels }: Props) {
  const [timeRange, setTimeRange] = useState<TimeRange>("today");

  const todaySorted = filterAndSortModels(todayModels);
  const weekSorted = filterAndSortModels(weekModels);

  if (todaySorted.length === 0 && weekSorted.length === 0) {
    return null;
  }

  const sortedModels = timeRange === "today" ? todaySorted : weekSorted;

  return (
    <div className="bg-neutral-50 dark:bg-neutral-800/50 rounded-lg p-3">
      <div className="flex items-center justify-between mb-2">
        <div className="text-xs text-neutral-600 dark:text-neutral-400 font-medium">
          Model Breakdown
        </div>
        <div className="flex gap-1 text-xs">
          <button
            onClick={() => setTimeRange("today")}
            className={`px-2 py-0.5 rounded transition-colors ${
              timeRange === "today"
                ? "bg-neutral-200 dark:bg-neutral-700 text-neutral-800 dark:text-neutral-200"
                : "text-neutral-500 dark:text-neutral-400 hover:text-neutral-700 dark:hover:text-neutral-300"
            }`}
          >
            Today
          </button>
          <button
            onClick={() => setTimeRange("week")}
            className={`px-2 py-0.5 rounded transition-colors ${
              timeRange === "week"
                ? "bg-neutral-200 dark:bg-neutral-700 text-neutral-800 dark:text-neutral-200"
                : "text-neutral-500 dark:text-neutral-400 hover:text-neutral-700 dark:hover:text-neutral-300"
            }`}
          >
            This Week
          </button>
        </div>
      </div>
      <div className="space-y-2">
        {sortedModels.map((model) => {
          const totalTokens =
            model.inputTokens +
            model.outputTokens +
            model.cacheCreationInputTokens +
            model.cacheReadInputTokens;

          return (
            <div
              key={model.model}
              className="flex items-center justify-between text-sm"
            >
              <div className="flex items-center gap-2">
                <span className="text-neutral-700 dark:text-neutral-300">
                  {getModelDisplayName(model.model)}
                </span>
                <span className="text-xs text-neutral-400 dark:text-neutral-500">
                  {formatTokens(totalTokens)}
                </span>
              </div>
              <span className="font-medium text-neutral-800 dark:text-neutral-200">
                {formatCurrency(model.costUsd)}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
