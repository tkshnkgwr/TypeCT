# TypeCT Testing Specifications (TESTING)

This document details the testing architecture, test matrix, and verification plan for `TypeCT`.

---

## 1. Testing Strategy

Quality assurance is maintained through three testing tiers:

1. **Unit Tests**:
   - Isolates and verifies components in `legacy.rs`, `model.rs`, `input.rs`, and `tray.rs`.
2. **Integration Tests (`tests/integration_test.rs`)**:
   - Validates end-to-end flows: binary parsing -> data aggregation -> JSON persistence -> reload -> date rollover.
3. **Doc-Tests**:
   - Ensures code examples inside rustdoc comments compile and pass assertions.

---

## 2. Test Suite Matrix

### 2.1 Legacy Header Parser (`legacy.rs`)
| No | Test Function                    | Scope & Verification Goal                               |
| :- | :------------------------------- | :------------------------------------------------------ |
| 1  | `test_parse_valid_legacy_header` | Correctly parses 7 u32 fields from 28-byte little-endian |
| 2  | `test_parse_exact_28_bytes`      | Verifies boundary behavior with exactly 28 bytes        |
| 3  | `test_parse_insufficient_length` | Safely returns `None` when buffer is under 28 bytes     |

### 2.2 Model & Persistence (`model.rs`)
| No | Test Function                            | Scope & Verification Goal                                       |
| :- | :--------------------------------------- | :-------------------------------------------------------------- |
| 4  | `test_default_typect_data`               | Default counters, arrays, and flags initialized                 |
| 5  | `test_record_key_click_and_mouse_move`   | Aggregation of keys, clicks, hourly slots, and mouse travel     |
| 6  | `test_format_distance`                   | Accurate distance unit conversion (m / km) and decimal formatting|
| 7  | `test_date_rollover_with_mouse_and_goal` | Slides today's metrics to yesterday & resets goal toast flag    |
| 8  | `test_goal_notification`                 | Triggers celebration flag at target and prevents double-firing  |
| 9  | `test_export_to_csv`                     | Full CSV export containing summaries, hourly slots, and rankings|
| 10 | `test_daily_history_rollover_and_csv`    | Validates daily summary ledger rollover and CSV integration     |
| 11 | `test_save_and_load_json`                | Full round-trip serialization and deserialization via JSON      |

### 2.3 Input Mapping & Travel Distance (`input.rs`)
| No | Test Function                | Scope & Verification Goal                                     |
| :- | :--------------------------- | :------------------------------------------------------------ |
| 12 | `test_map_rdev_keys`         | Correct symbolic mapping for alphanumeric, modifier & arrows  |
| 13 | `test_map_mouse_buttons`     | Name resolution for mouse buttons (Left, Right, Middle, Extra)|
| 14 | `test_distance_calculation`  | Converts cursor displacement into physical meters (96 DPI)    |

### 2.4 Keyboard Heatmap (`heatmap.rs`)
| No | Test Function                     | Scope & Verification Goal                                      |
| :- | :-------------------------------- | :------------------------------------------------------------- |
| 15 | `test_keyboard_layout_integrity`  | Verifies 6-row QWERTY keyboard layout integrity and coordinates |
| 16 | `test_ratio_to_heatmap_color`     | Maps relative typing frequency to 4-tier color gradient values |

### 2.5 Toast, Startup & System Tray (`notify.rs`, `startup.rs`, `tray.rs`)
| No | Test Function                     | Scope & Verification Goal                                      |
| :- | :-------------------------------- | :------------------------------------------------------------- |
| 17 | `test_notification_creation`      | Toast alert constructor and notification message formatting    |
| 18 | `test_startup_key_path_constants` | Windows registry Run key path and application identity strings |
| 19 | `test_generate_tray_icon_rgba`    | Validates 32x32 RGBA buffer length (4096 bytes) & logo glyph   |

### 2.6 Integration Tests (`tests/integration_test.rs`)
| No | Test Function                                            | Scope & Verification Goal                                 |
| :- | :------------------------------------------------------- | :-------------------------------------------------------- |
| 20 | `test_legacy_header_parser_edge_cases`                   | Defends against zeroed or truncated buffers               |
| 21 | `test_full_lifecycle_with_legacy_import_and_persistence` | Legacy import -> increment -> JSON -> rollover shift      |
| 22 | `test_daily_history_multi_day_and_compatibility`         | Multi-day rollover into daily history ledger & backward compatibility |

### 2.7 Documentation Tests (`src/legacy.rs`)
| No | Target Function              | Scope & Verification Goal                                      |
| :- | :--------------------------- | :------------------------------------------------------------- |
| 23 | `legacy::LegacyHeader::parse`| Verifies runnable sample code in RustDoc binary parsing section|

---

## 3. Running the Tests

```powershell
cargo test --verbose
```
