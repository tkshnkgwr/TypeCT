# TypeCT Detailed Specifications (SPECIFICATION)

This document provides detailed technical specifications for `TypeCT`, an ergonomics and productivity counter application built with Rust and `egui`.

---

## 1. System Overview

- **Name**: TypeCT (Keystroke, Mouse Click & Travel Counter)
- **Version**: 0.3.0
- **Target OS**: Windows 11 (64-bit native) / Windows 10 (64-bit)
- **Language**: Rust (2021 Edition)
- **Key Dependencies**:
  - GUI: `eframe` 0.29, `egui` 0.29, `egui_plot` 0.29
  - Input Hook: `rdev` 0.5
  - Serialization: `serde` 1.0, `serde_json` 1.0
  - Date & Time: `chrono` 0.4
  - Single Instance: `single-instance` 0.3
  - Taskbar System Tray: `tray-icon` 0.19
  - Windows Toast Notifications: `notify-rust` 4.18
  - Windows Registry: `winreg` 0.52
  - Windows API: `windows-sys` 0.59

---

## 2. Functional Requirements

### 2.1 Single Instance Enforcement
- Prevents multi-instance execution using a Windows Named Mutex (`TypeCT_Windows11_SingleInstance_App_Lock`).
- Subsequent instances exit silently immediately with exit code 0.

### 2.2 Global Input Monitoring & Privacy Protection
- Runs `rdev::listen` asynchronously on a dedicated thread, forwarding events via non-blocking channels.
- Strictly keylogger-free. Does not buffer or store textual strings, words, or passwords.

### 2.3 Mouse Travel Distance Measurement
- Captures `rdev::EventType::MouseMove { x, y }` events.
- Calculates Euclidean pixel distance and converts to meters (96 DPI baseline: 1 px ≈ 0.0002645833 m).
- Displays today's, yesterday's, and cumulative distance formatted cleanly in meters or kilometers.

### 2.4 Visual Keyboard Heatmap
- Renders an interactive QWERTY keyboard layout across 6 rows.
- Colors keys along a 4-tier gradient (Blue -> Cyan -> Green -> Yellow -> Red) proportional to relative key frequency.
- Hovering reveals key name, count, and percentage share.

### 2.5 Milestone Goal Toast Alerts
- Configurable daily keystroke target (default: 10,000 keys).
- Dispatches a native Windows toast notification upon milestone completion.
- Throttled to once per day, auto-resetting upon midnight rollover.

### 2.6 Color Theme Modes
- Instant theme switching between `System` (OS-follow), `Dark`, and `Light`. Preferences are persisted automatically.

### 2.7 Windows Startup Registration
- Toggleable registration in `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` managed via `winreg`.

### 2.8 CSV Data Export
- Exports comprehensive statistics (summaries, 24-hour distribution, key ranking, mouse buttons) to CSV.

### 2.9 Legacy Migration & Rollover
- Automatically imports legacy Borland C++ binary dumps (`data` and `data2`) on first run.
- Automatically rolls over counts at midnight.

---

## 3. UI Layout & Navigation

1. **Top Header Panel**:
   - Title & status
   - Shortcut controls: `📊 CSV`, `🌙/☀️/💻 Theme`, `💾 Save`, `📥 Tray`, `❌ Exit`
   - Navigation tabs: `📊 Dashboard`, `⌨ Keyboard Heatmap`, `⚙ Settings`
2. **Central Panel**:
   - **Dashboard**: 4 summary cards (keys, clicks, mouse distance, target ratio), target progress bar, 24-hour bar chart, top 12 key rankings, and recent daily history table.
   - **Keyboard Heatmap**: Full interactive QWERTY heatmap.
   - **Settings**: Startup toggle, goal slider (1,000..50,000), theme selector, CSV export button.
3. **Bottom Status Bar**:
   - Status message and active JSON path.

---

## 4. JSON Schema (`typect.json`)

| Field                        | Type                            | Description                                      |
| :--------------------------- | :------------------------------ | :----------------------------------------------- |
| `total_keys`                 | `u64`                           | Cumulative keystrokes                            |
| `today_keys`                 | `u64`                           | Today's keystrokes                               |
| `yesterday_keys`             | `u64`                           | Yesterday's keystrokes                           |
| `total_clicks`               | `u64`                           | Cumulative mouse clicks                          |
| `today_clicks`               | `u64`                           | Today's mouse clicks                             |
| `yesterday_clicks`           | `u64`                           | Yesterday's mouse clicks                         |
| `total_mouse_distance_m`     | `f64`                           | Cumulative mouse travel distance (meters)        |
| `today_mouse_distance_m`     | `f64`                           | Today's mouse travel distance (meters)           |
| `yesterday_mouse_distance_m` | `f64`                           | Yesterday's mouse travel distance (meters)       |
| `days_recorded`              | `u32`                           | Total days recorded                              |
| `last_date`                  | `String`                        | Last recorded date (`YYYY-MM-DD`)                |
| `hourly_keys`                | `[u32; 24]`                     | Keystroke counts across 24 hours (0..=23)        |
| `key_counts`                 | `HashMap<String,u64>`           | Key frequency mapping                            |
| `mouse_counts`               | `HashMap<String,u64>`           | Mouse button click mapping                       |
| `daily_goal_keys`            | `u64`                           | Daily keystroke goal (default: 10,000)           |
| `goal_notified`              | `bool`                          | Flag indicating if today's goal fired            |
| `theme`                      | `ThemeMode`                     | Active theme (`System`, `Dark`, `Light`)         |
| `auto_start`                 | `bool`                          | Windows startup registration state               |
| `daily_history`              | `BTreeMap<String,DailySummary>` | Historical daily summary ledger (privacy-safe)   |
| `legacy_imported`            | `bool`                          | Flag indicating legacy migration status          |
