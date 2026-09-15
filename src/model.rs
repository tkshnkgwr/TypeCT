//! データモデル & 永続化モジュール
//!
//! 打鍵数・マウスクリック数の集計データ構造体 `TypeCtData`、
//! 日付ロールオーバー判定、旧バイナリデータ自動インポート、
//! および JSON ファイルの読み書き機能を提供します。

use crate::legacy::LegacyHeader;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// JSON 保存用の集計データ構造体
///
/// 打鍵数およびマウスクリック数の統計情報、時間帯別分布、
/// 主要キー別のランキング情報などを保持します。
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TypeCtData {
    /// 累計総打鍵数
    pub total_keys: u64,
    /// 本日の打鍵数
    pub today_keys: u64,
    /// 昨日の打鍵数
    pub yesterday_keys: u64,
    /// 累計マウスクリック数
    pub total_clicks: u64,
    /// 本日のマウスクリック数
    pub today_clicks: u64,
    /// 昨日のマウスクリック数
    pub yesterday_clicks: u64,
    /// 記録日数（通算日数）
    pub days_recorded: u32,
    /// 最終記録日（"YYYY-MM-DD" 形式）
    pub last_date: String,
    /// 直近24時間の時間帯別打鍵数（0時〜23時）
    pub hourly_keys: [u32; 24],
    /// キー種別ごとの打鍵数マップ
    pub key_counts: HashMap<String, u64>,
    /// マウスボタン種別ごとのクリック数マップ
    pub mouse_counts: HashMap<String, u64>,
    /// 旧バージョン (typect) からのインポートが実行済みかどうか
    pub legacy_imported: bool,
}

impl Default for TypeCtData {
    /// 現在日付を初期値として `TypeCtData` を初期化します。
    fn default() -> Self {
        let today = Local::now().format("%Y-%m-%d").to_string();
        Self {
            total_keys: 0,
            today_keys: 0,
            yesterday_keys: 0,
            total_clicks: 0,
            today_clicks: 0,
            yesterday_clicks: 0,
            days_recorded: 1,
            last_date: today,
            hourly_keys: [0; 24],
            key_counts: HashMap::new(),
            mouse_counts: HashMap::new(),
            legacy_imported: false,
        }
    }
}

impl TypeCtData {
    /// 1日あたりの平均打鍵数を算出します。
    ///
    /// 記録日数が 0 の場合は累計打鍵数をそのまま返します。
    ///
    /// # 例
    /// ```rust
    /// use typect_lib::model::TypeCtData;
    ///
    /// let mut data = TypeCtData::default();
    /// data.total_keys = 10000;
    /// data.days_recorded = 5;
    /// assert_eq!(data.average_daily_keys(), 2000);
    /// ```
    #[inline]
    pub fn average_daily_keys(&self) -> u64 {
        if self.days_recorded > 0 {
            self.total_keys / (self.days_recorded as u64)
        } else {
            self.total_keys
        }
    }

    /// 日付変更（日付またぎ）の判定と集計ロールオーバー処理を行います。
    ///
    /// 前回記録日と現在日付が異なる場合、本日の記録を昨日へスライドし、
    /// 本日および時間帯別カウントをリセットして記録日数を 1 加算します。
    pub fn check_date_rollover(&mut self) {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.check_date_rollover_with_date(&today);
    }

    /// 指定日付文字列を用いてロールオーバー処理を行います（テスト容易性のため）。
    pub fn check_date_rollover_with_date(&mut self, current_date: &str) {
        if self.last_date != current_date {
            self.yesterday_keys = self.today_keys;
            self.yesterday_clicks = self.today_clicks;
            self.today_keys = 0;
            self.today_clicks = 0;
            self.hourly_keys = [0; 24];
            self.days_recorded = self.days_recorded.saturating_add(1);
            self.last_date = current_date.to_string();
        }
    }

    /// キー入力イベントを集計に反映します。
    ///
    /// # 引数
    /// * `key_name` - キー表示名
    /// * `hour` - 現在の時刻 (0..=23)
    pub fn record_key(&mut self, key_name: &str, hour: usize) {
        self.total_keys = self.total_keys.saturating_add(1);
        self.today_keys = self.today_keys.saturating_add(1);
        if hour < 24 {
            self.hourly_keys[hour] = self.hourly_keys[hour].saturating_add(1);
        }
        *self.key_counts.entry(key_name.to_string()).or_insert(0) += 1;
    }

    /// マウスクリックイベントを集計に反映します。
    ///
    /// # 引数
    /// * `button_name` - マウスボタン表示名
    pub fn record_click(&mut self, button_name: &str) {
        self.total_clicks = self.total_clicks.saturating_add(1);
        self.today_clicks = self.today_clicks.saturating_add(1);
        *self
            .mouse_counts
            .entry(button_name.to_string())
            .or_insert(0) += 1;
    }

    /// 旧バイナリファイル (`data` / `data2`) が指定ディレクトリに存在する場合の自動インポート
    ///
    /// すでにインポート済みの場合はスキップします。
    ///
    /// # 引数
    /// * `base_dir` - ファイル探索対象のディレクトリパス
    ///
    /// # 戻り値
    /// * `true` - 新たに旧データをインポートした場合
    /// * `false` - 旧データが存在しなかった、または既にインポート済みの場合
    pub fn try_import_legacy(&mut self, base_dir: &Path) -> bool {
        if self.legacy_imported {
            return false;
        }

        let key_data_path = base_dir.join("data");
        let mouse_data_path = base_dir.join("data2");
        let mut imported_any = false;

        // 打鍵データ (data) の引き継ぎ
        if key_data_path.exists() {
            if let Ok(bytes) = fs::read(&key_data_path) {
                if let Some(header) = LegacyHeader::parse(&bytes) {
                    self.total_keys = self.total_keys.saturating_add(header.total_count as u64);
                    self.today_keys = header.today_count as u64;
                    self.yesterday_keys = header.yesterday_count as u64;
                    self.days_recorded = self.days_recorded.max(header.days_recorded);
                    imported_any = true;
                }
            }
        }

        // マウスクリックデータ (data2) の引き継ぎ
        if mouse_data_path.exists() {
            if let Ok(bytes) = fs::read(&mouse_data_path) {
                if let Some(header) = LegacyHeader::parse(&bytes) {
                    self.total_clicks = self.total_clicks.saturating_add(header.total_count as u64);
                    self.today_clicks = header.today_count as u64;
                    self.yesterday_clicks = header.yesterday_count as u64;
                    self.days_recorded = self.days_recorded.max(header.days_recorded);
                    imported_any = true;
                }
            }
        }

        if imported_any {
            self.legacy_imported = true;
        }

        imported_any
    }

    /// JSON ファイルからデータをロードします。ファイルが存在しない場合や破損時はデフォルト値を返します。
    pub fn load_or_default(json_path: &Path) -> Self {
        if json_path.exists() {
            if let Ok(content) = fs::read_to_string(json_path) {
                if let Ok(data) = serde_json::from_str::<TypeCtData>(&content) {
                    return data;
                }
            }
        }
        Self::default()
    }

    /// データを JSON 形式でファイルへ保存します。
    pub fn save_to_file(&self, json_path: &Path) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(self)?;
        fs::write(json_path, serialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_typect_data() {
        let data = TypeCtData::default();
        assert_eq!(data.total_keys, 0);
        assert_eq!(data.today_keys, 0);
        assert_eq!(data.yesterday_keys, 0);
        assert_eq!(data.total_clicks, 0);
        assert_eq!(data.days_recorded, 1);
        assert!(!data.legacy_imported);
        assert_eq!(data.hourly_keys, [0; 24]);
    }

    #[test]
    fn test_record_key_and_click() {
        let mut data = TypeCtData::default();
        data.record_key("A", 14);
        data.record_key("A", 14);
        data.record_key("Enter", 15);
        data.record_click("左クリック");

        assert_eq!(data.total_keys, 3);
        assert_eq!(data.today_keys, 3);
        assert_eq!(data.hourly_keys[14], 2);
        assert_eq!(data.hourly_keys[15], 1);
        assert_eq!(data.key_counts.get("A"), Some(&2));
        assert_eq!(data.key_counts.get("Enter"), Some(&1));

        assert_eq!(data.total_clicks, 1);
        assert_eq!(data.today_clicks, 1);
        assert_eq!(data.mouse_counts.get("左クリック"), Some(&1));
    }

    #[test]
    fn test_average_daily_keys() {
        let mut data = TypeCtData {
            total_keys: 12000,
            days_recorded: 4,
            ..Default::default()
        };
        assert_eq!(data.average_daily_keys(), 3000);

        data.days_recorded = 0;
        assert_eq!(data.average_daily_keys(), 12000);
    }

    #[test]
    fn test_date_rollover() {
        let mut data = TypeCtData {
            total_keys: 500,
            today_keys: 300,
            yesterday_keys: 200,
            total_clicks: 100,
            today_clicks: 60,
            yesterday_clicks: 40,
            days_recorded: 2,
            last_date: "2026-09-14".to_string(),
            hourly_keys: [10; 24],
            key_counts: HashMap::new(),
            mouse_counts: HashMap::new(),
            legacy_imported: false,
        };

        // 翌日 "2026-09-15" へのロールオーバー
        data.check_date_rollover_with_date("2026-09-15");

        assert_eq!(data.yesterday_keys, 300);
        assert_eq!(data.today_keys, 0);
        assert_eq!(data.yesterday_clicks, 60);
        assert_eq!(data.today_clicks, 0);
        assert_eq!(data.hourly_keys, [0; 24]);
        assert_eq!(data.days_recorded, 3);
        assert_eq!(data.last_date, "2026-09-15");

        // 同一日付の場合は変化なし
        data.check_date_rollover_with_date("2026-09-15");
        assert_eq!(data.days_recorded, 3);
    }

    #[test]
    fn test_save_and_load_json() {
        let dir = tempdir().expect("tempdir failed");
        let json_path = dir.path().join("typect.json");

        let mut data = TypeCtData::default();
        data.record_key("Space", 10);
        data.record_click("右クリック");

        data.save_to_file(&json_path).expect("save should succeed");
        assert!(json_path.exists());

        let loaded = TypeCtData::load_or_default(&json_path);
        assert_eq!(loaded.total_keys, 1);
        assert_eq!(loaded.total_clicks, 1);
        assert_eq!(loaded.key_counts.get("Space"), Some(&1));
    }

    #[test]
    fn test_try_import_legacy() {
        let dir = tempdir().expect("tempdir failed");
        let base_path = dir.path();

        // 疑似 data (打鍵) 作成
        let mut key_buf = vec![0u8; 28];
        key_buf[0x00..0x04].copy_from_slice(&50u32.to_le_bytes()); // 50日
        key_buf[0x10..0x14].copy_from_slice(&10000u32.to_le_bytes()); // 10000打
        key_buf[0x14..0x18].copy_from_slice(&200u32.to_le_bytes()); // 今日200
        key_buf[0x18..0x1C].copy_from_slice(&350u32.to_le_bytes()); // 昨日350
        fs::write(base_path.join("data"), key_buf).expect("write data failed");

        // 疑似 data2 (クリック) 作成
        let mut mouse_buf = vec![0u8; 28];
        mouse_buf[0x00..0x04].copy_from_slice(&40u32.to_le_bytes()); // 40日
        mouse_buf[0x10..0x14].copy_from_slice(&5000u32.to_le_bytes()); // 5000クリック
        mouse_buf[0x14..0x18].copy_from_slice(&120u32.to_le_bytes()); // 今日120
        mouse_buf[0x18..0x1C].copy_from_slice(&180u32.to_le_bytes()); // 昨日180
        fs::write(base_path.join("data2"), mouse_buf).expect("write data2 failed");

        let mut data = TypeCtData::default();
        let imported = data.try_import_legacy(base_path);
        assert!(imported);
        assert!(data.legacy_imported);
        assert_eq!(data.total_keys, 10000);
        assert_eq!(data.today_keys, 200);
        assert_eq!(data.yesterday_keys, 350);
        assert_eq!(data.total_clicks, 5000);
        assert_eq!(data.today_clicks, 120);
        assert_eq!(data.yesterday_clicks, 180);
        assert_eq!(data.days_recorded, 50);

        // 二重インポート防止
        let imported_second = data.try_import_legacy(base_path);
        assert!(!imported_second);
        assert_eq!(data.total_keys, 10000);
    }
}
