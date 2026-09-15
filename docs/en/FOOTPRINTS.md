# TypeCT Heritage & Transition History (FOOTPRINTS)

This chronicle documents the origin of the classic Windows freeware "TypeCT" and its revival in modern Rust.

---

## 1. Origins: The Golden Era of Borland C++ 5.5

During the late 1990s and 2000s across Windows 95, 98, 2000, and XP, programmers and authors sought tools to track ergonomics and productivity. "TypeCT (タイプ数カウンター)" by original author was one such beloved utility.

Compiled with Borland C++ 5.5, it was featherweight and reliable. It quietly lived in the system tray, dumping data into binary files (`data` and `data2`) as continuous 4-byte integers. For countless users, it recorded millions of keystrokes over decades.

---

## 2. The Windows 11 Shift

Decades later, modern Windows 11 (64-bit) brought high-DPI displays, new security constraints, and refreshed notification trays:
- Classic 32-bit binaries suffered from DPI blurriness and UI scaling artifacts.
- Outdated binary dumps lacked compatibility with contemporary data science workflows.
- Yet users held immense attachment to years of historical typing data stored in their `data` files.

The goal was clear: rebuild TypeCT with uncompromising modern engineering while preserving every single historical keystroke.

---

## 3. Resurrection with Rust + egui

Rust was chosen as the modern foundation:
- **Memory Safety**: Bulletproof long-term resident execution without memory leaks or crashes.
- **Immediate-Mode UI (`egui`)**: Blazing fast, GPU-accelerated, and natively crisp across high-DPI displays.
- **Zero Input Lag**: Background hook integration powered by `rdev`.
- **Honoring Legacy Data**: Defensive binary parsers extract every byte from legacy `data` / `data2` files seamlessly into modern JSON.

With deep respect for the classic era, TypeCT is ready for the decades ahead.
