//! Windows スタートアップ登録管理モジュール
//!
//! Windows のレジストリ (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`) を利用して、
//! PC 起動時の TypeCT 自動起動登録および解除を行います。

#[cfg(windows)]
use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE};
#[cfg(windows)]
use winreg::RegKey;

const RUN_KEY_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const APP_NAME: &str = "TypeCT";

/// 現在の実行ファイルが Windows スタートアップに登録されているか確認します。
///
/// # 戻り値
/// * `true` - スタートアップに登録されている場合
/// * `false` - 登録されていない、または確認失敗時
pub fn is_startup_enabled() -> bool {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(run_key) = hkcu.open_subkey_with_flags(RUN_KEY_PATH, KEY_READ) {
            let res: Result<String, _> = run_key.get_value(APP_NAME);
            return res.is_ok();
        }
    }
    false
}

/// Windows スタートアップの登録または解除を設定します。
///
/// # 引数
/// * `enable` - `true` で登録、`false` で解除
///
/// # 戻り値
/// * `std::io::Result<()>` - 成功時は `Ok(())`
pub fn set_startup_enabled(enable: bool) -> std::io::Result<()> {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (run_key, _) = hkcu.create_subkey_with_flags(RUN_KEY_PATH, KEY_WRITE)?;

        if enable {
            let current_exe = std::env::current_exe()?;
            let exe_str = current_exe.to_string_lossy().to_string();
            // スペースを含むパスに対応するためダブルクォーテーションで囲む
            let command = format!("\"{}\"", exe_str);
            run_key.set_value(APP_NAME, &command)?;
        } else {
            let _ = run_key.delete_value(APP_NAME);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup_key_path_constants() {
        assert_eq!(APP_NAME, "TypeCT");
        assert!(RUN_KEY_PATH.contains("CurrentVersion\\Run"));
    }
}
