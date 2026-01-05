import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { DashboardData, UsageData, BillingWindow, SessionSummary } from "../lib/types";

export function useUsageData() {
  const [data, setData] = useState<DashboardData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      setLoading(true);
      const [today, week, month, windows, sessions] = await Promise.all([
        invoke<UsageData>("get_usage_data", { timeRange: "today" }),
        invoke<UsageData>("get_usage_data", { timeRange: "week" }),
        invoke<UsageData>("get_usage_data", { timeRange: "month" }),
        invoke<BillingWindow[]>("get_billing_windows"),
        invoke<SessionSummary[]>("get_session_breakdown_cmd"),
      ]);

      setData({
        today,
        week,
        month,
        billingWindows: windows,
        sessions,
      });
      setError(null);
    } catch (e) {
      console.error("Failed to fetch usage data:", e);
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();

    // Listen for file changes from the backend watcher (real-time updates)
    const unlistenFilesChanged = listen("files-changed", () => {
      refresh();
    });

    return () => {
      unlistenFilesChanged.then((fn) => fn());
    };
  }, [refresh]);

  return { data, loading, error, refresh };
}
