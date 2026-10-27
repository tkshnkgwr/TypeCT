# TypeCT Architecture Design (ARCHITECTURE)

This document details the internal architecture, thread communication, module layout, and power-saving design of `TypeCT`.

---

## 1. High-Level Architecture

`TypeCT` utilizes a decoupled two-thread architecture separating high-speed OS hook capture from GUI rendering.

```mermaid
graph TD
    OS[Windows OS] --> SingleInst{Single Instance Check}
    SingleInst -- Already Running --> SilentExit[Immediate Exit]
    SingleInst -- Primary Instance --> MainProc[Initialize Main]
    
    subgraph InputThread [Input Hook Thread (rdev)]
        RDevListen[rdev::listen Global Hook] --> Filter[map_rdev_key / map_mouse_button]
        Filter --> Sender[mpsc::Sender]
    end

    subgraph MainThread [GUI & Aggregation Thread (eframe)]
        Receiver[mpsc::Receiver] --> AppUpdate[TypeCtApp::update loop]
        AppUpdate --> Aggregation[TypeCtData: Aggregate Counts]
        AppUpdate --> RolloverCheck{Midnight Rollover Check}
        AppUpdate --> TrayUpdate[tray_icon::set_tooltip dynamic update]
        AppUpdate --> AutoSave{5 min elapsed or exit?}
        AutoSave -- Yes --> WriteDisk[Save typect.json]
        AppUpdate --> EguiRender[egui: Dashboard & Plot Render]
    end

    Sender -->|Async MPSC Channel| Receiver
    OS -->|Keyboard / Mouse Events| RDevListen
```

---

## 2. Source Code Modules

```
src/
├── lib.rs       # Library root & public re-exports
├── main.rs      # Binary entry point (Single instance, thread spawn, eframe startup)
├── legacy.rs    # Legacy binary (data / data2) parser (LegacyHeader)
├── model.rs     # Data structures (TypeCtData), rollover logic, JSON persistence, CSV export
├── input.rs     # Input messages (InputHookMessage), safe key mapping, distance calculation
├── font.rs      # Windows Japanese font discovery (setup_japanese_fonts)
├── tray.rs      # Embedded 32x32 RGBA tray icon generation (create_embedded_tray_icon)
├── heatmap.rs   # Keyboard layout definition & heatmap rendering
├── notify.rs    # Milestone goal toast notifications (notify-rust)
├── startup.rs   # Windows startup registry management (winreg)
└── app.rs       # GUI application implementation (TypeCtApp: 3 tabs)
```

### 2.1 Module Responsibilities
- **`legacy.rs`**: Parses 28-byte binary headers defensively against malformed files.
- **`model.rs`**: Maintains counters, daily summary ledger, rollover logic, JSON persistence, and CSV export.
- **`input.rs`**: Converts raw hardware events into sanitized display names and calculates Euclidean mouse travel distance.
- **`font.rs`**: Dynamically discovers and registers standard Windows CJK fonts to prevent "tofu".
- **`tray.rs`**: Renders a clean 32x32 RGBA system tray icon in memory without external `.ico` assets.
- **`heatmap.rs`**: Defines QWERTY keyboard layout and computes dynamic 4-tier heatmap gradient colors.
- **`notify.rs`**: Manages Windows milestone toast notifications when reaching the daily keystroke goal.
- **`startup.rs`**: Handles Windows registry (`HKCU\...\Run`) auto-start registration and deregistration.
- **`app.rs`**: Orchestrates event polling, tray click reactions, periodic saving, 3-tab layout, and UI rendering.

---

## 3. Concurrency & Event Flow

1. **Input Monitoring Thread**:
   - Spawns via `std::thread::spawn` and executes `rdev::listen`.
   - Listens for `KeyPress`, `ButtonPress`, and `MouseMove` events.
   - Pushes sanitized messages through `mpsc::Sender<InputHookMessage>`.
2. **Main GUI Thread**:
   - Runs `eframe::App::update`.
   - Drains all pending events via non-blocking `try_recv()` at the start of each frame.

---

## 4. Power Saving & 0% Idle CPU Design

For an always-running background utility, minimizing CPU cycles is paramount:

1. **Adaptive Repaint Timing**:
   - Active input: Triggers immediate redraw with `ctx.request_repaint()`.
   - Inactive: Throttles redraws with `ctx.request_repaint_after(Duration::from_millis(500))`.
2. **Suspended Repaint When Minimized**:
   - When hidden in the system tray (`is_window_visible == false`), the rendering loop completely halts.
   - Only lightweight background event consumption occurs, maintaining **0.0% CPU usage**.
