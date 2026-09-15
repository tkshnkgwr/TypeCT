# TypeCT テスト結果報告書 (TEST_REPORT)

本レポートは、`TypeCT` に対する自動テストおよびコード静的解析の実施結果をまとめたものです。

---

## 1. 実施環境情報

- **OS**: Windows 11 (64bit)
- **Rust バージョン**: rustc 1.80+ (stable-x86_64-pc-windows-msvc)
- **実施日時**: 2026-09-15
- **対象バージョン**: v0.2.0

---

## 2. 総合判定

| 検査項目                | 実行コマンド                           | 結果    | 備考                              |
| :---------------------- | :------------------------------------- | :------ | :-------------------------------- |
| 単体テスト (Unit Tests) | `cargo test --lib`                     | **合格**| 19 件合格 / 0 件失敗              |
| 結合テスト (Integration)| `cargo test --test integration_test`   | **合格**| 3 件合格 / 0 件失敗               |
| ドキュメントテスト      | `cargo test --doc`                     | **合格**| 1 件合格 / 0 件失敗               |
| 静的解析 (Clippy)       | `cargo clippy --all-targets -- -D ...` | **合格**| 警告・エラー 0 件                 |
| コード書式 (rustfmt)    | `cargo fmt --check`                    | **合格**| 書式違反 0 件                     |
| ドキュメント生成        | `cargo doc --no-deps`                  | **合格**| 警告なし・HTML正常出力確認済み    |

**総合判定: 合格 (ALL PASSED)**

---

## 3. テスト詳細結果

```text
running 18 tests
test heatmap::tests::test_keyboard_layout_integrity ... ok
test input::tests::test_map_mouse_buttons ... ok
test legacy::tests::test_parse_exact_28_bytes ... ok
test heatmap::tests::test_ratio_to_heatmap_color ... ok
test input::tests::test_distance_calculation ... ok
test legacy::tests::test_parse_insufficient_length ... ok
test input::tests::test_map_rdev_keys ... ok
test model::tests::test_format_distance ... ok
test legacy::tests::test_parse_valid_legacy_header ... ok
test notify::tests::test_notification_creation ... ok
test startup::tests::test_startup_key_path_constants ... ok
test tray::tests::test_generate_tray_icon_rgba ... ok
test model::tests::test_default_typect_data ... ok
test model::tests::test_record_key_click_and_mouse_move ... ok
test model::tests::test_export_to_csv ... ok
test model::tests::test_goal_notification ... ok
test model::tests::test_date_rollover_with_mouse_and_goal ... ok
test model::tests::test_save_and_load_json ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 2 tests
test test_legacy_header_parser_edge_cases ... ok
test test_full_lifecycle_with_legacy_import_and_persistence ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc-tests typect_lib
running 1 test
test src\legacy.rs - legacy::LegacyHeader::parse (line 51) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

全 21 テストすべてが合格し、新機能（マウス移動距離計算、ヒートマップレイアウト、通知生成、スタートアップパス定数、CSVエクスポート、設定保持）が期待通り動作することを確認しました。
