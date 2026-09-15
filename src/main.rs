//! # TypeCT - 打鍵・マウスクリック・マウス移動距離計測デスクトップアプリ
//!
//! Windows 11 (64bit) ネイティブ対応の Rust + egui デスクトップアプリケーション。

// Windows リリースビルド時にコンソール画面（黒い画面）がポップアップするのを抑制する設定
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use single_instance::SingleInstance;
use std::sync::mpsc::{channel, Receiver, Sender};
use typect_lib::app::TypeCtApp;
use typect_lib::input::{
    calculate_euclidean_distance, map_mouse_button, map_rdev_key, pixel_distance_to_meters,
    InputHookMessage,
};

fn main() -> eframe::Result<()> {
    // 1. 二重起動の防止 (Single Instance)
    // 既に起動している場合は後から起動したプロセスを静かに即終了
    let instance = SingleInstance::new("TypeCT_Windows11_SingleInstance_App_Lock")
        .expect("Failed to initialize single-instance mutex");
    if !instance.is_single() {
        return Ok(());
    }

    // 2. rdev による低遅延グローバル入力フックスレッドの生成
    let (tx, rx): (Sender<InputHookMessage>, Receiver<InputHookMessage>) = channel();
    std::thread::spawn(move || {
        let mut last_mouse_pos: Option<(f64, f64)> = None;

        let _ = rdev::listen(move |event| match event.event_type {
            rdev::EventType::KeyPress(key) => {
                let name = map_rdev_key(key).to_string();
                let _ = tx.send(InputHookMessage::Key(name));
            }
            rdev::EventType::ButtonPress(btn) => {
                let name = map_mouse_button(btn).to_string();
                let _ = tx.send(InputHookMessage::Mouse(name));
            }
            rdev::EventType::MouseMove { x, y } => {
                if let Some((prev_x, prev_y)) = last_mouse_pos {
                    let px_dist = calculate_euclidean_distance(prev_x, prev_y, x, y);
                    // わずかなジッターを除外（1px以上移動時）
                    if px_dist >= 1.0 {
                        let meters = pixel_distance_to_meters(px_dist);
                        let _ = tx.send(InputHookMessage::MouseMove(meters));
                    }
                }
                last_mouse_pos = Some((x, y));
            }
            _ => {}
        });
    });

    // 3. eframe GUI の初期設定と起動
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("TypeCT - タイプ数カウンター")
            .with_inner_size([780.0, 600.0])
            .with_min_inner_size([580.0, 460.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TypeCT",
        native_options,
        Box::new(|cc| Ok(Box::new(TypeCtApp::new(cc, rx)))),
    )
}
