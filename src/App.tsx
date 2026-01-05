import { useEffect } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import Dashboard from "./components/Dashboard";

function App() {
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        getCurrentWebviewWindow().hide();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  return (
    <div className="min-h-screen bg-white/95 dark:bg-neutral-900/95 backdrop-blur-xl rounded-xl overflow-hidden shadow-2xl">
      <Dashboard />
    </div>
  );
}

export default App;
