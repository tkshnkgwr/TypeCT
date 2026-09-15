# TypeCT Changelog (CHANGELOG)

All notable changes to `TypeCT` are documented in this file.

---

## [0.1.0] - 2026-09-15

### 🎉 Initial Release (Modern Rewrite)

- **Complete Modern Rewrite**:
  - Re-engineered classic Borland C++ 5.5 "TypeCT (typect.exe)" in Rust 2021 + `egui` native for Windows 11 (64-bit).
- **Single Instance Enforcement**:
  - Windows Named Mutex (`single-instance`) prevents duplicate instances.
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
  - Comprehensive RustDoc and 16 unit, integration, and doc-tests.
  - Automated GitHub Actions CI and Release pipelines.
