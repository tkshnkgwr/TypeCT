# TypeCT Feature Roadmap (TODO)

This document tracks upcoming feature proposals, ideas, and completed milestones for `TypeCT`.

---

## 📌 Feature Roadmap & Proposals

| Priority | Feature                           | Category         | Description                                                  |
| :------- | :-------------------------------- | :--------------- | :----------------------------------------------------------- |
| High     | Real-time Typing Speed (KPM/WPM)  | Analytics        | Real-time keystroke speed (keys/min) gauge & daily peak speed|
| High     | GitHub-style Activity Calendar    | Visualization    | Year/month contribution-style heatmap grid to review days    |
| High     | Automated Backup Rotation         | Reliability      | Automated backup rotation to `backups/` at midnight rollover |
| Medium   | Per-App / Process Breakdown       | Productivity     | Typing distribution breakdown by foreground application window|
| Medium   | Ergonomic Break / Pomodoro Alert  | Healthcare       | Reminders to rest hands after prolonged continuous typing    |
| Medium   | Pause Tracking Hotkey             | Privacy & Gaming | Global shortcut (`Ctrl+Shift+P`) to temporarily pause count  |
| Medium   | Social Summary Card Export        | Sharing & Output | Export weekly achievements as a shareable PNG card image     |
| Low      | Mechanical / Typewriter Sounds    | Customization    | Optional retro typewriter or mechanical switch audio feedback|
| Low      | E2E Encrypted Cloud Backup        | Cloud Sync       | Optional end-to-end encrypted multi-workstation sync         |

---

## 📋 Completed Milestones

| Version    | Feature                           | Description                                                  |
| :--------- | :-------------------------------- | :----------------------------------------------------------- |
| **v0.2.0** | Mouse Travel Distance             | Real-time calculation of cursor distance in meters/kilometers|
| **v0.2.0** | Keyboard Heatmap Overlay          | Visual QWERTY heatmap tab illustrating key frequency         |
| **v0.2.0** | Milestone Toast Alerts            | Toast notification upon reaching daily keystroke goal        |
| **v0.2.0** | Dark / Light Theme Toggle         | Theme switching between System, Dark, and Light modes        |
| **v0.2.0** | Windows Startup Registration      | One-click configuration to launch automatically at logon     |
| **v0.2.0** | CSV / TSV Export                  | Export daily/hourly/key-level statistics to CSV files        |
| **v0.1.0** | Single Instance Enforcement       | Windows Named Mutex preventing multi-instance launches       |
| **v0.1.0** | Japanese Font Anti-Tofu System    | Automatic detection and registration of Windows system fonts |
| **v0.1.0** | Zero-Lag Global Input Hook        | Non-blocking input monitoring via `rdev` background thread   |
| **v0.1.0** | Seamless Legacy Migration         | One-time auto-import from classic `data` / `data2` binaries   |
| **v0.1.0** | Taskbar Tray Resident & Tooltips  | Minimize to notification tray with dynamic hover metrics     |
| **v0.1.0** | 0% Idle CPU Power Saving          | Suspended redraw loop when minimized or in background        |
