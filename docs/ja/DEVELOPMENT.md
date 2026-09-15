# TypeCT 開発ガイド (DEVELOPMENT)

本書は、`TypeCT` の開発者向け環境構築手順、ビルド手順、テスト実行、静的解析、およびドキュメント生成手順についてのガイドです。

---

## 1. 開発環境の準備

### 1.1 前提ソフトウェア
- **OS**: Windows 11 (64bit) / Windows 10 (64bit)
- **Rust**: 1.80 以上 (MSVC ツールチェーン推奨)
  ```powershell
  rustup default stable-x86_64-pc-windows-msvc
  rustup component add rustfmt clippy
  ```

---

## 2. ビルドコマンド一覧

### 2.1 デバッグビルド & 実行
```powershell
# デバッグビルドして即時実行
cargo run

# コンパイルチェックのみ
cargo check
```

### 2.2 リリースビルド
```powershell
cargo build --release
```
生成されるバイナリ: `target/release/typect.exe`

---

## 3. テストの実行

### 3.1 単体テスト & 結合テスト & ドキュメントテスト
```powershell
# すべてのテストを実行
cargo test

# 詳細ログを表示して実行
cargo test -- --nocapture
```

---

## 4. コード品質管理 (Lint & Format)

コミット前には必ず以下の検査を実施してください。

```powershell
# コードフォーマットチェック
cargo fmt --check

# 自動フォーマット適用
cargo fmt

# Clippy による静的解析（警告をエラーとして検出）
cargo clippy --all-targets -- -D warnings
```

---

## 5. RustDoc ドキュメント生成

コードベース内のすべてのモジュール、構造体、関数には詳細な RustDoc コメントが記述されています。

```powershell
# ドキュメントをビルドしてブラウザで開く
cargo doc --no-deps --open
```
生成された HTML は `target/doc/typect_lib/index.html` に配置されます。
