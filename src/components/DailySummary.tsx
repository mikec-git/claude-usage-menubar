import type { UsageData } from "../lib/types";
import { formatCurrency, formatTokens } from "../lib/formatters";

interface Props {
  data: UsageData;
}

export default function DailySummary({ data }: Props) {
  const totalTokens =
    data.totalTokens.inputTokens +
    data.totalTokens.outputTokens +
    data.totalTokens.cacheCreationInputTokens +
    data.totalTokens.cacheReadInputTokens;

  return (
    <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-3">
      <div className="text-xs text-blue-600 dark:text-blue-400 font-medium mb-1">
        Today
      </div>
      <div className="text-xl font-bold text-blue-700 dark:text-blue-300">
        {formatCurrency(data.totalCostUsd)}
      </div>
      <div className="text-xs text-blue-500 dark:text-blue-400 mt-1">
        {formatTokens(totalTokens)} tokens
      </div>
    </div>
  );
}
