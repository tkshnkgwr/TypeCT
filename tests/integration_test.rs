//! TypeCT 結合テスト
//!
//! 旧データインポートから集計、マウス移動距離、永続化、ロールオーバー、
//! CSV エクスポートまでの一連のライフサイクルをテストします。

use std::fs;
use tempfile::tempdir;
use typect_lib::legacy::LegacyHeader;
use typect_lib::model::{ThemeMode, TypeCtData};

#[test]
fn test_full_lifecycle_with_legacy_import_and_persistence() {
    let dir = tempdir().expect("tempdir failed");
    let base_path = dir.path();
    let json_path = base_path.join("typect.json");

    // 1. 旧形式打鍵データ (data) を作成
    let mut key_buf = vec![0u8; 28];
    key_buf[0x00..0x04].copy_from_slice(&100u32.to_le_bytes()); // 100日間
    key_buf[0x10..0x14].copy_from_slice(&50000u32.to_le_bytes()); // 50,000打
    key_buf[0x14..0x18].copy_from_slice(&1200u32.to_le_bytes()); // 今日1200
    key_buf[0x18..0x1C].copy_from_slice(&1500u32.to_le_bytes()); // 昨日1500
    fs::write(base_path.join("data"), key_buf).expect("failed to write data");

    // 2. 旧形式クリックデータ (data2) を作成
    let mut mouse_buf = vec![0u8; 28];
    mouse_buf[0x00..0x04].copy_from_slice(&90u32.to_le_bytes()); // 90日間
    mouse_buf[0x10..0x14].copy_from_slice(&20000u32.to_le_bytes()); // 20,000クリック
    mouse_buf[0x14..0x18].copy_from_slice(&600u32.to_le_bytes()); // 今日600
    mouse_buf[0x18..0x1C].copy_from_slice(&700u32.to_le_bytes()); // 昨日700
    fs::write(base_path.join("data2"), mouse_buf).expect("failed to write data2");

    // 3. TypeCtData を生成し、旧データを自動インポート
    let mut data = TypeCtData {
        daily_goal_keys: 50002, // 目標を 50002 に設定
        theme: ThemeMode::Dark,
        ..Default::default()
    };
    assert!(data.try_import_legacy(base_path));
    assert!(data.legacy_imported);

    assert_eq!(data.total_keys, 50000);
    assert_eq!(data.today_keys, 1200);
    assert_eq!(data.yesterday_keys, 1500);
    assert_eq!(data.total_clicks, 20000);
    assert_eq!(data.today_clicks, 600);
    assert_eq!(data.yesterday_clicks, 700);
    assert_eq!(data.days_recorded, 100);

    // 4. 今日の打鍵、クリック、マウス移動距離を加算
    data.record_key("A", 10);
    data.record_key("Enter", 10);
    data.record_click("左クリック");
    data.record_mouse_move(150.75);

    assert_eq!(data.total_keys, 50002);
    assert_eq!(data.today_keys, 1202);
    assert_eq!(data.hourly_keys[10], 2);
    assert_eq!(data.total_clicks, 20001);
    assert_eq!(data.today_clicks, 601);
    assert_eq!(data.today_mouse_distance_m, 150.75);

    // 5. JSONファイルへ保存
    data.save_to_file(&json_path).expect("save should succeed");
    assert!(json_path.exists());

    // 6. JSONファイルから再ロード
    let mut reloaded = TypeCtData::load_or_default(&json_path);
    assert_eq!(reloaded.total_keys, 50002);
    assert_eq!(reloaded.today_keys, 1202);
    assert!(reloaded.legacy_imported);
    assert_eq!(reloaded.theme, ThemeMode::Dark);
    assert_eq!(reloaded.today_mouse_distance_m, 150.75);

    // 7. CSV エクスポートのテスト
    let csv = reloaded.export_to_csv();
    assert!(csv.contains("# TypeCT 集計統計エクスポート"));
    assert!(csv.contains("本日,1202,601,150.75"));

    // 8. 翌日へのロールオーバー
    reloaded.check_date_rollover_with_date("2099-01-01");
    assert_eq!(reloaded.yesterday_keys, 1202);
    assert_eq!(reloaded.today_keys, 0);
    assert_eq!(reloaded.yesterday_clicks, 601);
    assert_eq!(reloaded.today_clicks, 0);
    assert_eq!(reloaded.yesterday_mouse_distance_m, 150.75);
    assert_eq!(reloaded.today_mouse_distance_m, 0.0);
    assert_eq!(reloaded.days_recorded, 101);
    assert_eq!(reloaded.last_date, "2099-01-01");
}

#[test]
fn test_legacy_header_parser_edge_cases() {
    // 壊れた短いバッファ
    assert!(LegacyHeader::parse(&[0; 10]).is_none());

    // ちょうど28バイトのオールゼロ
    let header = LegacyHeader::parse(&[0; 28]).expect("parse succeeds");
    assert_eq!(header.total_count, 0);
    assert_eq!(header.days_recorded, 0);
}

#[test]
fn test_daily_history_multi_day_and_compatibility() {
    let dir = tempdir().expect("tempdir failed");
    let json_path = dir.path().join("typect.json");

    // 1. 旧バージョン（daily_history フィールドが存在しない JSON）からのロード互換性
    let legacy_json = r#"{
        "total_keys": 1000,
        "today_keys": 200,
        "yesterday_keys": 150,
        "total_clicks": 300,
        "today_clicks": 50,
        "yesterday_clicks": 40,
        "days_recorded": 5,
        "last_date": "2026-09-10",
        "hourly_keys": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        "key_counts": {},
        "mouse_counts": {},
        "legacy_imported": false
    }"#;
    fs::write(&json_path, legacy_json).expect("failed to write legacy json");

    let mut data = TypeCtData::load_or_default(&json_path);
    assert_eq!(data.total_keys, 1000);
    assert!(data.daily_history.is_empty()); // 後方互換性: 自動で空の BTreeMap が初期化される

    // 2. 9/10 -> 9/11 へ日付ロールオーバー
    data.check_date_rollover_with_date("2026-09-11");
    assert_eq!(data.daily_history.len(), 1);
    let s_sep10 = data
        .daily_history
        .get("2026-09-10")
        .expect("2026-09-10 exists");
    assert_eq!(s_sep10.keys, 200);
    assert_eq!(s_sep10.clicks, 50);

    // 3. 9/11 の入力を記録
    data.record_key("KeyA", 9);
    data.record_key("KeyB", 9);
    data.record_click("Left");
    data.record_mouse_move(50.0);

    // 4. 9/11 -> 9/12 へ日付ロールオーバー
    data.check_date_rollover_with_date("2026-09-12");
    assert_eq!(data.daily_history.len(), 2);
    let s_sep11 = data
        .daily_history
        .get("2026-09-11")
        .expect("2026-09-11 exists");
    assert_eq!(s_sep11.keys, 2);
    assert_eq!(s_sep11.clicks, 1);
    assert_eq!(s_sep11.mouse_distance_m, 50.0);

    // 5. 永続化とリロード
    data.sync_today_to_history();
    data.save_to_file(&json_path).expect("save succeeds");

    let reloaded = TypeCtData::load_or_default(&json_path);
    assert_eq!(reloaded.daily_history.len(), 3); // 9/10, 9/11, 9/12 (今日)
    assert!(reloaded.daily_history.contains_key("2026-09-10"));
    assert!(reloaded.daily_history.contains_key("2026-09-11"));
    assert!(reloaded.daily_history.contains_key("2026-09-12"));

    // 6. CSV エクスポート
    let csv = reloaded.export_to_csv();
    assert!(csv.contains("# 日毎サマリー履歴"));
    assert!(csv.contains("2026-09-10,200,50,0.00"));
    assert!(csv.contains("2026-09-11,2,1,50.00"));
}
