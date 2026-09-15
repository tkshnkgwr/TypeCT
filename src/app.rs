//! GUI アプリケーション本体モジュール (`TypeCtApp`)
//!
//! `eframe` / `egui` (0.29+) を用いたモダンな統計ダッシュボード描画、
//! `egui_plot::BarChart` による時間帯別グラフ、主要キーランキング、
//! タスクトレイ常駐制御、および省電力設計を提供します。

use crate::font::setup_japanese_fonts;
use crate::input::InputHookMessage;
use crate::model::TypeCtData;
use crate::tray::create_embedded_tray_icon;
use chrono::{Local, Timelike};
use eframe::egui;
use egui_plot::{Bar, BarChart, Plot};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

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
}

impl TypeCtApp {
    /// 新規 `TypeCtApp` インスタンスを構築します。
    ///
    /// 日本語フォントを自動登録し、同一フォルダの `typect.json` をロード
    /// （存在しない場合は旧 `data`/`data2` の自動インポートを試行）します。
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

        // タスクトレイアイコンの構築
        let tray_icon = if let Some(icon) = create_embedded_tray_icon() {
            tray_icon::TrayIconBuilder::new()
                .with_tooltip(format!(
                    "TypeCT - 今日: {}打 / {}クリック",
                    data.today_keys, data.today_clicks
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
        }
    }

    /// JSONファイルへの安全な書き出し
    pub fn save_to_disk(&mut self) {
        if self.data.save_to_file(&self.json_path).is_ok() {
            self.last_save_time = Instant::now();
        }
    }

    /// トレイアイコンのツールチップを動的に更新（最短1秒間隔）
    pub fn update_tray_tooltip(&mut self) {
        if self.last_tooltip_update.elapsed() >= Duration::from_secs(1) {
            if let Some(tray) = &mut self.tray_icon {
                let tooltip = format!(
                    "TypeCT - 今日: {}打 / {}クリック",
                    self.data.today_keys, self.data.today_clicks
                );
                let _ = tray.set_tooltip(Some(tooltip));
            }
            self.last_tooltip_update = Instant::now();
        }
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
            }
        }

        // 2. タスクトレイイベントの検知（クリックされたらウィンドウを復帰・再表示）
        if let Ok(tray_icon::TrayIconEvent::Click {
            button: tray_icon::MouseButton::Left,
            ..
        }) = tray_icon::TrayIconEvent::receiver().try_recv()
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            self.is_window_visible = true;
        }

        // 3. ウィンドウ最小化・閉じるボタン処理（トレイへ格納）
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            self.is_window_visible = false;
            self.save_to_disk();
        }

        // 4. 定期自動保存（5分毎）
        if self.last_save_time.elapsed() >= Duration::from_secs(300) {
            self.save_to_disk();
        }

        // 5. 動的ツールチップ更新
        self.update_tray_tooltip();

        // 6. GUI 画面描画
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
                });
            });
            ui.add_space(4.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // サマリーカード表示（今日、昨日、トータル、1日平均）
            let avg_keys = self.data.average_daily_keys();

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
                    });
                });
            });

            ui.add_space(10.0);
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
                .height(160.0)
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

            egui::ScrollArea::vertical()
                .max_height(140.0)
                .show(ui, |ui| {
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
                });

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

        egui::TopBottomPanel::bottom("bottom_status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("ステータス: {}", self.status_message));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("保存先: {}", self.json_path.display()));
                });
            });
        });

        // 省電力設計:
        // ウィンドウが表示されている場合のみ適度な間隔（500ms）で再描画を要求。
        // 入力イベントがあった場合は即時再描画し、最小化・バックグラウンド時は待機時CPU 0% を維持。
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
