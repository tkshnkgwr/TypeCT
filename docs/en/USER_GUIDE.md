# TypeCT User Guide (USER_GUIDE)

This guide explains how to install, configure, operate, and migrate data with `TypeCT`.

---

## 1. Installation & Launch

### 1.1 Installation
`TypeCT` is portable and self-contained in a single executable (`typect.exe`). No installer is required.
Place `typect.exe` in any folder of your choice (e.g., `%USERPROFILE%\Tools\TypeCT\`).

### 1.2 Launching
Double-click `typect.exe` to run.

> [!NOTE]
> `TypeCT` features single-instance enforcement. Launching a second instance silently exits immediately. If no window appears, check the taskbar notification area (system tray).

---

## 2. Using the Dashboard

When open, the main window presents key metrics:

```
+-------------------------------------------------------------------------+
| ⌨ TypeCT (タイプ数カウンター)               [💾 保存] [📥 トレイに格納] [❌ 終了] |
+-------------------------------------------------------------------------+
| [Today's Keys]       [Yesterday's Keys]    [Total Cumulative]   [Days/Average]      |
|  12,345 keys         10,200 keys           1,234,567 keys       120 days            |
|  Clicks: 3,456       Clicks: 2,800         Clicks: 450,000      Avg: 10,288 keys/day|
+-------------------------------------------------------------------------+
| Hourly Keystrokes (Past 24 Hours)                                       |
| [  ▂▃▅▆▇█▇▆▅▃▂   ] (Interactive Bar Chart)                              |
+-------------------------------------------------------------------------+
| Key Frequency Ranking (Top 12)                                          |
|  1. Space      [████████████████████████] 2,450 times                   |
|  2. E          [████████████████        ] 1,820 times                   |
|  3. Backspace  [████████████            ] 1,210 times                   |
|  ...                                                                    |
+-------------------------------------------------------------------------+
| Status: 計測中（常駐中）              Save Path: %USERPROFILE%\...\typect.json|
+-------------------------------------------------------------------------+
```

### 2.1 Header Buttons
- **💾 保存** (Save): Immediately flushes statistics to disk.
- **📥 トレイに格納** (Minimize to Tray): Hides the window to the taskbar tray while continuing monitoring.
- **❌ 終了** (Exit): Saves data and closes the application cleanly.

---

## 3. System Tray Operations

- **Restore Window**:
  - **Left-click** the "T" icon in the Windows taskbar notification area to restore and bring the window to the front.
- **Real-Time Hover Tooltip**:
  - Hovering your cursor over the tray icon displays current totals dynamically:
  ```text
  TypeCT - 今日: 6,183打 / 4,952クリック
  ```

---

## 4. Migrating from Classic "TypeCT"

If you have historical binary records from the original Borland C++ application:

1. Copy `data` (keystrokes) and `data2` (clicks) into the same folder as `typect.exe`.
2. Start `typect.exe`.
3. Historical records will automatically be detected and merged into `typect.json`.
4. A green indicator badge confirms: `※ 旧 typect (data / data2) からの累計データを引き継ぎ済み`.
5. Subsequent updates are saved cleanly into `typect.json`.

---

## 5. Persistence & Safeguards

Statistics are safely written to `typect.json`:
- Automatically every 5 minutes.
- Whenever minimized to the system tray.
- Whenever clicking `❌ 終了` or on OS shutdown.
