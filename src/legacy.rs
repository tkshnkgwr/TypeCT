//! 旧バージョン (typect) のバイナリデータ解析モジュール
//!
//! Borland C++ 5.5 製の旧「タイプ数カウンター (typect.exe)」が生成する
//! 固定長バイナリファイル (`data`: 打鍵数, `data2`: マウスクリック数) の
//! 先頭ヘッダ（28バイト・リトルエンディアン u32）を解析します。

/// 旧バージョン(typect)の固定長バイナリヘッダ（打鍵: `data` / クリック: `data2`）
///
/// 先頭28バイト（4バイト・リトルエンディアン整数の連続ダンプ）を解析し、
/// 過去の累計日数、最終記録年月日、トータルカウント、本日・昨日のカウントを抽出します。
///
/// # メモリレイアウト
/// - `0x00..0x04`: 記録日数 (`days_recorded`)
/// - `0x04..0x08`: 最終記録年 (`last_year`)
/// - `0x08..0x0C`: 最終記録月 (`last_month`)
/// - `0x0C..0x10`: 最終記録日 (`last_day`)
/// - `0x10..0x14`: 累計トータルカウント (`total_count`)
/// - `0x14..0x18`: 本日のカウント (`today_count`)
/// - `0x18..0x1C`: 昨日のカウント (`yesterday_count`)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LegacyHeader {
    /// 記録日数（通算日数）
    pub days_recorded: u32,
    /// 最終記録年（西暦）
    pub last_year: u32,
    /// 最終記録月 (1..=12)
    pub last_month: u32,
    /// 最終記録日 (1..=31)
    pub last_day: u32,
    /// 累計トータルカウント
    pub total_count: u32,
    /// 本日のカウント
    pub today_count: u32,
    /// 昨日のカウント
    pub yesterday_count: u32,
}

impl LegacyHeader {
    /// バイトスライスから旧フォーマットヘッダを読み取ります。
    ///
    /// バッファ長が 28 バイト未満の場合は `None` を返します。
    ///
    /// # 引数
    /// * `bytes` - 読み取り対象のバイナリスライス
    ///
    /// # 戻り値
    /// * `Some(LegacyHeader)` - 解析成功時
    /// * `None` - バッファサイズ不足時
    ///
    /// # 例
    /// ```rust
    /// use typect_lib::legacy::LegacyHeader;
    ///
    /// let mut buffer = vec![0u8; 28];
    /// // 累計カウントを 1000 (0x03E8) に設定
    /// buffer[0x10..0x14].copy_from_slice(&1000u32.to_le_bytes());
    ///
    /// let header = LegacyHeader::parse(&buffer).expect("パース成功");
    /// assert_eq!(header.total_count, 1000);
    /// ```
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 28 {
            return None;
        }
        let read_u32 = |offset: usize| -> u32 {
            u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ])
        };

        Some(LegacyHeader {
            days_recorded: read_u32(0x00),
            last_year: read_u32(0x04),
            last_month: read_u32(0x08),
            last_day: read_u32(0x0C),
            total_count: read_u32(0x10),
            today_count: read_u32(0x14),
            yesterday_count: read_u32(0x18),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_legacy_header() {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&120u32.to_le_bytes()); // 0x00: days_recorded
        buffer.extend_from_slice(&2023u32.to_le_bytes()); // 0x04: last_year
        buffer.extend_from_slice(&11u32.to_le_bytes()); // 0x08: last_month
        buffer.extend_from_slice(&25u32.to_le_bytes()); // 0x0C: last_day
        buffer.extend_from_slice(&543210u32.to_le_bytes()); // 0x10: total_count
        buffer.extend_from_slice(&4500u32.to_le_bytes()); // 0x14: today_count
        buffer.extend_from_slice(&3800u32.to_le_bytes()); // 0x18: yesterday_count
                                                          // 余剰バイト（旧データファイルの後半データ等）があっても正常パース可能
        buffer.extend_from_slice(&[0xFF; 64]);

        let header = LegacyHeader::parse(&buffer).expect("Should successfully parse");
        assert_eq!(header.days_recorded, 120);
        assert_eq!(header.last_year, 2023);
        assert_eq!(header.last_month, 11);
        assert_eq!(header.last_day, 25);
        assert_eq!(header.total_count, 543210);
        assert_eq!(header.today_count, 4500);
        assert_eq!(header.yesterday_count, 3800);
    }

    #[test]
    fn test_parse_exact_28_bytes() {
        let mut buffer = [0u8; 28];
        buffer[0x10..0x14].copy_from_slice(&999u32.to_le_bytes());

        let header = LegacyHeader::parse(&buffer).expect("Exact 28 bytes should parse");
        assert_eq!(header.total_count, 999);
        assert_eq!(header.days_recorded, 0);
    }

    #[test]
    fn test_parse_insufficient_length() {
        let buffer = [0u8; 27];
        assert!(LegacyHeader::parse(&buffer).is_none());

        let empty: [u8; 0] = [];
        assert!(LegacyHeader::parse(&empty).is_none());
    }
}
