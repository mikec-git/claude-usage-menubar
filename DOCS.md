# Claude Usage Menu Bar App

A macOS menu bar application that tracks Claude Code usage and costs.

## Features

- **Menu Bar Integration**: Lives in the macOS menu bar, click to open dashboard
- **Real-time Tracking**: Reads Claude Code JSONL logs from `~/.claude/projects/`
- **Cost Calculation**: Automatic pricing for all Claude models (Opus, Sonnet, Haiku)
- **Dashboard Views**:
  - Today's cost and token usage
  - Monthly summary
  - Per-model breakdown
  - 5-hour billing window status
  - Recent sessions list
- **Auto-refresh**: Updates every 30 seconds

## Prerequisites

1. **Rust** (required for Tauri):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (v18+):
   ```bash
   # Using nvm
   nvm install 18
   ```

3. **Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```

## Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

## Building

```bash
# Build for production
npm run tauri build
```

The built application will be at:
- `src-tauri/target/release/bundle/macos/Claude Usage.app`
- `src-tauri/target/release/bundle/dmg/Claude Usage_0.1.0_universal.dmg`

## Project Structure

```
claude-usage-menubar/
├── src/                    # React frontend
│   ├── components/         # Dashboard UI components
│   ├── hooks/              # React hooks (data fetching, auto-refresh)
│   └── lib/                # Types and formatters
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC commands
│   │   ├── parser/         # JSONL parsing logic
│   │   └── pricing/        # Cost calculation
│   └── tauri.conf.json     # Tauri configuration
└── DOCS.md
```

## Data Source

Reads JSONL files from:
- `~/.claude/projects/**/*.jsonl`
- `~/.config/claude/projects/**/*.jsonl`

## Model Pricing (per million tokens)

| Model | Input | Output | Cache Create | Cache Read |
|-------|-------|--------|--------------|------------|
| Opus 4.5 | $5.00 | $25.00 | $6.25 | $0.50 |
| Sonnet 4.5 | $3.00 | $15.00 | $3.75 | $0.30 |
| Haiku 4.5 | $1.00 | $5.00 | $1.25 | $0.10 |

## Credits

Inspired by [ccusage](https://github.com/ryoppippi/ccusage) - CLI tool for Claude Code usage analysis.
