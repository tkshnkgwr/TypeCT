<div align="center">

# ⌨ TypeCT (タイプ数カウンター)

**Windows 11 (64bitネイティブ) 向けに刷新された、Rust + egui 製の超軽量・高精度な打鍵 & マウスクリック数計測デスクトップアプリ**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 2021](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%2011%20(64bit)-lightgrey.svg)](https://github.com/tkshnkgwr/TypeCT)
[![CI](https://img.shields.io/badge/CI-Passing-brightgreen.svg)](.github/workflows/ci.yml)

[English](./README.md) | [日本語](./README_JA.md)

</div>

---

## 📌 概要

`TypeCT` は、かつて Windows で広く愛用されていた Borland C++ 5.5 製のフリーソフト「タイプ数カウンター (typect.exe)」の良さ（シンプル・軽量・常駐性）を受け継ぎ、現代の Windows 11 環境に最適化してフルスクラッチで再構築した Rust 製デスクトップアプリケーションです。

バックグラウンドで打鍵数とマウスクリック数を正確かつ超低遅延でカウントし、直近24時間の打鍵時間帯分布やキー別打鍵ランキングをモダンな UI で可視化します。
また、旧「タイプ数カウンター」の記録ファイル (`data` / `data2`) が同一フォルダーに存在する場合、初回起動時に過去の累計記録を自動インポートしてそのまま引き継ぐことができます。

---

## ✨ 主な特徴

- 🔒 **多重起動防止 (Single Instance)**: Windows の Named Mutex により二重起動を確実に防止。2つ目以降のプロセスは静かに即座に終了します。
- 🔤 **日本語フォント確実表示 (豆腐化防止)**: Windows OS 標準の日本語フォント（游ゴシック、メイリオ、MSゴシック等）を自動検出し、文字化け（□）を完全に防止します。
- ⚡ **超低遅延グローバル入力監視**: `rdev` を用いた独立バックグラウンドスレッドでイベントを捕捉し、キー入力遅延を起こさずにノンブロッキングで集計します。
- 🛡️ **万全のプライバシー & セキュリティ保護**: 本アプリはキーロガーではありません。入力された文字列やパスワード等の内容は一切保持・保存せず、キーコード別のカウンター加算のみを行います。
- 📂 **旧データ自動インポート**: 昔の `typect.exe` が出力した固定長バイナリ `data`（打鍵）および `data2`（クリック）を自動解析し、累計記録をスムーズに引き継ぎます。
- 📊 **モダンなダッシュボード (egui + egui_plot)**:
  - サマリーカード表示（本日、昨日、累計、記録日数、1日平均打鍵数）
  - 直近24時間の時間帯別打鍵数棒グラフ
  - 主要キー別の打鍵数ランキング Top 12 (進捗バー表示)
- 📥 **タスクトレイ常駐 & 動的ツールチップ**: 最小化時やウィンドウ「✕」押下時にタスクトレイに格納。トレイアイコンにマウスを合わせると、リアルタイムの打鍵・クリック状況がツールチップに動的表示されます。
- 🍃 **ゼロ負荷・省電力設計**: アイドル時やバックグラウンド格納時は不要な再描画を行わず、待機時 CPU 使用率 0% を維持します。
- 📦 **単一バイナリ & 極小サイズ**: リリースビルド時の LTO（リンク時最適化）、サイズ優先最適化 (`opt-level = "z"`)、デバッグシンボル削除により極小のシングル実行ファイルを生成します。

---

## 🚀 インストール & ビルド手順

### 必要要件
- 64bit Windows 10 / 11
- Rust 1.80 以上 (Cargo)

### ローカルビルド
```powershell
# リポジトリのクローン
git clone https://github.com/tkshnkgwr/TypeCT.git
cd TypeCT

# デバッグ実行
cargo run

# リリースバイナリのビルド
cargo build --release
```
生成された実行ファイルは `target/release/typect.exe` に配置されます。

---

## 💾 旧「タイプ数カウンター」からのデータ引き継ぎ

長年使い込んできた旧 `typect.exe` の記録がある場合でも安心です。

1. 新しい `typect.exe` を任意のフォルダー（例: `%USERPROFILE%\Tools\TypeCT\`）に配置します。
2. 旧 `typect.exe` が使用していたフォルダー内にある以下のファイルを、`typect.exe` と同じフォルダーにコピーします。
   - `data` : 旧打鍵数データ（固定長バイナリ）
   - `data2` : 旧クリック数データ（固定長バイナリ）
3. `typect.exe` を起動します。
4. 初回起動時に自動的にファイルが検出・解析され、累計データがモダンな `typect.json` に取り込まれます。画面下部に「旧 typect (data / data2) からの累計データを引き継ぎ済み」と表示されます。

---

## 📁 データフォーマット (`typect.json`)

データは扱いやすい JSON 形式で同一フォルダーの `typect.json` に自動保存（アプリ終了時および5分毎）されます。

```json
{
  "total_keys": 543210,
  "today_keys": 4500,
  "yesterday_keys": 3800,
  "total_clicks": 125000,
  "today_clicks": 1200,
  "yesterday_clicks": 950,
  "days_recorded": 120,
  "last_date": "2026-09-15",
  "hourly_keys": [0, 0, 0, 0, 0, 0, 15, 120, 850, 1200, ...],
  "key_counts": {
    "A": 340,
    "Enter": 520,
    "Space": 890
  },
  "mouse_counts": {
    "左クリック": 980,
    "右クリック": 220
  },
  "legacy_imported": true
}
```

---

## 🗂️ ソースコード構成

```
src/
├── lib.rs       # ライブラリのエントリーポイント & モジュール公開
├── main.rs      # バイナリエントリーポイント（二重起動防止、入力スレッド、eframe起動）
├── legacy.rs    # 旧固定長バイナリヘッダ (LegacyHeader) 解析
├── model.rs     # 集計データ構造体 (TypeCtData)、ロールオーバー、JSON保存
├── input.rs     # 入力イベント (InputHookMessage)、安全なキー/マウス変換
├── font.rs      # Windows 日本語フォント自動検出・登録 (setup_japanese_fonts)
├── tray.rs      # 自己完結型 32x32 RGBA タスクトレイアイコン動的生成
└── app.rs       # GUI アプリケーション本体 (TypeCtApp)
tests/
└── integration_test.rs # 旧データインポート〜永続化〜ロールオーバーの結合テスト
```

---

## 📚 ドキュメント一覧

| ドキュメント                                      | 言語   | 内容                                                 |
| :------------------------------------------------ | :----- | :--------------------------------------------------- |
| [SPECIFICATION.md](docs/ja/SPECIFICATION.md)     | 日本語 | 機能詳細、バイナリヘッダ仕様、JSONスキーマ、UIレイアウト |
| [ARCHITECTURE.md](docs/ja/ARCHITECTURE.md)       | 日本語 | 内部設計書、スレッド間通信、レンダリングフロー、省電力 |
| [USER_GUIDE.md](docs/ja/USER_GUIDE.md)           | 日本語 | 詳しい操作方法、タスクトレイ操作、旧データ移行手順   |
| [DEVELOPMENT.md](docs/ja/DEVELOPMENT.md)         | 日本語 | 開発環境構築、ビルド、テスト、RustDoc生成手順        |
| [TESTING.md](docs/ja/TESTING.md)                 | 日本語 | テスト方針、単体テスト・結合テスト仕様書             |
| [TEST_REPORT.md](docs/ja/TEST_REPORT.md)         | 日本語 | 最新テスト実行結果レポート                           |
| [SECURITY.md](docs/ja/SECURITY.md)               | 日本語 | セキュリティ & プライバシー保護方針（非キーロガー証明）|
| [CHANGELOG.md](docs/ja/CHANGELOG.md)             | 日本語 | バージョンごとの更新履歴                             |
| [CONTRIBUTING.md](docs/ja/CONTRIBUTING.md)       | 日本語 | 開発参加・コントリビューションガイドライン           |
| [TODO.md](docs/ja/TODO.md)                       | 日本語 | 今後の機能拡張ロードマップ                           |
| [FOOTPRINTS.md](docs/ja/FOOTPRINTS.md)           | 日本語 | 開発経緯・刷新の背景                                 |
| [English Docs](docs/en/)                          | 英語   | 上記ドキュメントの英語版一式                         |

---

## 📄 ライセンス

本ソフトウェアは [MIT License](LICENSE) のもとで公開されています。
