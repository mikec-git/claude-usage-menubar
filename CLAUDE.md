# Claude Usage Menu Bar

macOS menu bar app for tracking Claude Code API usage and costs. Built with Tauri 2.0, React 18, TypeScript, and Tailwind CSS.

## Development

```bash
npm install              # Install dependencies
npm run tauri dev        # Run app in development mode
npm run tauri build      # Build production app (DMG + .app)
```

## Architecture

### Frontend (`src/`)
- **components/**: Dashboard UI components (DailySummary, MonthlySummary, etc.)
- **hooks/**: `useUsageData` (IPC data fetching), `useAutoRefresh` (30s intervals)
- **lib/**: Types (`types.ts`), formatters (`formatters.ts`)

### Backend (`src-tauri/src/`)
- **commands/**: Tauri IPC command handlers
- **parser/**: JSONL file parsing and data aggregation
- **pricing/**: Cost calculation per model

## Code Conventions

### TypeScript/React
- Strict mode enabled
- Tailwind for all styling (no CSS modules or inline styles)
- Small, focused components with clear props interfaces
- Custom hooks for data fetching and side effects
- IPC via `@tauri-apps/api/core.invoke()`

### Rust
- snake_case for Rust, camelCase for JSON (Serde `#[serde(rename = "fieldName")]`)
- `Result<T, String>` for Tauri command error handling
- Modules: commands, parser, pricing

## Data Source

Reads JSONL logs from:
- `~/.claude/projects/**/*.jsonl`
- `~/.config/claude/projects/**/*.jsonl`

## Key Files

| File | Purpose |
|------|---------|
| `src/lib/types.ts` | Frontend TypeScript interfaces |
| `src-tauri/src/parser/types.rs` | Backend Rust types (mirrors frontend) |
| `src-tauri/src/pricing/mod.rs` | Model pricing rates and cost calculation |
| `src-tauri/tauri.conf.json` | Tauri app configuration |
