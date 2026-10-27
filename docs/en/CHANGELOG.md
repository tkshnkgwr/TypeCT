# TypeCT Changelog (CHANGELOG)

All notable changes to `TypeCT` are documented in this file.

---

## [0.3.0] - 2026-10-27

### 📄 License Migration (GPL-3.0-or-later)

- **License Migration**:
  - Migrated distribution license from MIT to **GNU General Public License v3.0 or later (GPL-3.0-or-later)**.
  - Synchronized license metadata and badges across `Cargo.toml`, `README.md`, `README_JA.md`, `LICENSE`, and documentation.

---

## [0.2.1] - 2026-10-27

### 🔧 Improvements & Documentation Synchronization

- **Comprehensive Documentation Realignment**:
  - `ARCHITECTURE.md`: Added module responsibilities for `heatmap.rs`, `notify.rs`, and `startup.rs`; updated concurrency model with `MouseMove` event capture.
  - `TESTING.md`: Expanded test matrix to cover all 23 test cases (19 unit, 3 integration, 1 doc-test) with vertical table alignment.
  - `TEST_REPORT.md`: Updated execution logs to match latest test suite (23/23 PASS).
  - `README.md` / `README_JA.md`: Synchronized `daily_history` JSON schema example and restored full project documentation overview.
- **LICENSE Formatting**:
  - Removed UTF-8 BOM and restored trailing newline.
- **CI Build Error Fix**:
  - Resolved `float-literal-f32-fallback` compiler warning on modern Rust toolchains by explicitly typing float literals in `src/heatmap.rs` (`1.0_f32`, `4.0_f32`).

---

## [0.2.0] - 2026-09-15

### 🚀 New Features (Major Roadmap Integration)

- **Mouse Travel Distance Calculation**:
  - Captures `rdev::MouseMove` events, calculating cumulative Euclidean distance in meters/kilometers (96 DPI baseline).
  - Displays today's, yesterday's, and cumulative distance across summary cards.
- **Keyboard Heatmap Tab**:
  - Dedicated visual QWERTY keyboard heatmap tab.
  - Multi-tiered gradient heat coloring (Blue -> Cyan -> Green -> Yellow -> Red) mapped to key frequency.
  - Interactive hover tooltips showing individual count and ratio.
- **Milestone Goal Toast Alerts**:
  - Integrates `notify-rust` for native Windows toast alerts.
  - Configurable daily keystroke goal (default: 10,000 keys) with once-per-day celebration toast.
- **Dark / Light / System Theme Modes**:
  - Instant theme switching with `egui::Visuals` and preference persistence.
- **Windows Startup Auto-Launch Registration**:
  - Leverages `winreg` to configure `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
- **CSV Data Export**:
  - One-click export of comprehensive statistics (summaries, 24-hour distribution, key frequency, button clicks, daily summary ledger) to CSV.
- **Privacy-Preserving Daily Summary Ledger (`daily_history`)**:
  - Automatically records historical daily totals (keystrokes, mouse clicks, travel distance) without storing individual key sequences or sensitive strings.
  - Adds a "📅 Daily History (Recent)" grid to the Dashboard tab for effortless trend reviews.
- **3-Tab Modular UI**:
  - Reorganized interface with "📊 Dashboard", "⌨ Keyboard Heatmap", and "⚙ Settings" navigation.

---

## [0.1.0] - 2026-09-15

### 🎉 Initial Release (Modern Rewrite)

- **Complete Modern Rewrite**:
  - Re-engineered classic Borland C++ 5.5 "TypeCT" in Rust 2021 + `egui` native for Windows 11 (64-bit).
- **Single Instance Enforcement**:
  - Windows Named Mutex (`single-instance`) prevents duplicate processes.
- **Japanese Font Anti-Tofu System**:
  - Automatic dynamic loading of Windows system fonts (Yu Gothic, Meiryo, MS Gothic).
- **Zero-Lag Global Input Hook**:
  - Isolated background monitoring via `rdev` with non-blocking MPSC channels.
- **Privacy by Design**:
  - Increments counts per keycode only; zero string recording or telemetry.
- **Seamless Legacy Data Migration**:
  - Automatic detection and one-time import of legacy binary files (`data` and `data2`).
- **Dashboard & Analytics**:
  - Summary cards (Today, Yesterday, Cumulative, Days Recorded, Daily Average).
  - Hourly keystroke bar chart over past 24 hours (`egui_plot::BarChart`).
  - Key frequency ranking (Top 12) with progress bars.
- **Taskbar System Tray Resident**:
  - Minimize to notification tray on minimize/close.
  - Real-time dynamic count hover tooltips.
- **0% Idle CPU Usage**:
  - Halts UI repaint loop when minimized or in background.
- **Extreme Binary Optimization**:
  - Standalone binary with LTO, size optimization (`opt-level = "z"`), and symbol stripping.
- **Test Suite & CI**:
  - Comprehensive RustDoc and unit/integration test suite.
  - Automated GitHub Actions CI and Release pipelines.
