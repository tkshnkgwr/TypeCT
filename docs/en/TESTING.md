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
| No | Test Function                        | Scope & Verification Goal                               |
| :- | :----------------------------------- | :------------------------------------------------------ |
| 1  | `test_parse_valid_legacy_header`     | Correctly parses 7 u32 fields from 28-byte little-endian |
| 2  | `test_parse_exact_28_bytes`          | Verifies boundary behavior with exactly 28 bytes        |
| 3  | `test_parse_insufficient_length`     | Safely returns `None` when buffer is under 28 bytes     |

### 2.2 Model & Persistence (`model.rs`)
| No | Test Function                        | Scope & Verification Goal                               |
| :- | :----------------------------------- | :------------------------------------------------------ |
| 4  | `test_default_typect_data`           | Default counters, arrays, and flags initialized         |
| 5  | `test_record_key_and_click`          | Aggregation of keystrokes, clicks, and hourly slots     |
| 6  | `test_average_daily_keys`            | Calculation of daily average with divide-by-zero guard  |
| 7  | `test_date_rollover`                 | Slides today's metrics to yesterday on midnight change   |
| 8  | `test_save_and_load_json`            | Round-trip serialization and deserialization via JSON   |
| 9  | `test_try_import_legacy`             | Mock data/data2 import and idempotency verification    |

### 2.3 Input Mapping (`input.rs`)
| No | Test Function                        | Scope & Verification Goal                               |
| :- | :----------------------------------- | :------------------------------------------------------ |
| 10 | `test_map_rdev_keys`                 | Correct symbolic mapping for alphanumeric & arrows      |
| 11 | `test_map_mouse_buttons`             | Japanese name resolution for mouse buttons              |

### 2.4 System Tray Icon (`tray.rs`)
| No | Test Function                        | Scope & Verification Goal                               |
| :- | :----------------------------------- | :------------------------------------------------------ |
| 12 | `test_generate_tray_icon_rgba`       | Validates 32x32 RGBA buffer length (4096 bytes) & logo  |

### 2.5 Integration Tests (`tests/integration_test.rs`)
| No | Test Function                                            | Scope & Verification Goal                   |
| :- | :------------------------------------------------------- | :------------------------------------------ |
| 13 | `test_full_lifecycle_with_legacy_import_and_persistence` | Legacy import -> increment -> JSON -> shift |
| 14 | `test_legacy_header_parser_edge_cases`                   | Defends against zeroed or truncated buffers |

---

## 3. Running the Tests

```powershell
cargo test --verbose
```
