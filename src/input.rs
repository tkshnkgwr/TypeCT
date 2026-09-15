//! 入力イベント & キーマッピングモジュール
//!
//! `rdev` クレートから受け取った低レベルのキー、マウスボタン、マウスカーソル移動イベントを、
//! プライバシーに配慮した安全なメッセージ構造体へマッピングします。
//!
//! # プライバシー & セキュリティ保護
//! 本アプリはキーロガーではありません。入力された文字列や入力内容は一切記録・保持せず、
//! 押下されたキー種別のカウンター加算およびマウス移動距離の積算のみを行います。

/// 非同期入力フックスレッドから GUI スレッドへ送信されるイベント
#[derive(Debug, Clone, PartialEq)]
pub enum InputHookMessage {
    /// キーボード打鍵イベント（キー名称）
    Key(String),
    /// マウスクリックイベント（ボタン名称）
    Mouse(String),
    /// マウスカーソル移動量（メートル単位）
    MouseMove(f64),
}

/// ピクセル移動距離を標準的な画面 DPI (96 DPI: 1 inch = 25.4 mm) に基づいてメートルに換算します。
///
/// 1 px = 0.0254 m / 96 ≈ 0.0002645833 m
#[inline]
pub fn pixel_distance_to_meters(px: f64) -> f64 {
    px * (0.0254 / 96.0)
}

/// 2点間のユークリッド距離（ピクセル）を計算します。
#[inline]
pub fn calculate_euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

/// `rdev::Key` を安全で分かりやすい表示用文字列へマッピングします。
pub fn map_rdev_key(key: rdev::Key) -> &'static str {
    match key {
        rdev::Key::KeyA => "A",
        rdev::Key::KeyB => "B",
        rdev::Key::KeyC => "C",
        rdev::Key::KeyD => "D",
        rdev::Key::KeyE => "E",
        rdev::Key::KeyF => "F",
        rdev::Key::KeyG => "G",
        rdev::Key::KeyH => "H",
        rdev::Key::KeyI => "I",
        rdev::Key::KeyJ => "J",
        rdev::Key::KeyK => "K",
        rdev::Key::KeyL => "L",
        rdev::Key::KeyM => "M",
        rdev::Key::KeyN => "N",
        rdev::Key::KeyO => "O",
        rdev::Key::KeyP => "P",
        rdev::Key::KeyQ => "Q",
        rdev::Key::KeyR => "R",
        rdev::Key::KeyS => "S",
        rdev::Key::KeyT => "T",
        rdev::Key::KeyU => "U",
        rdev::Key::KeyV => "V",
        rdev::Key::KeyW => "W",
        rdev::Key::KeyX => "X",
        rdev::Key::KeyY => "Y",
        rdev::Key::KeyZ => "Z",
        rdev::Key::Num0 => "0",
        rdev::Key::Num1 => "1",
        rdev::Key::Num2 => "2",
        rdev::Key::Num3 => "3",
        rdev::Key::Num4 => "4",
        rdev::Key::Num5 => "5",
        rdev::Key::Num6 => "6",
        rdev::Key::Num7 => "7",
        rdev::Key::Num8 => "8",
        rdev::Key::Num9 => "9",
        rdev::Key::Space => "Space",
        rdev::Key::Return => "Enter",
        rdev::Key::Backspace => "Backspace",
        rdev::Key::Tab => "Tab",
        rdev::Key::ShiftLeft | rdev::Key::ShiftRight => "Shift",
        rdev::Key::ControlLeft | rdev::Key::ControlRight => "Ctrl",
        rdev::Key::Alt | rdev::Key::AltGr => "Alt",
        rdev::Key::Escape => "Esc",
        rdev::Key::Delete => "Delete",
        rdev::Key::UpArrow => "↑",
        rdev::Key::DownArrow => "↓",
        rdev::Key::LeftArrow => "←",
        rdev::Key::RightArrow => "→",
        _ => "Other",
    }
}

/// `rdev::Button` を表示用日本語文字列へマッピングします。
pub fn map_mouse_button(btn: rdev::Button) -> &'static str {
    match btn {
        rdev::Button::Left => "左クリック",
        rdev::Button::Right => "右クリック",
        rdev::Button::Middle => "ホイールクリック",
        _ => "拡張クリック",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_rdev_keys() {
        assert_eq!(map_rdev_key(rdev::Key::KeyA), "A");
        assert_eq!(map_rdev_key(rdev::Key::Space), "Space");
        assert_eq!(map_rdev_key(rdev::Key::Return), "Enter");
    }

    #[test]
    fn test_map_mouse_buttons() {
        assert_eq!(map_mouse_button(rdev::Button::Left), "左クリック");
        assert_eq!(map_mouse_button(rdev::Button::Right), "右クリック");
        assert_eq!(map_mouse_button(rdev::Button::Middle), "ホイールクリック");
    }

    #[test]
    fn test_distance_calculation() {
        // (0, 0) から (3, 4) の距離は 5 px
        let dist_px = calculate_euclidean_distance(0.0, 0.0, 3.0, 4.0);
        assert!((dist_px - 5.0).abs() < 1e-6);

        // 96 px = 1 inch = 0.0254 m
        let dist_m = pixel_distance_to_meters(96.0);
        assert!((dist_m - 0.0254).abs() < 1e-6);
    }
}
