//! 日本語フォントロード & 豆腐化防止モジュール
//!
//! Windows OS に標準搭載されている日本語フォントを自動探索して
//! `egui::FontDefinitions` の最優先フォントとして登録し、
//! 日本語文字の表示欠落（豆腐化: □）を完全に防止します。

use eframe::egui;
use std::fs;

/// Windows 標準の日本語フォント（游ゴシック、メイリオ、MSゴシック等）を探索し、`egui` のフォント定義へ登録します。
///
/// 探索優先順位:
/// 1. `C:\Windows\Fonts\YuGothM.ttc` (游ゴシック Medium)
/// 2. `C:\Windows\Fonts\meiryo.ttc` (メイリオ)
/// 3. `C:\Windows\Fonts\msgothic.ttc` (MS ゴシック)
/// 4. `C:\Windows\Fonts\YuGothR.ttc` (游ゴシック Regular)
/// 5. `C:\Windows\Fonts\msmincho.ttc` (MS 明朝)
///
/// # 引数
/// * `ctx` - `egui::Context` への参照
///
/// # 戻り値
/// * `bool` - 日本語フォントの読み込みに成功した場合 `true`
pub fn setup_japanese_fonts(ctx: &egui::Context) -> bool {
    let mut fonts = egui::FontDefinitions::default();

    let font_candidates = [
        r"C:\Windows\Fonts\YuGothM.ttc",
        r"C:\Windows\Fonts\meiryo.ttc",
        r"C:\Windows\Fonts\msgothic.ttc",
        r"C:\Windows\Fonts\YuGothR.ttc",
        r"C:\Windows\Fonts\msmincho.ttc",
    ];

    let mut loaded = false;
    for path in font_candidates {
        if let Ok(data) = fs::read(path) {
            fonts.font_data.insert(
                "windows_japanese".to_owned(),
                egui::FontData::from_owned(data),
            );
            // Proportional（標準テキスト）および Monospace（等幅）の最優先フォントに設定
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "windows_japanese".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "windows_japanese".to_owned());
            loaded = true;
            break;
        }
    }

    if loaded {
        ctx.set_fonts(fonts);
    }

    loaded
}
