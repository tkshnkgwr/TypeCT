//! タスクトレイアイコンモジュール
//!
//! 外部画像ファイル（.ico や .png）への依存を排除した、
//! 32x32 RGBA ピクセル配列の動的生成による自己完結型トレイアイコンの構築機能を提供します。

/// 外部画像ファイル不要の自己完結型タスクトレイアイコン（32x32 RGBA）用のバイト列を生成します。
///
/// 中央にシアン色（RGB: 34, 211, 238）の「T」シンボルを描画し、ダークトーンの背景とボーダーを付与します。
///
/// # 戻り値
/// * `Vec<u8>` - 32x32 x 4バイト = 4,096 バイトの RGBA ピクセルデータ
pub fn generate_tray_icon_rgba(width: u32, height: u32) -> Vec<u8> {
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            let is_border = x == 0 || x == width - 1 || y == 0 || y == height - 1;
            let is_t_bar = (6..=9).contains(&y) && (6..=25).contains(&x);
            let is_t_stem = (10..=25).contains(&y) && (14..=17).contains(&x);
            if is_t_bar || is_t_stem {
                // シアンの「T」ロゴ
                rgba.extend_from_slice(&[34, 211, 238, 255]);
            } else if is_border {
                // ボーダー（スレートグレー）
                rgba.extend_from_slice(&[51, 65, 85, 255]);
            } else {
                // ダーク背景
                rgba.extend_from_slice(&[15, 23, 42, 240]);
            }
        }
    }
    rgba
}

/// 外部画像不要の自己完結型タスクトレイアイコン（32x32 RGBA）を動的生成します。
///
/// # 戻り値
/// * `Option<tray_icon::Icon>` - トレイアイコンオブジェクト（生成失敗時は `None`）
pub fn create_embedded_tray_icon() -> Option<tray_icon::Icon> {
    let width = 32;
    let height = 32;
    let rgba = generate_tray_icon_rgba(width, height);
    tray_icon::Icon::from_rgba(rgba, width, height).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tray_icon_rgba() {
        let width = 32;
        let height = 32;
        let rgba = generate_tray_icon_rgba(width, height);

        // 32 x 32 x 4 = 4096 バイト
        assert_eq!(rgba.len(), (width * height * 4) as usize);

        // 左上隅 (0, 0) はボーダー色 [51, 65, 85, 255]
        assert_eq!(&rgba[0..4], &[51, 65, 85, 255]);

        // 中央 Tバーのピクセル (x=15, y=7)
        // index = (7 * 32 + 15) * 4 = (224 + 15) * 4 = 239 * 4 = 956
        let t_pixel_idx = ((7 * 32) + 15) * 4;
        assert_eq!(&rgba[t_pixel_idx..t_pixel_idx + 4], &[34, 211, 238, 255]);
    }
}
