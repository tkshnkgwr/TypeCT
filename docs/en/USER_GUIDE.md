# TypeCT User Guide (USER_GUIDE)

This user guide explains how to install, operate, and utilize all features of `TypeCT`, including the keyboard heatmap, goal notifications, startup registration, and CSV export.

---

## 1. Installation & Launch

### 1.1 Installation
`TypeCT` is fully portable in a standalone executable (`typect.exe`). Place `typect.exe` into any folder of your choice (e.g., `%USERPROFILE%\Tools\TypeCT\`).

### 1.2 Launching
Double-click `typect.exe`. With single-instance protection, secondary launches silently terminate. Check the Windows taskbar notification area (system tray) if the window is hidden.

---

## 2. Navigating the 3 Tabs

Switch between views using the tab selector in the header panel:

### 2.1 📊 Dashboard
- **Summary Cards**:
  - Today's Keystrokes, Clicks, and **Mouse Travel Distance** (e.g., `350.2 m` / `1.45 km`).
  - Yesterday's metrics.
  - Cumulative totals.
  - Total days, daily average, and goal completion percentage.
- **Daily Goal Progress Bar**:
  - Real-time animated progress toward your daily target.
- **24-Hour Keystroke Distribution**:
  - Interactive bar chart showing peak activity periods.
- **Top 12 Key Frequency Ranking**:
  - Top keys displayed with proportional progress bars.
- **Recent Daily History (Last 14 Days)**:
  - Table overview of historical daily totals (keystrokes, clicks, travel distance).
  - ※ **Privacy Preserved**: Individual key breakdowns are limited to today and yesterday. Daily history only retains aggregated numbers, preventing keylogger risks.


### 2.2 ⌨ Keyboard Heatmap
- Renders an interactive QWERTY layout colored by typing intensity.
- High-frequency keys glow in **Warm Colors (Red / Yellow)**; low-frequency keys in **Cool Colors (Blue / Cyan)**.
- Hover over any key to view exact keystroke count and relative share.

### 2.3 ⚙ Settings
- **Windows Startup Auto-Launch**:
  - Check the box to launch TypeCT automatically when logging into Windows.
- **Daily Keystroke Goal**:
  - Adjust target between 1,000 and 50,000 keys. Native toast notifications celebrate milestone completion.
- **Theme Selection**:
  - Choose between `💻 System`, `🌙 Dark`, and `☀️ Light` modes.
- **CSV Data Export**:
  - Click to export daily, hourly, key-level, and mouse statistics into a CSV file.

---

## 3. Header Shortcuts

- **📊 CSV**: Instantly export statistics from any tab.
- **🌙 / ☀️ / 💻**: Quickly toggle color themes.
- **💾 Save**: Immediately flush data to `typect.json`.
- **📥 Tray**: Hide window to system tray.
- **❌ Exit**: Save and cleanly close the application.

---

## 4. System Tray Resident Operations

- **Restore**: Left-click the "T" icon in the taskbar notification area.
- **Live Tooltip**: Hovering displays keys, clicks, and mouse travel distance in real time:
  ```text
  TypeCT - 今日: 6,183打 / 4,952クリック / 450.2 m
  ```

---

## 5. Migrating from Classic "TypeCT"

1. Copy legacy `data` (keystrokes) and `data2` (clicks) into the same folder as `typect.exe`.
2. Launch `typect.exe`.
3. Historical records are automatically migrated into modern `typect.json`.
