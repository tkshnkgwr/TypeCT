//! TypeCT 結合テスト
//!
//! 旧データインポートから集計、永続化、ロールオーバーまでの
//! 一連のライフサイクルをテストします。

use std::fs;
use tempfile::tempdir;
use typect_lib::legacy::LegacyHeader;
use typect_lib::model::TypeCtData;

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
    let mut data = TypeCtData::default();
    assert!(data.try_import_legacy(base_path));
    assert!(data.legacy_imported);

    assert_eq!(data.total_keys, 50000);
    assert_eq!(data.today_keys, 1200);
    assert_eq!(data.yesterday_keys, 1500);
    assert_eq!(data.total_clicks, 20000);
    assert_eq!(data.today_clicks, 600);
    assert_eq!(data.yesterday_clicks, 700);
    assert_eq!(data.days_recorded, 100);

    // 4. 今日の打鍵とクリックを加算
    data.record_key("A", 10);
    data.record_key("Enter", 10);
    data.record_click("左クリック");

    assert_eq!(data.total_keys, 50002);
    assert_eq!(data.today_keys, 1202);
    assert_eq!(data.hourly_keys[10], 2);
    assert_eq!(data.total_clicks, 20001);
    assert_eq!(data.today_clicks, 601);

    // 5. JSONファイルへ保存
    data.save_to_file(&json_path).expect("save should succeed");
    assert!(json_path.exists());

    // 6. JSONファイルから再ロード
    let mut reloaded = TypeCtData::load_or_default(&json_path);
    assert_eq!(reloaded.total_keys, 50002);
    assert_eq!(reloaded.today_keys, 1202);
    assert!(reloaded.legacy_imported);

    // 7. 翌日へのロールオーバー
    reloaded.check_date_rollover_with_date("2099-01-01");
    assert_eq!(reloaded.yesterday_keys, 1202);
    assert_eq!(reloaded.today_keys, 0);
    assert_eq!(reloaded.yesterday_clicks, 601);
    assert_eq!(reloaded.today_clicks, 0);
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
