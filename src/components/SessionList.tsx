import type { SessionSummary } from "../lib/types";
import { formatCurrency, formatTime } from "../lib/formatters";

interface Props {
  sessions: SessionSummary[];
}

export default function SessionList({ sessions }: Props) {
  if (sessions.length === 0) {
    return null;
  }

  return (
    <div className="bg-neutral-50 dark:bg-neutral-800/50 rounded-lg p-3">
      <div className="text-xs text-neutral-600 dark:text-neutral-400 font-medium mb-2">
        Recent Sessions
      </div>
      <div className="space-y-2">
        {sessions.map((session) => {
          const projectName = session.projectPath.split("/").pop() || "Unknown";

          return (
            <div
              key={session.sessionId}
              className="flex items-center justify-between text-sm"
            >
              <div className="flex-1 min-w-0">
                <div className="text-neutral-700 dark:text-neutral-300 truncate">
                  {projectName}
                </div>
                <div className="text-xs text-neutral-400 dark:text-neutral-500">
                  {formatTime(session.startTime)} · {session.messageCount} messages
                </div>
              </div>
              <span className="font-medium text-neutral-800 dark:text-neutral-200 ml-2">
                {formatCurrency(session.totalCostUsd)}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
