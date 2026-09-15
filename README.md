<div align="center">

# ⌨ TypeCT

**Ultra-lightweight, high-precision Keystroke, Mouse Click & Travel Distance Counter desktop app rebuilt for modern Windows 11 (64-bit native) in Rust + egui.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 2021](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%2011%20(64bit)-lightgrey.svg)](https://github.com/tkshnkgwr/TypeCT)
[![CI](https://img.shields.io/badge/CI-Passing-brightgreen.svg)](.github/workflows/ci.yml)

[English](./README.md) | [日本語](./README_JA.md)

</div>

---

## 📌 Overview

`TypeCT` is a modern, high-performance desktop counter app built with Rust and `egui`. It inherits the lightweight, unobtrusive, resident nature of the classic Windows freeware "TypeCT (タイプ数カウンター)" originally written in Borland C++ 5.5, while revamping it with modern design principles and safety guarantees for Windows 11 (64-bit).

It accurately captures keystrokes, mouse clicks, and **mouse travel distance (meters/kilometers)** in the background with zero input lag, rendering real-time statistics, hourly distributions, an interactive **Keyboard Heatmap**, and configurable milestone notifications. Furthermore, if legacy binary files (`data` / `data2`) from the old TypeCT exist in the directory, it automatically imports and migrates past cumulative data upon first launch.

---

## ✨ Key Features

- 🔒 **Single Instance Protection**: Enforces single-instance execution via Windows Named Mutex (`single-instance`). Secondary processes silently terminate immediately.
- 🔤 **Automatic Japanese Font Detection**: Automatically loads Windows OS standard fonts (Yu Gothic, Meiryo, MS Gothic) to completely eliminate "tofu" (□) mojibake.
- ⚡ **Zero-Lag Global Input Hook**: Captures global input asynchronously via `rdev` on a dedicated background thread, dispatching events to the GUI via non-blocking channels without typing delays.
- 🛡️ **Privacy & Security by Design**: Strictly NOT a keylogger. Does NOT record or store text, words, or passwords; only increments aggregated count totals per keycode.
- 🐭 **Real-Time Mouse Travel Distance**: Continuously calculates cursor travel distance in meters and kilometers (96 DPI baseline) for today, yesterday, and cumulative total.
- ⌨ **Visual Keyboard Heatmap**: Renders an interactive QWERTY layout colored by typing intensity (Blue -> Cyan -> Green -> Yellow -> Red).
- 🎯 **Milestone Goal Toast Alerts**: Native Windows toast notification upon reaching your daily keystroke target (default: 10,000 keys).
- 🎨 **Color Themes**: Toggle seamlessly between `💻 System`, `🌙 Dark`, and `☀️ Light` modes.
- 🚀 **Windows Startup Auto-Launch**: Configure Windows logon auto-start with a single checkbox via `winreg`.
- 📊 **CSV Data Export**: Export daily, hourly, key frequency, and mouse button statistics to CSV.
- 📂 **Seamless Legacy Data Migration**: Automatically parses legacy fixed-length binary dump files (`data`: keystrokes, `data2`: mouse clicks) and migrates historical counts to `typect.json`.
- 📥 **Taskbar Tray Resident & Dynamic Tooltips**: Minimizes to the Windows system tray on close or minimize. Hovering over the tray icon displays real-time counts and travel distance.
- 🍃 **Zero-Overhead Power Efficiency**: Halts rendering loop when minimized or in background, maintaining 0% idle CPU usage.
- 📦 **Compact Self-Contained Binary**: Employs LTO (Link-Time Optimization), size optimization (`opt-level = "z"`), and symbol stripping to generate a minimal standalone binary without external DLL dependencies.

---

## 🚀 Installation & Build

### Prerequisites
- 64-bit Windows 10 / 11
- Rust 1.80 or later (Cargo)

### Local Build
```powershell
# Clone repository
git clone https://github.com/tkshnkgwr/TypeCT.git
cd TypeCT

# Run in debug mode
cargo run

# Build optimized release binary
cargo build --release
```
The compiled binary will be placed at `target/release/typect.exe`.

---

## 💾 Migrating Legacy Data from Classic "TypeCT"

1. Copy your old data files into the same directory as `typect.exe`:
   - `data` : Legacy keystroke binary file
   - `data2` : Legacy mouse click binary file
2. Launch `typect.exe`.
3. Historical records will automatically be detected and merged into `typect.json`.

---

## 📁 Data Format (`typect.json`)

```json
{
  "total_keys": 543210,
  "today_keys": 4500,
  "yesterday_keys": 3800,
  "total_clicks": 125000,
  "today_clicks": 1200,
  "yesterday_clicks": 950,
  "total_mouse_distance_m": 1250.4,
  "today_mouse_distance_m": 350.2,
  "yesterday_mouse_distance_m": 420.8,
  "days_recorded": 120,
  "last_date": "2026-09-15",
  "hourly_keys": [0, 0, 0, 0, 0, 0, 15, 120, 850, 1200, ...],
  "key_counts": {
    "A": 340,
    "Enter": 520,
    "Space": 890
  },
  "mouse_counts": {
    "左クリック": 980,
    "右クリック": 220
  },
  "daily_goal_keys": 10000,
  "goal_notified": false,
  "theme": "System",
  "auto_start": false,
  "legacy_imported": true
}
```

---

## 🗂️ Architecture & Source Tree

```
src/
├── lib.rs       # Library root & public module exports
├── main.rs      # Application entry point (Single instance, input hook thread, eframe launch)
├── legacy.rs    # Legacy binary header parser (LegacyHeader)
├── model.rs     # Data structures (TypeCtData), rollover, JSON persistence, CSV export
├── input.rs     # Input messages (InputHookMessage), key mapping, distance calculation
├── font.rs      # Windows Japanese font discovery & registration (setup_japanese_fonts)
├── tray.rs      # Self-contained 32x32 RGBA taskbar tray icon generator
├── heatmap.rs   # Keyboard layout definition & heatmap rendering
├── notify.rs    # Windows milestone toast notification (notify-rust)
├── startup.rs   # Windows startup registry management (winreg)
└── app.rs       # GUI application implementation (TypeCtApp: 3 tabs)
tests/
└── integration_test.rs # Integration test suite (Legacy import -> Persistence -> Rollover -> CSV)
```

---

## 📚 Documentation

| Document                                          | Language | Description                                              |
| :------------------------------------------------ | :------- | :------------------------------------------------------- |
| [SPECIFICATION.md](docs/en/SPECIFICATION.md)     | English  | Full functional specifications, binary layouts, schemas  |
| [ARCHITECTURE.md](docs/en/ARCHITECTURE.md)       | English  | Architecture design, thread communication, power saving  |
| [USER_GUIDE.md](docs/en/USER_GUIDE.md)           | English  | Operating guide, tray controls, data migration steps     |
| [DEVELOPMENT.md](docs/en/DEVELOPMENT.md)         | English  | Developer guide, build instructions, test, RustDoc       |
| [TESTING.md](docs/en/TESTING.md)                 | English  | Testing strategy, unit tests, integration test suite     |
| [TEST_REPORT.md](docs/en/TEST_REPORT.md)         | English  | Latest test execution report                             |
| [SECURITY.md](docs/en/SECURITY.md)               | English  | Security & privacy policy (Keylogger-free guarantee)     |
| [CHANGELOG.md](docs/en/CHANGELOG.md)             | English  | Version release notes and changes                        |
| [CONTRIBUTING.md](docs/en/CONTRIBUTING.md)       | English  | Contribution guidelines and PR workflow                  |
| [TODO.md](docs/en/TODO.md)                       | English  | Feature roadmap and future plans                         |
| [FOOTPRINTS.md](docs/en/FOOTPRINTS.md)           | English  | Project origins and transition history                   |
| [日本語ドキュメント](docs/ja/)                    | 日本語   | Full Japanese documentation set                          |

---

## 📄 License

This software is released under the [MIT License](LICENSE).
