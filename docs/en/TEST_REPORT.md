# TypeCT Test Execution Report (TEST_REPORT)

This report summarizes automated verification, unit/integration test results, and static analysis checks for `TypeCT`.

---

## 1. Environment & Target

- **OS**: Windows 11 (64-bit)
- **Rust Toolchain**: rustc 1.80+ (stable-x86_64-pc-windows-msvc)
- **Report Date**: 2026-09-15
- **Target Version**: v0.1.0

---

## 2. Summary Verdict

| Verification Target       | Command                                | Status   | Details                        |
| :------------------------ | :------------------------------------- | :------- | :----------------------------- |
| Unit Tests                | `cargo test --lib`                     | **PASS** | 12 passed / 0 failed           |
| Integration Tests         | `cargo test --test integration_test`   | **PASS** | 2 passed / 0 failed            |
| Doc-Tests                 | `cargo test --doc`                     | **PASS** | 2 passed / 0 failed            |
| Static Analysis (Clippy)  | `cargo clippy --all-targets -- -D ...` | **PASS** | 0 warnings, 0 errors           |
| Code Formatting (rustfmt) | `cargo fmt --check`                    | **PASS** | Clean formatting               |
| Documentation Build       | `cargo doc --no-deps`                  | **PASS** | HTML built with zero warnings  |

**Overall Status: ALL TESTS PASSED**

---

## 3. Detailed Results

```text
running 12 tests
test input::tests::test_map_mouse_buttons ... ok
test input::tests::test_map_rdev_keys ... ok
test legacy::tests::test_parse_exact_28_bytes ... ok
test legacy::tests::test_parse_valid_legacy_header ... ok
test model::tests::test_date_rollover ... ok
test legacy::tests::test_parse_insufficient_length ... ok
test tray::tests::test_generate_tray_icon_rgba ... ok
test model::tests::test_try_import_legacy ... ok
test model::tests::test_default_typect_data ... ok
test model::tests::test_average_daily_keys ... ok
test model::tests::test_record_key_and_click ... ok
test model::tests::test_save_and_load_json ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 2 tests
test test_legacy_header_parser_edge_cases ... ok
test test_full_lifecycle_with_legacy_import_and_persistence ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc-tests typect_lib
running 2 tests
test src\legacy.rs - legacy::LegacyHeader::parse (line 51) ... ok
test src\model.rs - model::TypeCtData::average_daily_keys (line 73) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All 16 tests executed cleanly with zero failures.
