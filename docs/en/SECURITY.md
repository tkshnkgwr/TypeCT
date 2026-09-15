# TypeCT Security & Privacy Policy (SECURITY)

This document outlines the security policies, data privacy architecture, and explicit guarantees verifying that `TypeCT` is not a keylogger.

---

## 1. Guiding Principle: Total Local Privacy

`TypeCT` is strictly an ergonomics and productivity counter. Under no circumstances does it capture, store, or transmit sensitive information, credentials, or typed sentences.

---

## 2. Keylogger-Free Architectural Guarantee

1. **No Buffer for Typed Content**:
   - `map_rdev_key` in `src/input.rs` instantly translates key events into isolated token strings (e.g., `"A"`, `"Shift"`, `"Enter"`).
   - The application has zero memory buffers capable of accumulating sequences of keystrokes.
2. **Aggregated Counter Storage Only**:
   - The model in `src/model.rs` maintains strictly numeric counters (`HashMap<String, u64>`) representing how many times a particular key was tapped.
3. **Completely Air-Gapped (Zero Network Code)**:
   - `TypeCT` does not include networking libraries (no HTTP, WebSocket, or socket APIs).
   - Data is stored strictly on your local disk in `typect.json`.
4. **Transparent Open Source**:
   - The complete source code is available for auditing and reproducible compilation.

---

## 3. Reporting Vulnerabilities

To report a vulnerability or privacy concern, please open an issue or security advisory on the GitHub repository.
