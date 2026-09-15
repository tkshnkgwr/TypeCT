//! デスクトップ通知モジュール
//!
//! 1日の目標打鍵数達成時などに、Windows ネイティブのトースト通知を発行します。

use notify_rust::Notification;

/// 目標打鍵数達成時のトースト通知を発行します。
///
/// # 引数
/// * `keys` - 達成した本日の打鍵数
///
/// # 戻り値
/// * `bool` - 通知の送信に成功した場合 `true`
pub fn send_goal_reached_notification(keys: u64) -> bool {
    Notification::new()
        .summary("🎉 目標打鍵数達成！")
        .body(&format!(
            "本日の打鍵数が {} 打に到達しました！お疲れ様です！",
            keys
        ))
        .appname("TypeCT")
        .show()
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        // Notificationオブジェクトが問題なく生成できるかテスト
        let notif = Notification::new()
            .summary("Test")
            .body("Test message")
            .appname("TypeCT")
            .finalize();
        assert_eq!(notif.summary, "Test");
    }
}
