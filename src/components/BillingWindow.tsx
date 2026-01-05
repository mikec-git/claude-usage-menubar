import type { BillingWindow as BillingWindowType } from "../lib/types";
import { formatCurrency, formatDuration, formatTime } from "../lib/formatters";

interface Props {
  windows: BillingWindowType[];
}

export default function BillingWindow({ windows }: Props) {
  const activeWindow = windows.find((w) => w.isActive);

  if (!activeWindow) {
    return null;
  }

  const progressPercent = Math.max(
    0,
    Math.min(100, ((300 - activeWindow.remainingMinutes) / 300) * 100)
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
          style={{ width: `${progressPercent}%` }}
        />
      </div>

      <div className="flex items-center justify-between text-xs text-amber-600 dark:text-amber-400">
        <span>Started {formatTime(activeWindow.startTime)}</span>
        <span className="font-medium">{formatCurrency(activeWindow.costUsd)}</span>
      </div>
    </div>
  );
}
