//! データモデル & 永続化モジュール
//!
//! 打鍵数・マウスクリック数・マウス移動距離の集計データ構造体 `TypeCtData`、
//! 日付ロールオーバー判定、目標打鍵数達成判定、旧バイナリデータ自動インポート、
//! CSV エクスポート、および JSON ファイルの読み書き機能を提供します。

use crate::legacy::LegacyHeader;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

/// カラーテーマ設定
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    /// OS 設定追従（デフォルト）
    #[default]
    System,
    /// ダークテーマ
    Dark,
    /// ライトテーマ
    Light,
}

impl ThemeMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::System => "💻 システム設定",
            Self::Dark => "🌙 ダーク",
            Self::Light => "☀️ ライト",
        }
    }
}

/// 1日あたりの集計サマリー
///
/// プライバシーとセキュリティ（キーロガー化の完全防止）のため、
/// 個別のキー文字内訳は含まず、日別の数値合計（打鍵数・クリック数・移動距離）のみを保持します。
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub struct DailySummary {
    /// その日の総打鍵数
    pub keys: u64,
    /// その日の総マウスクリック数
    pub clicks: u64,
    /// その日の総マウス移動距離（メートル）
    #[serde(default)]
    pub mouse_distance_m: f64,
}

/// JSON 保存用の集計データ構造体
///
/// 打鍵数、マウスクリック数、マウス移動距離、時間帯別分布、
/// 主要キー別のランキング情報、目標設定および各種環境設定を保持します。
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

    // --- 新規拡張フィールド（後方互換性のため #[serde(default)]） ---
    /// 累計マウス移動距離（メートル）
    #[serde(default)]
    pub total_mouse_distance_m: f64,
    /// 本日のマウス移動距離（メートル）
    #[serde(default)]
    pub today_mouse_distance_m: f64,
    /// 昨日のマウス移動距離（メートル）
    #[serde(default)]
    pub yesterday_mouse_distance_m: f64,
    /// 1日の目標打鍵数（既定値: 10,000）
    #[serde(default = "default_daily_goal")]
    pub daily_goal_keys: u64,
    /// 本日の目標打鍵数達成通知済みフラグ
    #[serde(default)]
    pub goal_notified: bool,
    /// UI カラーテーマ
    #[serde(default)]
    pub theme: ThemeMode,
    /// Windows スタートアップ登録設定
    #[serde(default)]
    pub auto_start: bool,
    /// 過去の日毎集計サマリー台帳（日付 "YYYY-MM-DD" -> サマリー）
    ///
    /// キー順（時系列昇順）でシリアライズされるよう BTreeMap を使用。
    /// 文字ごとの内訳は含めず、数値の総数のみを安全に記録します。
    #[serde(default)]
    pub daily_history: BTreeMap<String, DailySummary>,
}

fn default_daily_goal() -> u64 {
    10_000
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
            total_mouse_distance_m: 0.0,
            today_mouse_distance_m: 0.0,
            yesterday_mouse_distance_m: 0.0,
            daily_goal_keys: 10_000,
            goal_notified: false,
            theme: ThemeMode::System,
            auto_start: false,
            daily_history: BTreeMap::new(),
        }
    }
}

impl TypeCtData {
    /// 1日あたりの平均打鍵数を算出します。
    ///
    /// 記録日数が 0 の場合は累計打鍵数をそのまま返します。
    #[inline]
    pub fn average_daily_keys(&self) -> u64 {
        if self.days_recorded > 0 {
            self.total_keys / (self.days_recorded as u64)
        } else {
            self.total_keys
        }
    }

    /// 日付変更（日付またぎ）の判定と集計ロールオーバー処理を行います。
    pub fn check_date_rollover(&mut self) {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.check_date_rollover_with_date(&today);
    }

    /// 指定日付文字列を用いてロールオーバー処理を行います（テスト容易性のため）。
    pub fn check_date_rollover_with_date(&mut self, current_date: &str) {
        if self.last_date != current_date {
            // 前日（直前の日）の実績を daily_history に永続保存
            if !self.last_date.is_empty() {
                self.daily_history.insert(
                    self.last_date.clone(),
                    DailySummary {
                        keys: self.today_keys,
                        clicks: self.today_clicks,
                        mouse_distance_m: self.today_mouse_distance_m,
                    },
                );
            }

            self.yesterday_keys = self.today_keys;
            self.yesterday_clicks = self.today_clicks;
            self.yesterday_mouse_distance_m = self.today_mouse_distance_m;

            self.today_keys = 0;
            self.today_clicks = 0;
            self.today_mouse_distance_m = 0.0;
            self.hourly_keys = [0; 24];
            self.days_recorded = self.days_recorded.saturating_add(1);
            self.last_date = current_date.to_string();
            self.goal_notified = false; // 新しい日は目標通知を再有効化
        }
    }

    /// キー入力イベントを集計に反映します。
    pub fn record_key(&mut self, key_name: &str, hour: usize) {
        self.total_keys = self.total_keys.saturating_add(1);
        self.today_keys = self.today_keys.saturating_add(1);
        if hour < 24 {
            self.hourly_keys[hour] = self.hourly_keys[hour].saturating_add(1);
        }
        *self.key_counts.entry(key_name.to_string()).or_insert(0) += 1;
    }

    /// マウスクリックイベントを集計に反映します。
    pub fn record_click(&mut self, button_name: &str) {
        self.total_clicks = self.total_clicks.saturating_add(1);
        self.today_clicks = self.today_clicks.saturating_add(1);
        *self
            .mouse_counts
            .entry(button_name.to_string())
            .or_insert(0) += 1;
    }

    /// マウス移動距離（メートル）を加算します。
    pub fn record_mouse_move(&mut self, distance_m: f64) {
        if distance_m > 0.0 {
            self.total_mouse_distance_m += distance_m;
            self.today_mouse_distance_m += distance_m;
        }
    }

    /// 目標打鍵数に達したかどうかを判定し、初回到達時のみ `true` を返します。
    pub fn check_goal_reached(&mut self) -> bool {
        if !self.goal_notified
            && self.daily_goal_keys > 0
            && self.today_keys >= self.daily_goal_keys
        {
            self.goal_notified = true;
            true
        } else {
            false
        }
    }

    /// 旧バイナリファイル (`data` / `data2`) が指定ディレクトリに存在する場合の自動インポート
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

    /// 本日の最新集計データを `daily_history` に反映・同期します。
    pub fn sync_today_to_history(&mut self) {
        if !self.last_date.is_empty() {
            self.daily_history.insert(
                self.last_date.clone(),
                DailySummary {
                    keys: self.today_keys,
                    clicks: self.today_clicks,
                    mouse_distance_m: self.today_mouse_distance_m,
                },
            );
        }
    }

    /// 本日分も含めた日毎サマリー履歴（日付昇順）を取得します。
    pub fn get_combined_history(&self) -> BTreeMap<String, DailySummary> {
        let mut history = self.daily_history.clone();
        if !self.last_date.is_empty() {
            history.insert(
                self.last_date.clone(),
                DailySummary {
                    keys: self.today_keys,
                    clicks: self.today_clicks,
                    mouse_distance_m: self.today_mouse_distance_m,
                },
            );
        }
        history
    }

    /// 集計データを CSV 形式の文字列として書き出します。
    pub fn export_to_csv(&self) -> String {
        let mut csv = String::new();

        // 1. サマリー情報
        csv.push_str("# TypeCT 集計統計エクスポート\n");
        csv.push_str(&format!("# 出力日付: {}\n", self.last_date));
        csv.push_str("項目,打鍵数,マウスクリック数,マウス移動距離(m)\n");
        csv.push_str(&format!(
            "本日,{},{},{:.2}\n",
            self.today_keys, self.today_clicks, self.today_mouse_distance_m
        ));
        csv.push_str(&format!(
            "昨日,{},{},{:.2}\n",
            self.yesterday_keys, self.yesterday_clicks, self.yesterday_mouse_distance_m
        ));
        csv.push_str(&format!(
            "累計,{},{},{:.2}\n",
            self.total_keys, self.total_clicks, self.total_mouse_distance_m
        ));
        csv.push_str(&format!(
            "記録日数 / 1日平均,{},{},-\n\n",
            self.days_recorded,
            self.average_daily_keys()
        ));

        // 2. 時間帯別打鍵数
        csv.push_str("# 直近24時間の時間帯別打鍵数\n");
        csv.push_str("時間帯,打鍵数\n");
        for (hour, &count) in self.hourly_keys.iter().enumerate() {
            csv.push_str(&format!("{:02}:00 - {:02}:59,{}\n", hour, hour, count));
        }
        csv.push('\n');

        // 3. 主要キー別打鍵ランキング
        csv.push_str("# 主要キー別打鍵ランキング\n");
        csv.push_str("順位,キー名,打鍵回数\n");
        let mut sorted_keys: Vec<(&String, &u64)> = self.key_counts.iter().collect();
        sorted_keys.sort_by(|a, b| b.1.cmp(a.1));
        for (i, (key, &count)) in sorted_keys.iter().enumerate() {
            csv.push_str(&format!("{},{},{}\n", i + 1, key, count));
        }
        csv.push('\n');

        // 4. マウスボタン別クリック数
        csv.push_str("# マウスボタン別クリック数\n");
        csv.push_str("ボタン種別,クリック回数\n");
        for (btn, count) in &self.mouse_counts {
            csv.push_str(&format!("{},{}\n", btn, count));
        }
        csv.push('\n');

        // 5. 日毎サマリー履歴（最新日付順）
        csv.push_str("# 日毎サマリー履歴\n");
        csv.push_str("日付,打鍵数,マウスクリック数,マウス移動距離(m)\n");
        let combined = self.get_combined_history();
        for (date, summary) in combined.iter().rev() {
            csv.push_str(&format!(
                "{},{},{},{:.2}\n",
                date, summary.keys, summary.clicks, summary.mouse_distance_m
            ));
        }

        csv
    }

    /// CSV ファイルを指定パスへ書き出します。
    pub fn export_csv_to_file(&self, path: &Path) -> std::io::Result<()> {
        let csv_content = self.export_to_csv();
        fs::write(path, csv_content)
    }
}

/// メートル単位の距離を人間が読みやすい形式（m / km）に整形します。
pub fn format_distance(meters: f64) -> String {
    if meters >= 1000.0 {
        format!("{:.2} km", meters / 1000.0)
    } else {
        format!("{:.1} m", meters)
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
        assert_eq!(data.total_mouse_distance_m, 0.0);
        assert_eq!(data.daily_goal_keys, 10_000);
        assert_eq!(data.theme, ThemeMode::System);
    }

    #[test]
    fn test_record_key_click_and_mouse_move() {
        let mut data = TypeCtData::default();
        data.record_key("A", 14);
        data.record_key("A", 14);
        data.record_key("Enter", 15);
        data.record_click("左クリック");
        data.record_mouse_move(12.5);

        assert_eq!(data.total_keys, 3);
        assert_eq!(data.today_keys, 3);
        assert_eq!(data.hourly_keys[14], 2);
        assert_eq!(data.hourly_keys[15], 1);
        assert_eq!(data.key_counts.get("A"), Some(&2));

        assert_eq!(data.total_clicks, 1);
        assert_eq!(data.today_clicks, 1);
        assert_eq!(data.today_mouse_distance_m, 12.5);
        assert_eq!(data.total_mouse_distance_m, 12.5);
    }

    #[test]
    fn test_goal_notification() {
        let mut data = TypeCtData {
            daily_goal_keys: 100,
            ..Default::default()
        };

        // まだ目標未満
        data.today_keys = 99;
        assert!(!data.check_goal_reached());
        assert!(!data.goal_notified);

        // 目標達成 (100)
        data.today_keys = 100;
        assert!(data.check_goal_reached());
        assert!(data.goal_notified);

        // 既に通知済みのため2回目は false
        assert!(!data.check_goal_reached());
    }

    #[test]
    fn test_date_rollover_with_mouse_and_goal() {
        let mut data = TypeCtData {
            today_keys: 500,
            today_clicks: 100,
            today_mouse_distance_m: 250.0,
            goal_notified: true,
            last_date: "2026-09-14".to_string(),
            ..Default::default()
        };

        data.check_date_rollover_with_date("2026-09-15");

        assert_eq!(data.yesterday_keys, 500);
        assert_eq!(data.today_keys, 0);
        assert_eq!(data.yesterday_mouse_distance_m, 250.0);
        assert_eq!(data.today_mouse_distance_m, 0.0);
        assert!(!data.goal_notified);
    }

    #[test]
    fn test_format_distance() {
        assert_eq!(format_distance(50.42), "50.4 m");
        assert_eq!(format_distance(999.9), "999.9 m");
        assert_eq!(format_distance(1234.5), "1.23 km");
        assert_eq!(format_distance(10500.0), "10.50 km");
    }

    #[test]
    fn test_export_to_csv() {
        let mut data = TypeCtData::default();
        data.record_key("Space", 10);
        data.record_click("左クリック");
        data.record_mouse_move(100.0);

        let csv = data.export_to_csv();
        assert!(csv.contains("# TypeCT 集計統計エクスポート"));
        assert!(csv.contains("本日,1,1,100.00"));
        assert!(csv.contains("Space,1"));
        assert!(csv.contains("左クリック,1"));
    }

    #[test]
    fn test_save_and_load_json() {
        let dir = tempdir().expect("tempdir failed");
        let json_path = dir.path().join("typect.json");

        let mut data = TypeCtData {
            daily_goal_keys: 5000,
            theme: ThemeMode::Dark,
            ..Default::default()
        };
        data.record_key("Space", 10);
        data.record_mouse_move(35.0);

        data.save_to_file(&json_path).expect("save should succeed");
        assert!(json_path.exists());

        let loaded = TypeCtData::load_or_default(&json_path);
        assert_eq!(loaded.total_keys, 1);
        assert_eq!(loaded.daily_goal_keys, 5000);
        assert_eq!(loaded.theme, ThemeMode::Dark);
        assert_eq!(loaded.total_mouse_distance_m, 35.0);
    }

    #[test]
    fn test_daily_history_rollover_and_csv() {
        let mut data = TypeCtData {
            today_keys: 1500,
            today_clicks: 300,
            today_mouse_distance_m: 85.5,
            last_date: "2026-09-14".to_string(),
            ..Default::default()
        };

        // 9/14 -> 9/15 へ日付またぎ
        data.check_date_rollover_with_date("2026-09-15");

        // 9/14 分が daily_history に記録されていること
        assert_eq!(data.daily_history.len(), 1);
        let h14 = data
            .daily_history
            .get("2026-09-14")
            .expect("9/14 entry exists");
        assert_eq!(h14.keys, 1500);
        assert_eq!(h14.clicks, 300);
        assert_eq!(h14.mouse_distance_m, 85.5);

        // 9/15 に入力を追加
        data.record_key("Enter", 9);
        data.record_click("左クリック");
        data.record_mouse_move(14.5);

        // combined_history には今日 (9/15) も含まれること
        let combined = data.get_combined_history();
        assert_eq!(combined.len(), 2);
        assert_eq!(combined.get("2026-09-15").unwrap().keys, 1);
        assert_eq!(combined.get("2026-09-15").unwrap().clicks, 1);
        assert_eq!(combined.get("2026-09-15").unwrap().mouse_distance_m, 14.5);

        // CSV出力に日毎サマリー履歴が含まれること
        let csv = data.export_to_csv();
        assert!(csv.contains("# 日毎サマリー履歴"));
        assert!(csv.contains("2026-09-15,1,1,14.50"));
        assert!(csv.contains("2026-09-14,1500,300,85.50"));
    }
}
