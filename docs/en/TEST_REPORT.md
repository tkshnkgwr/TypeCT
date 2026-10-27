# TypeCT Test Execution Report (TEST_REPORT)

This report summarizes automated verification, unit/integration test results, and static analysis checks for `TypeCT` v0.2.0.

---

## 1. Environment & Target

- **OS**: Windows 11 (64-bit)
- **Rust Toolchain**: rustc 1.80+ (stable-x86_64-pc-windows-msvc)
- **Report Date**: 2026-10-27
- **Target Version**: v0.3.0

---

## 2. Summary Verdict

| Verification Target       | Command                                | Status   | Details                         |
| :------------------------ | :------------------------------------- | :------- | :------------------------------ |
| Unit Tests                | `cargo test --lib`                     | **PASS** | 19 passed / 0 failed            |
| Integration Tests         | `cargo test --test integration_test`   | **PASS** | 3 passed / 0 failed             |
| Doc-Tests                 | `cargo test --doc`                     | **PASS** | 1 passed / 0 failed             |
| Static Analysis (Clippy)  | `cargo clippy --all-targets -- -D ...` | **PASS** | 0 warnings, 0 errors            |
| Code Formatting (rustfmt) | `cargo fmt --check`                    | **PASS** | Clean formatting                |
| Documentation Build       | `cargo doc --no-deps`                  | **PASS** | HTML built with zero warnings   |

**Overall Status: ALL TESTS PASSED**

---

## 3. Detailed Results

```text
running 19 tests
test legacy::tests::test_parse_valid_legacy_header ... ok
test model::tests::test_daily_history_rollover_and_csv ... ok
test model::tests::test_date_rollover_with_mouse_and_goal ... ok
test heatmap::tests::test_keyboard_layout_integrity ... ok
test heatmap::tests::test_ratio_to_heatmap_color ... ok
test input::tests::test_distance_calculation ... ok
test input::tests::test_map_mouse_buttons ... ok
test input::tests::test_map_rdev_keys ... ok
test legacy::tests::test_parse_exact_28_bytes ... ok
test legacy::tests::test_parse_insufficient_length ... ok
test model::tests::test_export_to_csv ... ok
test model::tests::test_default_typect_data ... ok
test tray::tests::test_generate_tray_icon_rgba ... ok
test model::tests::test_format_distance ... ok
test model::tests::test_goal_notification ... ok
test model::tests::test_record_key_click_and_mouse_move ... ok
test model::tests::test_save_and_load_json ... ok
test notify::tests::test_notification_creation ... ok
test startup::tests::test_startup_key_path_constants ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 3 tests
test test_legacy_header_parser_edge_cases ... ok
test test_full_lifecycle_with_legacy_import_and_persistence ... ok
test test_daily_history_multi_day_and_compatibility ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc-tests typect_lib
running 1 test
test src\legacy.rs - legacy::LegacyHeader::parse (line 51) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All 23 tests executed cleanly with zero failures.
