# TypeCT Development Guide (DEVELOPMENT)

This guide describes how to set up the development environment, build the code, run tests, and generate RustDoc for `TypeCT`.

---

## 1. Prerequisites

- **OS**: Windows 11 (64-bit) / Windows 10 (64-bit)
- **Rust**: 1.80 or later (MSVC toolchain recommended)
  ```powershell
  rustup default stable-x86_64-pc-windows-msvc
  rustup component add rustfmt clippy
  ```

---

## 2. Build Commands

### 2.1 Debug Run
```powershell
# Run with debugging
cargo run

# Fast type/syntax check
cargo check
```

### 2.2 Release Build
```powershell
# Build optimized binary
cargo build --release
```
Target binary: `target/release/typect.exe`

---

## 3. Running Tests

### 3.1 Unit, Integration, and Documentation Tests
```powershell
# Run the entire test suite
cargo test

# Run with verbose output
cargo test --verbose
```

---

## 4. Code Quality & Linting

Run formatting and linting checks prior to submitting contributions:

```powershell
# Verify formatting
cargo fmt --check

# Apply auto-formatting
cargo fmt

# Clippy static analysis
cargo clippy --all-targets -- -D warnings
```

---

## 5. Generating RustDoc

All public items in the codebase include doc comments with runnable doc-tests:

```powershell
# Generate docs and open in browser
cargo doc --no-deps --open
```
HTML documentation is generated at `target/doc/typect_lib/index.html`.
