import { useUsageData } from "../hooks/useUsageData";
import { useAutoRefresh } from "../hooks/useAutoRefresh";
import DailySummary from "./DailySummary";
import MonthlySummary from "./MonthlySummary";
import ModelBreakdown from "./ModelBreakdown";
import BillingWindow from "./BillingWindow";
import SessionList from "./SessionList";

export default function Dashboard() {
  const { data, loading, error, refresh } = useUsageData();

  useAutoRefresh(refresh, 30000);

  if (loading && !data) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-neutral-500 dark:text-neutral-400">Loading...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4">
        <div className="text-red-500 text-sm">Error: {error}</div>
        <button
          onClick={refresh}
          className="mt-2 text-blue-500 hover:text-blue-600 text-sm"
        >
          Retry
        </button>
      </div>
    );
  }

  if (!data) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-neutral-500 dark:text-neutral-400">No data available</div>
      </div>
    );
  }

  return (
    <div className="p-3 space-y-3 max-h-[500px] overflow-y-auto">
      <header className="flex items-center justify-between pb-2 border-b border-neutral-200 dark:border-neutral-700">
        <h1 className="text-base font-semibold text-neutral-800 dark:text-neutral-100">
          Claude Usage
        </h1>
        <button
          onClick={refresh}
          className="text-xs text-neutral-500 hover:text-neutral-700 dark:text-neutral-400 dark:hover:text-neutral-200"
          title="Refresh"
        >
          {loading ? "Updating..." : "↻ Refresh"}
        </button>
      </header>

      <div className="grid grid-cols-2 gap-2">
        <DailySummary data={data.today} />
        <MonthlySummary data={data.month} />
      </div>

      {data.billingWindows.length > 0 && (
        <BillingWindow windows={data.billingWindows} />
      )}

      <ModelBreakdown models={data.today.modelBreakdown} />

      {data.sessions.length > 0 && (
        <SessionList sessions={data.sessions.slice(0, 5)} />
      )}
    </div>
  );
}
