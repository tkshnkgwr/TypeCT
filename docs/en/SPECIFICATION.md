# TypeCT Detailed Specifications (SPECIFICATION)

This document provides detailed technical specifications for `TypeCT`, a lightweight keystroke and mouse click counter desktop application built with Rust and `egui`.

---

## 1. System Overview

- **Name**: TypeCT (Keystroke & Mouse Click Counter)
- **Version**: 0.1.0
- **Target OS**: Windows 11 (64-bit native) / Windows 10 (64-bit)
- **Language**: Rust (2021 Edition)
- **Key Dependencies**:
  - GUI: `eframe` 0.29, `egui` 0.29, `egui_plot` 0.29
  - Input Monitoring: `rdev` 0.5
  - Serialization: `serde` 1.0, `serde_json` 1.0
  - Date & Time: `chrono` 0.4
  - Single Instance: `single-instance` 0.3
  - Taskbar System Tray: `tray-icon` 0.19
  - Windows API: `windows-sys` 0.59

---

## 2. Functional Requirements

### 2.1 Single Instance Enforcement
- Prevents multi-instance execution using a Windows Named Mutex (`TypeCT_Windows11_SingleInstance_App_Lock`).
- If an instance is already running, any newly launched process exits silently immediately with exit code 0 without creating windows.

### 2.2 Global Input Monitoring & Privacy Protection
- Runs `rdev::listen` on a dedicated background thread spawned via `std::thread::spawn`.
- Communicates asynchronously with the GUI thread using `std::sync::mpsc::channel`, causing zero latency or input lag for typing and gaming.
- **Privacy by Design**:
  - Strictly keylogger-free.
  - No text strings, sentences, or passwords are recorded or stored.
  - Increments count aggregates per key (e.g., `A`, `Enter`, `Shift`) and mouse buttons (e.g., `左クリック`, `右クリック`).

### 2.3 Legacy Data Migration
Automatically detects and imports legacy data files from the classic Borland C++ 5.5 "TypeCT (typect.exe)".

#### Legacy Binary Header Layout (`data`: keystrokes, `data2`: clicks)
First 28 bytes consisting of consecutive 4-byte little-endian `u32` integers:

| Offset | Type | Field Name        | Description                             |
| :----- | :--- | :---------------- | :-------------------------------------- |
| `0x00` | u32  | `days_recorded`   | Total recorded days                     |
| `0x04` | u32  | `last_year`       | Last recorded year                      |
| `0x08` | u32  | `last_month`      | Last recorded month (1..=12)            |
| `0x0C` | u32  | `last_day`        | Last recorded day (1..=31)              |
| `0x10` | u32  | `total_count`     | Cumulative total count                  |
| `0x14` | u32  | `today_count`     | Today's count                           |
| `0x18` | u32  | `yesterday_count` | Yesterday's count                       |

- Upon launch, if `data` or `data2` exists in the application directory and has not yet been imported into `typect.json`, the headers are parsed and cumulative historical totals are imported.

### 2.4 Date Rollover
- Automatically detects midnight transition (`last_date != current_date`):
  - `yesterday_keys = today_keys`
  - `yesterday_clicks = today_clicks`
  - `today_keys = 0`
  - `today_clicks = 0`
  - `hourly_keys = [0; 24]`
  - `days_recorded += 1`
  - `last_date = current_date`

### 2.5 Taskbar System Tray & Dynamic Tooltips
- Minimizing or clicking the window close button (`X`) minimizes the application to the Windows notification tray.
- Left-clicking the tray icon restores and focuses the window.
- Hovering over the tray icon dynamically displays real-time counts:
  ```text
  TypeCT - 今日: 6,183打 / 4,952クリック
  ```
- Tooltip updates are throttled to a minimum interval of 1 second to eliminate CPU overhead.

---

## 3. UI Layout & Dashboard

1. **Header Panel (`TopBottomPanel::top`)**:
   - Title: `⌨ TypeCT (タイプ数カウンター)`
   - Control buttons:
     - `💾 保存` (Save): Immediately flushes data to `typect.json`.
     - `📥 トレイに格納` (Tray): Hides window to system tray.
     - `❌ 終了` (Exit): Saves data and exits application cleanly.
2. **Central Dashboard Panel (`CentralPanel`)**:
   - **4 Summary Cards**:
     - Today's Keystrokes & Clicks
     - Yesterday's Keystrokes & Clicks
     - Cumulative Totals
     - Total Days Recorded & Daily Average
   - **Hourly Keystroke Chart**:
     - 24-hour bar chart rendered via `egui_plot::BarChart`.
   - **Top 12 Key Frequency Ranking**:
     - Progress bars visualizing keystroke distribution.
   - **Legacy Migration Badge**:
     - Displays green indicator if data was imported from legacy files.
3. **Status Bar (`TopBottomPanel::bottom`)**:
   - Displays monitoring status and full path of `typect.json`.

---

## 4. JSON Schema (`typect.json`)

| Field Name         | Type                  | Description                               |
| :----------------- | :-------------------- | :---------------------------------------- |
| `total_keys`       | `u64`                 | Cumulative keystrokes                     |
| `today_keys`       | `u64`                 | Today's keystrokes                        |
| `yesterday_keys`   | `u64`                 | Yesterday's keystrokes                    |
| `total_clicks`     | `u64`                 | Cumulative mouse clicks                   |
| `today_clicks`     | `u64`                 | Today's mouse clicks                      |
| `yesterday_clicks` | `u64`                 | Yesterday's mouse clicks                  |
| `days_recorded`    | `u32`                 | Total days recorded                       |
| `last_date`        | `String`              | Last recorded date (`YYYY-MM-DD`)         |
| `hourly_keys`      | `[u32; 24]`           | Keystroke counts across 24 hours (0..=23) |
| `key_counts`       | `HashMap<String,u64>` | Key frequency mapping                     |
| `mouse_counts`     | `HashMap<String,u64>` | Mouse button click mapping                |
| `legacy_imported`  | `bool`                | Flag indicating legacy migration status   |
