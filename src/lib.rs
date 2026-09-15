//! # TypeCT (タイプ数カウンター)
//!
//! Windows 11 (64bitネイティブ) 向けに刷新された、Rust + egui 製の打鍵・マウスクリック数計測デスクトップアプリケーション。
//!
//! かつて Windows で愛用されていた Borland C++ 5.5 製フリーソフト「タイプ数カウンター (typect.exe)」の
//! シンプルさ・軽快さを継承しつつ、現代的なテクノロジースタックで再構築されています。
//!
//! ## 主な特徴
//! - **二重起動防止**: Windows Named Mutex (`single-instance`) による多重起動防止
//! - **日本語フォント確実表示**: Windows標準フォントを自動検出し、文字化け（豆腐化: □）を完全防止
//! - **低遅延グローバル入力監視**: `rdev` による独立スレッドでの監視と、キーロガーを行わない安全な集計
//! - **旧データ自動移行**: 同一フォルダ内の旧 `data`（打鍵）および `data2`（クリック）固定長バイナリからの自動引き継ぎ
//! - **モダンUI**: `eframe` / `egui` による統計ダッシュボードと `egui_plot` による時間帯別打鍵グラフ
//! - **タスクトレイ常駐**: 動的ツールチップ（本日の打鍵/クリック数表示）および最小化格納
//! - **省電力設計**: アイドル時・バックグラウンド時は待機時 CPU 0% を維持
//! - **バイナリ極小化**: LTO、サイズ優先最適化 (`opt-level = "z"`)、デバッグシンボル削除
//!
//! ## モジュール構成
//! - [`legacy`][] - 旧バージョン (typect) の固定長バイナリヘッダ解析
//! - [`model`][] - 打鍵・クリック数集計データ構造体、日付ロールオーバー、永続化
//! - [`input`][] - 入力イベントメッセージ、プライバシー保護キーマッピング
//! - [`font`][] - 日本語フォント探索・ロード
//! - [`tray`][] - 自己完結型タスクトレイアイコン生成
//! - [`app`][] - eframe / egui GUI アプリケーション本体

pub mod app;
pub mod font;
pub mod input;
pub mod legacy;
pub mod model;
pub mod tray;

pub use app::TypeCtApp;
pub use input::InputHookMessage;
pub use legacy::LegacyHeader;
pub use model::TypeCtData;
