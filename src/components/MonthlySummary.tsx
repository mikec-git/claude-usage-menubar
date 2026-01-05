import type { UsageData } from "../lib/types";
import { formatCurrency, formatTokens } from "../lib/formatters";

interface Props {
  data: UsageData;
}

export default function MonthlySummary({ data }: Props) {
  const totalTokens =
    data.totalTokens.inputTokens +
    data.totalTokens.outputTokens +
    data.totalTokens.cacheCreationInputTokens +
    data.totalTokens.cacheReadInputTokens;

  return (
    <div className="bg-purple-50 dark:bg-purple-900/20 rounded-lg p-3">
      <div className="text-xs text-purple-600 dark:text-purple-400 font-medium mb-1">
        This Month
      </div>
      <div className="text-xl font-bold text-purple-700 dark:text-purple-300">
        {formatCurrency(data.totalCostUsd)}
      </div>
      <div className="text-xs text-purple-500 dark:text-purple-400 mt-1">
        {formatTokens(totalTokens)} tokens
      </div>
    </div>
  );
}
