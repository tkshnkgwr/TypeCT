# TypeCT 内部設計書 (ARCHITECTURE)

本書は、`TypeCT` の内部アーキテクチャ、マルチスレッド設計、モジュール構成、状態管理、イベントループ、および省電力設計について解説します。

---

## 1. 全体アーキテクチャ図

`TypeCT` は、低遅延入力フックスレッドと、GUIレンダリングを担うメインスレッドの2スレッド構成を採用しています。

```mermaid
graph TD
    OS[Windows OS] --> SingleInst{二重起動チェック}
    SingleInst -- 既に起動中 --> SilentExit[即時終了]
    SingleInst -- 初回プロセス --> MainProc[メイン処理開始]
    
    subgraph InputThread [入力フックスレッド (rdev)]
        RDevListen[rdev::listen グローバルフック] --> Filter[map_rdev_key / map_mouse_button]
        Filter --> Sender[mpsc::Sender]
    end

    subgraph MainThread [GUI & 集計メインスレッド (eframe)]
        Receiver[mpsc::Receiver] --> AppUpdate[TypeCtApp::update ループ]
        AppUpdate --> Aggregation[TypeCtData: キー/クリック集計]
        AppUpdate --> RolloverCheck{日付またぎチェック}
        AppUpdate --> TrayUpdate[tray_icon::set_tooltip 動的更新]
        AppUpdate --> AutoSave{5分経過 または 終了時?}
        AutoSave -- Yes --> WriteDisk[typect.json 保存]
        AppUpdate --> EguiRender[egui: ダッシュボード & グラフ描画]
    end

    Sender -->|非同期チャンネル通信| Receiver
    OS -->|キー / マウス操作| RDevListen
```

---

## 2. モジュール構成

```
src/
├── lib.rs       # ライブラリのルートモジュール & 再エクスポート
├── main.rs      # エントリーポイント（二重起動防止、入力スレッド起動、ウィンドウ起動）
├── legacy.rs    # 旧バイナリ（data / data2）ヘッダ解析モジュール (LegacyHeader)
├── model.rs     # 集計データモデル (TypeCtData)、ロールオーバー、JSON保存、CSV出力
├── input.rs     # 入力フックメッセージ (InputHookMessage)、安全マッピング、距離換算
├── font.rs      # 日本語フォント探索・豆腐化防止モジュール (setup_japanese_fonts)
├── tray.rs      # 自己完結型 32x32 RGBA トレイアイコン動的生成 (create_embedded_tray_icon)
├── heatmap.rs   # キーボード配列定義 & ヒートマップグラデーション描画
├── notify.rs    # 目標打鍵数達成 Windows トースト通知
├── startup.rs   # Windows スタートアップ登録管理 (winreg)
└── app.rs       # GUI アプリケーション本体 (TypeCtApp: 3タブ構成)
```

### 2.1 モジュールの責務
- **`legacy.rs`**: 旧バイナリ 28 バイトヘッダのパース。破損データに対する防御的パース。
- **`model.rs`**: 状態データの一元管理。日付またぎ時のスライド更新、日毎サマリー台帳永続化、インポート処理、JSON 保存、CSV 出力。
- **`input.rs`**: キーコードから表示文字列への純粋変換、マウス移動座標差分からのユークリッド距離積算。パスワードや文字列の保持を構造的に不可能にする防壁。
- **`font.rs`**: OS のフォントディレクトリから日本語フォントを優先探索し `egui` に登録。
- **`tray.rs`**: 外部 `.ico` ファイル不要でアイコンをピクセル単位で構築。
- **`heatmap.rs`**: QWERTY キー配列定義、最高打鍵比率に基づく4段階カラーグラデーション描画。
- **`notify.rs`**: 目標打鍵数到達時の Windows ネイティブトースト通知制御。
- **`startup.rs`**: Windows レジストリ (`HKCU\...\Run`) への自動起動エントリ登録・解除。
- **`app.rs`**: イベント受信、タスクトレイイベント処理、定期保存、UI描画、省電力待機制御。

---

## 3. スレッドモデル & 通信

1. **入力監視スレッド**:
   - `std::thread::spawn` により生成。
   - `rdev::listen` を常駐実行。
   - キー押下 (`rdev::EventType::KeyPress`)、マウスボタン押下 (`rdev::EventType::ButtonPress`)、およびマウスカーソル移動 (`rdev::EventType::MouseMove`) を検出。
   - `mpsc::Sender<InputHookMessage>` を通じてメインスレッドへ送信。
2. **GUIメインスレッド**:
   - `eframe::run_native` による UI イベントループ。
   - 毎フレーム `mpsc::Receiver::try_recv` によりノンブロッキングで全蓄積イベントを取り出して集計に加算。

---

## 4. 省電力 & 待機時 CPU 0% 設計

常駐型ユーティリティにおいて CPU 負荷は最重要課題です。`TypeCT` では以下の設計によりアイドル時・バックグラウンド時の負荷を最小化しています。

1. **表示時の動的描画間隔**:
   - 入力イベントを受信したフレーム: `ctx.request_repaint()` で即時反映。
   - 無操作時: `ctx.request_repaint_after(Duration::from_millis(500))` により 0.5 秒ごとの緩やかな再描画。
2. **最小化・トレイ格納時の完全休止**:
   - `is_window_visible == false` の間は `request_repaint` を呼び出さず、UI の再描画ループを完全に停止。
   - 入力スレッドからのイベント受信と内部集計のみがバックグラウンドで動作し、**CPU使用率 0.0%** を維持します。
