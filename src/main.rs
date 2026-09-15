//! # TypeCT - 打鍵・マウスクリック数計測デスクトップアプリ
//!
//! Windows 11 (64bit) ネイティブ対応の Rust + egui デスクトップアプリケーション。

// Windows リリースビルド時にコンソール画面（黒い画面）がポップアップするのを抑制する設定
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use single_instance::SingleInstance;
use std::sync::mpsc::{channel, Receiver, Sender};
use typect_lib::app::TypeCtApp;
use typect_lib::input::{map_mouse_button, map_rdev_key, InputHookMessage};

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
        let _ = rdev::listen(move |event| match event.event_type {
            rdev::EventType::KeyPress(key) => {
                let name = map_rdev_key(key).to_string();
                let _ = tx.send(InputHookMessage::Key(name));
            }
            rdev::EventType::ButtonPress(btn) => {
                let name = map_mouse_button(btn).to_string();
                let _ = tx.send(InputHookMessage::Mouse(name));
            }
            _ => {}
        });
    });

    // 3. eframe GUI の初期設定と起動
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("TypeCT - タイプ数カウンター")
            .with_inner_size([720.0, 560.0])
            .with_min_inner_size([540.0, 420.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TypeCT",
        native_options,
        Box::new(|cc| Ok(Box::new(TypeCtApp::new(cc, rx)))),
    )
}
