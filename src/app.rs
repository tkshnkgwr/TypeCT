//! GUI アプリケーション本体モジュール (`TypeCtApp`)
//!
//! `eframe` / `egui` (0.29+) を用いたモダンな統計ダッシュボード描画、
//! `egui_plot::BarChart` による時間帯別グラフ、主要キーランキング、
//! キーボードヒートマップ、目標打鍵数トースト通知、テーマ切替、
//! Windows スタートアップ登録、CSV エクスポート、および省電力設計を提供します。

use crate::font::setup_japanese_fonts;
use crate::heatmap::render_keyboard_heatmap;
use crate::input::InputHookMessage;
use crate::model::{format_distance, ThemeMode, TypeCtData};
use crate::notify::send_goal_reached_notification;
use crate::startup::{is_startup_enabled, set_startup_enabled};
use crate::tray::create_embedded_tray_icon;
use chrono::{Local, Timelike};
use eframe::egui;
use egui_plot::{Bar, BarChart, Plot};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

/// ナビゲーションタブ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppTab {
    /// 統計ダッシュボード
    #[default]
    Dashboard,
    /// キーボードヒートマップ
    Heatmap,
    /// 環境設定
    Settings,
}

/// TypeCT メインアプリケーション構造体
pub struct TypeCtApp {
    /// 計測データモデル
    pub data: TypeCtData,
    /// 保存先 JSON パス
    pub json_path: PathBuf,
    /// 最終自動保存時刻
    pub last_save_time: Instant,
    /// 最終トレイツールチップ更新時刻
    pub last_tooltip_update: Instant,
    /// 入力フックスレッドからのメッセージ受信用レシーバー
    pub input_receiver: Receiver<InputHookMessage>,
    /// タスクトレイアイコンインスタンス
    pub tray_icon: Option<tray_icon::TrayIcon>,
    /// ウィンドウが表示されているかどうかのフラグ
    pub is_window_visible: bool,
    /// ステータスバー表示文字列
    pub status_message: String,
    /// 現在選択中のタブ
    pub current_tab: AppTab,
}

impl TypeCtApp {
    /// 新規 `TypeCtApp` インスタンスを構築します。
    pub fn new(cc: &eframe::CreationContext<'_>, rx: Receiver<InputHookMessage>) -> Self {
        // 日本語フォントを適用
        setup_japanese_fonts(&cc.egui_ctx);

        let base_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let json_path = base_dir.join("typect.json");

        // 既存の typect.json があればロード、なければ新規作成
        let mut data = TypeCtData::load_or_default(&json_path);

        // 旧バイナリ (data / data2) が存在する場合は自動インポート
        data.try_import_legacy(&base_dir);
        data.check_date_rollover();

        // 実際のレジストリ状態と設定の同期
        data.auto_start = is_startup_enabled();

        // カラーテーマを適用
        apply_theme(&cc.egui_ctx, data.theme);

        // タスクトレイアイコンの構築
        let tray_icon = if let Some(icon) = create_embedded_tray_icon() {
            tray_icon::TrayIconBuilder::new()
                .with_tooltip(format!(
                    "TypeCT - 今日: {}打 / {}クリック / {}",
                    data.today_keys,
                    data.today_clicks,
                    format_distance(data.today_mouse_distance_m)
                ))
                .with_icon(icon)
                .build()
                .ok()
        } else {
            None
        };

        Self {
            data,
            json_path,
            last_save_time: Instant::now(),
            last_tooltip_update: Instant::now(),
            input_receiver: rx,
            tray_icon,
            is_window_visible: true,
            status_message: "計測中（常駐中）".to_string(),
            current_tab: AppTab::Dashboard,
        }
    }

    /// JSONファイルへの安全な書き出し
    pub fn save_to_disk(&mut self) {
        self.data.sync_today_to_history();
        if self.data.save_to_file(&self.json_path).is_ok() {
            self.last_save_time = Instant::now();
        }
    }

    /// トレイアイコンのツールチップを動的に更新（最短1秒間隔）
    pub fn update_tray_tooltip(&mut self) {
        if self.last_tooltip_update.elapsed() >= Duration::from_secs(1) {
            if let Some(tray) = &mut self.tray_icon {
                let tooltip = format!(
                    "TypeCT - 今日: {}打 / {}クリック / {}",
                    self.data.today_keys,
                    self.data.today_clicks,
                    format_distance(self.data.today_mouse_distance_m)
                );
                let _ = tray.set_tooltip(Some(tooltip));
            }
            self.last_tooltip_update = Instant::now();
        }
    }

    /// CSV ファイルのエクスポートを実行
    pub fn export_csv(&mut self) {
        let base_dir = self
            .json_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        let file_name = format!("typect_export_{}.csv", Local::now().format("%Y%m%d_%H%M%S"));
        let export_path = base_dir.join(&file_name);

        match self.data.export_csv_to_file(&export_path) {
            Ok(_) => {
                self.status_message = format!("CSVを出力しました: {}", file_name);
            }
            Err(e) => {
                self.status_message = format!("CSV出力に失敗しました: {}", e);
            }
        }
    }
}

/// カラーテーマを `egui::Context` に適用します。
fn apply_theme(ctx: &egui::Context, theme: ThemeMode) {
    match theme {
        ThemeMode::Dark => ctx.set_visuals(egui::Visuals::dark()),
        ThemeMode::Light => ctx.set_visuals(egui::Visuals::light()),
        ThemeMode::System => ctx.set_visuals(egui::Visuals::default()),
    }
}

impl eframe::App for TypeCtApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 日付またぎチェック
        self.data.check_date_rollover();

        // 1. チャネルから入力イベントをノンブロッキングですべて受信して加算
        let current_hour = Local::now().hour() as usize;
        let mut received_any = false;

        while let Ok(msg) = self.input_receiver.try_recv() {
            received_any = true;
            match msg {
                InputHookMessage::Key(k) => {
                    self.data.record_key(&k, current_hour);
                }
                InputHookMessage::Mouse(btn) => {
                    self.data.record_click(&btn);
                }
                InputHookMessage::MouseMove(dist_m) => {
                    self.data.record_mouse_move(dist_m);
                }
            }
        }

        // 2. 目標打鍵数達成判定（初回到達時にトースト通知を発行）
        if self.data.check_goal_reached() {
            send_goal_reached_notification(self.data.today_keys);
            self.status_message = format!(
                "🎉 本日の目標打鍵数（{}打）を達成しました！",
                self.data.today_keys
            );
        }

        // 3. タスクトレイイベントの検知（クリックされたらウィンドウを復帰・再表示）
        if let Ok(tray_icon::TrayIconEvent::Click {
            button: tray_icon::MouseButton::Left,
            ..
        }) = tray_icon::TrayIconEvent::receiver().try_recv()
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            self.is_window_visible = true;
        }

        // 4. ウィンドウ最小化・閉じるボタン処理（トレイへ格納）
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            self.is_window_visible = false;
            self.save_to_disk();
        }

        // 5. 定期自動保存（5分毎）
        if self.last_save_time.elapsed() >= Duration::from_secs(300) {
            self.save_to_disk();
        }

        // 6. 動的ツールチップ更新
        self.update_tray_tooltip();

        // 7. GUI 画面描画
        egui::TopBottomPanel::top("top_header").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.heading("⌨ TypeCT (タイプ数カウンター)");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❌ 終了").clicked() {
                        self.save_to_disk();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("📥 トレイに格納").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                        self.is_window_visible = false;
                    }
                    if ui.button("💾 保存").clicked() {
                        self.save_to_disk();
                        self.status_message = "保存完了".to_string();
                    }
                    if ui.button("📊 CSV出力").clicked() {
                        self.export_csv();
                    }

                    // テーマ切り替えショートカット
                    let theme_btn_label = match self.data.theme {
                        ThemeMode::System => "💻",
                        ThemeMode::Dark => "🌙",
                        ThemeMode::Light => "☀️",
                    };
                    if ui
                        .button(theme_btn_label)
                        .on_hover_text("カラーテーマ切り替え (システム / ダーク / ライト)")
                        .clicked()
                    {
                        self.data.theme = match self.data.theme {
                            ThemeMode::System => ThemeMode::Dark,
                            ThemeMode::Dark => ThemeMode::Light,
                            ThemeMode::Light => ThemeMode::System,
                        };
                        apply_theme(ctx, self.data.theme);
                        self.save_to_disk();
                    }
                });
            });

            ui.add_space(4.0);

            // タブセレクター
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.current_tab,
                    AppTab::Dashboard,
                    "📊 ダッシュボード",
                );
                ui.selectable_value(
                    &mut self.current_tab,
                    AppTab::Heatmap,
                    "⌨ キーボードヒートマップ",
                );
                ui.selectable_value(&mut self.current_tab, AppTab::Settings, "⚙ 設定");
            });

            ui.add_space(2.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.current_tab {
            AppTab::Dashboard => self.render_dashboard_tab(ui),
            AppTab::Heatmap => render_keyboard_heatmap(ui, &self.data.key_counts),
            AppTab::Settings => self.render_settings_tab(ui, ctx),
        });

        egui::TopBottomPanel::bottom("bottom_status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("ステータス: {}", self.status_message));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("保存先: {}", self.json_path.display()));
                });
            });
        });

        // 省電力設計
        if self.is_window_visible {
            if received_any {
                ctx.request_repaint();
            } else {
                ctx.request_repaint_after(Duration::from_millis(500));
            }
        }
    }

    /// アプリ終了時の確実な自動保存
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_to_disk();
    }
}

impl TypeCtApp {
    /// ダッシュボードタブの描画
    fn render_dashboard_tab(&mut self, ui: &mut egui::Ui) {
        let avg_keys = self.data.average_daily_keys();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // 1. サマリーカード表示（今日、昨日、トータル、1日平均）
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("本日の打鍵数").strong());
                        ui.label(
                            egui::RichText::new(format!("{} 打", self.data.today_keys))
                                .size(24.0)
                                .color(egui::Color32::from_rgb(56, 189, 248)),
                        );
                        ui.label(format!("クリック: {} 回", self.data.today_clicks));
                        ui.label(format!(
                            "移動距離: {}",
                            format_distance(self.data.today_mouse_distance_m)
                        ));
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("昨日の実績").strong());
                        ui.label(
                            egui::RichText::new(format!("{} 打", self.data.yesterday_keys))
                                .size(20.0)
                                .color(egui::Color32::from_rgb(148, 163, 184)),
                        );
                        ui.label(format!("クリック: {} 回", self.data.yesterday_clicks));
                        ui.label(format!(
                            "移動距離: {}",
                            format_distance(self.data.yesterday_mouse_distance_m)
                        ));
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("累計トータル").strong());
                        ui.label(
                            egui::RichText::new(format!("{} 打", self.data.total_keys))
                                .size(20.0)
                                .color(egui::Color32::from_rgb(74, 222, 128)),
                        );
                        ui.label(format!("クリック累計: {} 回", self.data.total_clicks));
                        ui.label(format!(
                            "総移動距離: {}",
                            format_distance(self.data.total_mouse_distance_m)
                        ));
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("記録日数 / 1日平均").strong());
                        ui.label(
                            egui::RichText::new(format!("{} 日間", self.data.days_recorded))
                                .size(20.0),
                        );
                        ui.label(format!("平均: {} 打/日", avg_keys));
                        let goal_ratio = if self.data.daily_goal_keys > 0 {
                            (self.data.today_keys as f32 / self.data.daily_goal_keys as f32)
                                .min(1.0)
                        } else {
                            1.0
                        };
                        ui.label(format!("目標: {}% 達成", (goal_ratio * 100.0) as u32));
                    });
                });
            });

            // 目標達成プログレスバー
            if self.data.daily_goal_keys > 0 {
                ui.add_space(6.0);
                let goal_ratio =
                    (self.data.today_keys as f32 / self.data.daily_goal_keys as f32).min(1.0);
                let progress_text = format!(
                    "本日の目標達成度: {} / {} 打 ({:.1}%)",
                    self.data.today_keys,
                    self.data.daily_goal_keys,
                    goal_ratio * 100.0
                );
                let pbar = egui::ProgressBar::new(goal_ratio)
                    .text(progress_text)
                    .animate(false);
                ui.add(pbar);
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // 直近24時間の時間帯別打鍵数グラフ (egui_plot::BarChart)
            ui.label(
                egui::RichText::new("直近24時間の時間帯別打鍵数")
                    .strong()
                    .size(15.0),
            );
            let chart_bars: Vec<Bar> = self
                .data
                .hourly_keys
                .iter()
                .enumerate()
                .map(|(hour, &count)| {
                    Bar::new(hour as f64, count as f64)
                        .width(0.7)
                        .fill(egui::Color32::from_rgb(14, 165, 233))
                })
                .collect();

            let bar_chart = BarChart::new(chart_bars).name("時間帯打鍵数");
            Plot::new("hourly_plot")
                .height(130.0)
                .include_y(0.0)
                .include_x(0.0)
                .include_x(23.0)
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(bar_chart);
                });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // 主要キー別ランキング
            ui.label(
                egui::RichText::new("キー別打鍵数ランキング (Top 12)")
                    .strong()
                    .size(15.0),
            );

            let mut sorted_keys: Vec<(&String, &u64)> = self.data.key_counts.iter().collect();
            sorted_keys.sort_by(|a, b| b.1.cmp(a.1));

            let max_key_count = sorted_keys.first().map(|(_, &c)| c).unwrap_or(1).max(1);

            for (rank, (k, &count)) in sorted_keys.iter().take(12).enumerate() {
                let ratio = (count as f32) / (max_key_count as f32);
                ui.horizontal(|ui| {
                    ui.label(format!("{:2}. {:<10}", rank + 1, k));
                    let progress = egui::ProgressBar::new(ratio)
                        .text(format!("{} 回", count))
                        .animate(false);
                    ui.add_sized([280.0, 16.0], progress);
                });
            }

            // 日別サマリー履歴（直近14日）
            let combined_history = self.data.get_combined_history();
            if !combined_history.is_empty() {
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("📅 日別サマリー履歴 (直近)")
                        .strong()
                        .size(15.0),
                );

                egui::Grid::new("daily_history_grid")
                    .striped(true)
                    .min_col_width(90.0)
                    .spacing([12.0, 4.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("日付").strong());
                        ui.label(egui::RichText::new("打鍵数").strong());
                        ui.label(egui::RichText::new("クリック").strong());
                        ui.label(egui::RichText::new("移動距離").strong());
                        ui.end_row();

                        for (date, summary) in combined_history.iter().rev().take(14) {
                            ui.label(date);
                            ui.label(format!("{} 打", summary.keys));
                            ui.label(format!("{} 回", summary.clicks));
                            ui.label(format_distance(summary.mouse_distance_m));
                            ui.end_row();
                        }
                    });
            }

            if self.data.legacy_imported {
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(
                        "※ 旧 typect (data / data2) からの累計データを引き継ぎ済み",
                    )
                    .color(egui::Color32::from_rgb(16, 185, 129))
                    .size(11.0),
                );
            }
        });
    }

    /// 設定タブの描画
    fn render_settings_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical(|ui| {
            ui.heading("⚙ 環境設定");
            ui.add_space(10.0);

            // 1. スタートアップ自動起動設定
            ui.group(|ui| {
                ui.label(egui::RichText::new("🚀 Windows スタートアップ自動起動").strong());
                let mut auto_start = self.data.auto_start;
                if ui
                    .checkbox(&mut auto_start, "PC 起動時に TypeCT を自動的に起動する")
                    .changed()
                {
                    match set_startup_enabled(auto_start) {
                        Ok(_) => {
                            self.data.auto_start = auto_start;
                            self.status_message = if auto_start {
                                "スタートアップ登録を有効化しました".to_string()
                            } else {
                                "スタートアップ登録を解除しました".to_string()
                            };
                            self.save_to_disk();
                        }
                        Err(e) => {
                            self.status_message = format!("スタートアップ設定変更エラー: {}", e);
                        }
                    }
                }
                ui.label(
                    egui::RichText::new("※ Windows レジストリ (HKCU\\...\\Run) に登録されます。")
                        .size(11.0)
                        .color(egui::Color32::from_rgb(148, 163, 184)),
                );
            });

            ui.add_space(10.0);

            // 2. 目標打鍵数設定
            ui.group(|ui| {
                ui.label(egui::RichText::new("🎯 1日の目標打鍵数とトースト通知").strong());
                ui.horizontal(|ui| {
                    ui.label("目標打鍵数:");
                    let mut goal = self.data.daily_goal_keys as i64;
                    if ui
                        .add(egui::Slider::new(&mut goal, 1000..=50000).step_by(500.0))
                        .changed()
                    {
                        self.data.daily_goal_keys = goal.max(0) as u64;
                        self.save_to_disk();
                    }
                    ui.label("打/日");
                });
                ui.label(
                    egui::RichText::new(
                        "※ 本日の打鍵数がこの数値に到達した際、Windows トースト通知でお知らせします。",
                    )
                    .size(11.0)
                    .color(egui::Color32::from_rgb(148, 163, 184)),
                );
            });

            ui.add_space(10.0);

            // 3. テーマ切り替え
            ui.group(|ui| {
                ui.label(egui::RichText::new("🎨 カラーテーマ").strong());
                ui.horizontal(|ui| {
                    let mut theme = self.data.theme;
                    let changed = ui
                        .selectable_value(&mut theme, ThemeMode::System, "💻 システム設定")
                        .clicked()
                        || ui
                            .selectable_value(&mut theme, ThemeMode::Dark, "🌙 ダーク")
                            .clicked()
                        || ui
                            .selectable_value(&mut theme, ThemeMode::Light, "☀️ ライト")
                            .clicked();

                    if changed {
                        self.data.theme = theme;
                        apply_theme(ctx, theme);
                        self.save_to_disk();
                    }
                });
            });

            ui.add_space(10.0);

            // 4. データ出力
            ui.group(|ui| {
                ui.label(egui::RichText::new("📁 データエクスポート").strong());
                if ui.button("📊 CSV ファイルに出力").clicked() {
                    self.export_csv();
                }
                ui.label(
                    egui::RichText::new(
                        "※ キー別ランキング、時間帯別打鍵数、日別統計を CSV 形式で保存します。",
                    )
                    .size(11.0)
                    .color(egui::Color32::from_rgb(148, 163, 184)),
                );
            });
        });
    }
}
