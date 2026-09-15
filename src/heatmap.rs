//! キーボードヒートマップ描画モジュール
//!
//! キーボード配列（QWERTY）をビジュアル化し、
//! 各キーの打鍵頻度に応じたグラデーションカラー（ヒートマップ）をレンダリングします。

use eframe::egui::{self, Color32, Pos2, Vec2};
use std::collections::HashMap;

/// キーボードレイアウト上の単一キー定義
#[derive(Debug, Clone)]
pub struct KeyDef {
    /// キーの識別ラベル（`key_counts` マップとの照合用）
    pub id: &'static str,
    /// 画面表示用ラベル
    pub label: &'static str,
    /// キーの横幅倍率（通常キーは 1.0, Spaceは 6.0 等）
    pub width_units: f32,
}

impl KeyDef {
    pub const fn new(id: &'static str, label: &'static str, width_units: f32) -> Self {
        Self {
            id,
            label,
            width_units,
        }
    }
}

/// 標準的な QWERTY キーボードの行別レイアウトを取得します。
pub fn get_keyboard_layout() -> Vec<Vec<KeyDef>> {
    vec![
        // 行 1: ファンクション行
        vec![
            KeyDef::new("Esc", "Esc", 1.0),
            KeyDef::new("F1", "F1", 1.0),
            KeyDef::new("F2", "F2", 1.0),
            KeyDef::new("F3", "F3", 1.0),
            KeyDef::new("F4", "F4", 1.0),
            KeyDef::new("F5", "F5", 1.0),
            KeyDef::new("F6", "F6", 1.0),
            KeyDef::new("F7", "F7", 1.0),
            KeyDef::new("F8", "F8", 1.0),
            KeyDef::new("F9", "F9", 1.0),
            KeyDef::new("F10", "F10", 1.0),
            KeyDef::new("F11", "F11", 1.0),
            KeyDef::new("F12", "F12", 1.0),
            KeyDef::new("Delete", "Del", 1.0),
        ],
        // 行 2: 数字行
        vec![
            KeyDef::new("`", "`", 1.0),
            KeyDef::new("1", "1", 1.0),
            KeyDef::new("2", "2", 1.0),
            KeyDef::new("3", "3", 1.0),
            KeyDef::new("4", "4", 1.0),
            KeyDef::new("5", "5", 1.0),
            KeyDef::new("6", "6", 1.0),
            KeyDef::new("7", "7", 1.0),
            KeyDef::new("8", "8", 1.0),
            KeyDef::new("9", "9", 1.0),
            KeyDef::new("0", "0", 1.0),
            KeyDef::new("-", "-", 1.0),
            KeyDef::new("=", "=", 1.0),
            KeyDef::new("Backspace", "Back", 1.8),
        ],
        // 行 3: QWERTY 行
        vec![
            KeyDef::new("Tab", "Tab", 1.4),
            KeyDef::new("Q", "Q", 1.0),
            KeyDef::new("W", "W", 1.0),
            KeyDef::new("E", "E", 1.0),
            KeyDef::new("R", "R", 1.0),
            KeyDef::new("T", "T", 1.0),
            KeyDef::new("Y", "Y", 1.0),
            KeyDef::new("U", "U", 1.0),
            KeyDef::new("I", "I", 1.0),
            KeyDef::new("O", "O", 1.0),
            KeyDef::new("P", "P", 1.0),
            KeyDef::new("[", "[", 1.0),
            KeyDef::new("]", "]", 1.0),
            KeyDef::new("\\", "\\", 1.4),
        ],
        // 行 4: ASDF 行
        vec![
            KeyDef::new("CapsLock", "Caps", 1.7),
            KeyDef::new("A", "A", 1.0),
            KeyDef::new("S", "S", 1.0),
            KeyDef::new("D", "D", 1.0),
            KeyDef::new("F", "F", 1.0),
            KeyDef::new("G", "G", 1.0),
            KeyDef::new("H", "H", 1.0),
            KeyDef::new("J", "J", 1.0),
            KeyDef::new("K", "K", 1.0),
            KeyDef::new("L", "L", 1.0),
            KeyDef::new(";", ";", 1.0),
            KeyDef::new("'", "'", 1.0),
            KeyDef::new("Enter", "Enter", 2.1),
        ],
        // 行 5: ZXCV 行
        vec![
            KeyDef::new("Shift", "Shift", 2.2),
            KeyDef::new("Z", "Z", 1.0),
            KeyDef::new("X", "X", 1.0),
            KeyDef::new("C", "C", 1.0),
            KeyDef::new("V", "V", 1.0),
            KeyDef::new("B", "B", 1.0),
            KeyDef::new("N", "N", 1.0),
            KeyDef::new("M", "M", 1.0),
            KeyDef::new(",", ",", 1.0),
            KeyDef::new(".", ".", 1.0),
            KeyDef::new("/", "/", 1.0),
            KeyDef::new("Shift", "Shift", 2.6),
        ],
        // 行 6: スペース / 最下行
        vec![
            KeyDef::new("Ctrl", "Ctrl", 1.4),
            KeyDef::new("Win", "Win", 1.2),
            KeyDef::new("Alt", "Alt", 1.2),
            KeyDef::new("Space", "Space", 6.2),
            KeyDef::new("Alt", "Alt", 1.2),
            KeyDef::new("Ctrl", "Ctrl", 1.4),
            KeyDef::new("←", "←", 1.0),
            KeyDef::new("↑", "↑", 1.0),
            KeyDef::new("↓", "↓", 1.0),
            KeyDef::new("→", "→", 1.0),
        ],
    ]
}

/// 打鍵比率 (0.0 .. 1.0) をヒートマップカラー (暗青 -> シアン -> 緑 -> 黄 -> 赤) に変換します。
pub fn ratio_to_heatmap_color(ratio: f32) -> Color32 {
    let r = ratio.clamp(0.0, 1.0);
    if r <= 0.0 {
        // 未使用キー: 落ち着いたダークスレート
        return Color32::from_rgb(30, 41, 59);
    }

    if r < 0.25 {
        // 0.0 .. 0.25 : 青 (0, 100, 220) -> シアン (0, 220, 220)
        let t = r / 0.25;
        let g = (100.0 + t * 120.0) as u8;
        Color32::from_rgb(0, g, 220)
    } else if r < 0.5 {
        // 0.25 .. 0.5 : シアン (0, 220, 220) -> 緑 (34, 197, 94)
        let t = (r - 0.25) / 0.25;
        let red = (t * 34.0) as u8;
        let b = (220.0 - t * 126.0) as u8;
        Color32::from_rgb(red, 200, b)
    } else if r < 0.75 {
        // 0.5 .. 0.75 : 緑 (34, 197, 94) -> 黄 (234, 179, 8)
        let t = (r - 0.5) / 0.25;
        let red = (34.0 + t * 200.0) as u8;
        let g = (197.0 - t * 18.0) as u8;
        let b = (94.0 - t * 86.0) as u8;
        Color32::from_rgb(red, g, b)
    } else {
        // 0.75 .. 1.0 : 黄 (234, 179, 8) -> 燃える赤 (239, 68, 68)
        let t = (r - 0.75) / 0.25;
        let red = (234.0 + t * 5.0) as u8;
        let g = (179.0 - t * 111.0) as u8;
        let b = (8.0 + t * 60.0) as u8;
        Color32::from_rgb(red, g, b)
    }
}

/// キーボードヒートマップを描画します。
pub fn render_keyboard_heatmap(ui: &mut egui::Ui, key_counts: &HashMap<String, u64>) {
    let max_count = key_counts.values().copied().max().unwrap_or(1).max(1) as f32;
    let layout = get_keyboard_layout();

    let unit_size = 32.0;
    let spacing = 3.0;

    ui.vertical(|ui| {
        ui.label(
            egui::RichText::new("⌨ キーボードヒートマップ (打鍵頻度分布)")
                .strong()
                .size(15.0),
        );
        ui.label(
            egui::RichText::new("※ 打鍵数が多いキーほど暖色系（赤・黄）、少ないキーは寒色系（青・シアン）で表示されます。")
                .size(11.0)
                .color(Color32::from_rgb(148, 163, 184)),
        );
        ui.add_space(8.0);

        for row in layout {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(spacing, spacing);
                for key in row {
                    let count = key_counts.get(key.id).copied().unwrap_or(0);
                    let ratio = (count as f32) / max_count;
                    let fill_color = ratio_to_heatmap_color(ratio);

                    let key_width = key.width_units * unit_size;
                    let (rect, response) =
                        ui.allocate_exact_size(Vec2::new(key_width, 32.0), egui::Sense::hover());

                    let painter = ui.painter();
                    // キー背景
                    painter.rect_filled(rect, 4.0, fill_color);
                    // キー枠線
                    painter.rect_stroke(
                        rect,
                        4.0,
                        egui::Stroke::new(1.0, Color32::from_rgb(71, 85, 105)),
                    );

                    // テキスト色（背景の明るさに応じて調整）
                    let text_color = if ratio > 0.4 {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    };

                    // キーラベル
                    painter.text(
                        Pos2::new(rect.center().x, rect.center().y - 4.0),
                        egui::Align2::CENTER_CENTER,
                        key.label,
                        egui::FontId::proportional(11.0),
                        text_color,
                    );

                    // カウント数
                    if count > 0 {
                        painter.text(
                            Pos2::new(rect.center().x, rect.center().y + 8.0),
                            egui::Align2::CENTER_CENTER,
                            format!("{}", count),
                            egui::FontId::monospace(8.5),
                            text_color,
                        );
                    }

                    // ホバー時の詳細ツールチップ
                    response.on_hover_ui(|ui| {
                        ui.label(format!("キー: {}", key.label));
                        ui.label(format!("打鍵数: {} 回", count));
                        ui.label(format!("最高頻度比: {:.1}%", ratio * 100.0));
                    });
                }
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratio_to_heatmap_color() {
        let col_zero = ratio_to_heatmap_color(0.0);
        assert_eq!(col_zero, Color32::from_rgb(30, 41, 59));

        let col_max = ratio_to_heatmap_color(1.0);
        // 赤系
        assert!(col_max.r() > 200);

        let col_mid = ratio_to_heatmap_color(0.5);
        // 緑〜黄系
        assert!(col_mid.g() > 150);
    }

    #[test]
    fn test_keyboard_layout_integrity() {
        let layout = get_keyboard_layout();
        assert_eq!(layout.len(), 6);
        assert!(!layout[0].is_empty());
    }
}
