import type { ModelUsage } from "../lib/types";
import { formatCurrency, formatTokens, getModelDisplayName } from "../lib/formatters";

interface Props {
  models: ModelUsage[];
}

export default function ModelBreakdown({ models }: Props) {
  if (models.length === 0) {
    return null;
  }

  const sortedModels = [...models].sort((a, b) => b.costUsd - a.costUsd);

  return (
    <div className="bg-neutral-50 dark:bg-neutral-800/50 rounded-lg p-3">
      <div className="text-xs text-neutral-600 dark:text-neutral-400 font-medium mb-2">
        Model Breakdown (Today)
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
