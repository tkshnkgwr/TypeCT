# TypeCT テスト結果報告書 (TEST_REPORT)

本レポートは、`TypeCT` に対する自動テストおよびコード静的解析の実施結果をまとめたものです。

---

## 1. 実施環境情報

- **OS**: Windows 11 (64bit)
- **Rust バージョン**: rustc 1.80+ (stable-x86_64-pc-windows-msvc)
- **実施日時**: 2026-09-15
- **対象バージョン**: v0.1.0

---

## 2. 総合判定

| 検査項目                | 実行コマンド                           | 結果    | 備考                              |
| :---------------------- | :------------------------------------- | :------ | :-------------------------------- |
| 単体テスト (Unit Tests) | `cargo test --lib`                     | **合格**| 12 件合格 / 0 件失敗              |
| 結合テスト (Integration)| `cargo test --test integration_test`   | **合格**| 2 件合格 / 0 件失敗               |
| ドキュメントテスト      | `cargo test --doc`                     | **合格**| 2 件合格 / 0 件失敗               |
| 静的解析 (Clippy)       | `cargo clippy --all-targets -- -D ...` | **合格**| 警告・エラー 0 件                 |
| コード書式 (rustfmt)    | `cargo fmt --check`                    | **合格**| 書式違反 0 件                     |
| ドキュメント生成        | `cargo doc --no-deps`                  | **合格**| 警告なし・HTML正常出力確認済み    |

**総合判定: 合格 (ALL PASSED)**

---

## 3. テスト詳細結果

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

すべてのテスト項目が期待通りの振る舞いを示し、境界値および旧データインポートの堅牢性が確認されました。
