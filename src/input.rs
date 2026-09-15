//! 入力イベント & キーマッピングモジュール
//!
//! `rdev` クレートから受け取った低レベルのキーおよびマウスイベントを、
//! プライバシーに配慮した安全な表示用シンボル文字列へマッピングします。
//!
//! # プライバシー & セキュリティ保護
//! 本アプリはキーロガーではありません。入力された文字列や入力内容は一切記録・保持せず、
//! 押下されたキー種別（例: `A`, `Enter`, `Shift` 等）のカウンター加算のみを行います。

/// 非同期入力フックスレッドから GUI スレッドへ送信されるイベント
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputHookMessage {
    /// キーボード打鍵イベント（キー名称）
    Key(String),
    /// マウスクリックイベント（ボタン名称）
    Mouse(String),
}

/// `rdev::Key` を安全で分かりやすい表示用文字列へマッピングします。
///
/// キーストロークの内容（連続した単語やパスワード等）は一切保持せず、
/// キー名称のみを返します。
///
/// # 引数
/// * `key` - `rdev::Key` 列挙型値
///
/// # 戻り値
/// * `&'static str` - キーの表示文字列（例: `"A"`, `"Enter"`, `"Space"` 等）
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
///
/// # 引数
/// * `btn` - `rdev::Button` 列挙型値
///
/// # 戻り値
/// * `&'static str` - ボタンの表示文字列（例: `"左クリック"`, `"右クリック"` 等）
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
        assert_eq!(map_rdev_key(rdev::Key::KeyZ), "Z");
        assert_eq!(map_rdev_key(rdev::Key::Num0), "0");
        assert_eq!(map_rdev_key(rdev::Key::Num9), "9");
        assert_eq!(map_rdev_key(rdev::Key::Space), "Space");
        assert_eq!(map_rdev_key(rdev::Key::Return), "Enter");
        assert_eq!(map_rdev_key(rdev::Key::Backspace), "Backspace");
        assert_eq!(map_rdev_key(rdev::Key::Tab), "Tab");
        assert_eq!(map_rdev_key(rdev::Key::ShiftLeft), "Shift");
        assert_eq!(map_rdev_key(rdev::Key::ShiftRight), "Shift");
        assert_eq!(map_rdev_key(rdev::Key::ControlLeft), "Ctrl");
        assert_eq!(map_rdev_key(rdev::Key::Escape), "Esc");
        assert_eq!(map_rdev_key(rdev::Key::UpArrow), "↑");
        assert_eq!(map_rdev_key(rdev::Key::DownArrow), "↓");
    }

    #[test]
    fn test_map_mouse_buttons() {
        assert_eq!(map_mouse_button(rdev::Button::Left), "左クリック");
        assert_eq!(map_mouse_button(rdev::Button::Right), "右クリック");
        assert_eq!(map_mouse_button(rdev::Button::Middle), "ホイールクリック");
        assert_eq!(map_mouse_button(rdev::Button::Unknown(4)), "拡張クリック");
    }
}
