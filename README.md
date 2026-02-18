# Claude Statusline

Claude Codeのステータスライン表示ツール（Rust実装）

## 概要

Claude Codeセッションのリアルタイム統計情報をターミナルのステータスラインに表示するツールです。トークン使用量、コスト、セッション時間、Gitブランチなどの情報を視覚的にわかりやすく表示します。

## 主な機能

### 表示モード

ターミナル幅に応じて自動的に最適な表示モードを選択します：

- **Fullモード** (68文字以上): 4行の詳細表示
- **Compactモード** (35-67文字): ラベルを短縮した表示
- **Tightモード** (20-34文字): モデル名とメッセージ数のみ
- **Minimalモード** (20文字未満): モデル名のみ

### 表示情報

#### Line 1: 基本情報
- モデル名（短縮表示）
- Gitブランチと変更ファイル数
- プロジェクト名
- メッセージ数
- 累計コスト

#### Line 2: コンテキストウィンドウ
- 使用率を視覚的に表示（プログレスバー）
- パーセンテージ表示
- トークン数（入力/出力）

#### Line 3: セッション情報
- セッション経過時間のプログレスバー
- 経過時間/制限時間
- 現在時刻と時間範囲

#### Line 4: トークンバーン
- トークン消費量のタイムライン（スパークライン）
- 総トークン数（キャッシュ込み）
- トークン消費レート（tokens/minute）

### 特殊モード

- **エージェントモード**: エージェント名をシンプルに表示
- **スケジュールモード**: 今後の予定イベントを表示

## インストール

### ビルド

```bash
git clone https://github.com/ronkovic/claude-statusline.git
cd claude-statusline
cargo build --release
```

ビルドされたバイナリは `target/release/cc-statusline` に生成されます。

### 配置

```bash
# 任意の場所にバイナリをコピー
cp target/release/cc-statusline ~/.local/bin/
# または
cp target/release/cc-statusline /usr/local/bin/
```

## 使い方

### 基本的な使い方

標準入力からJSON形式のセッション情報を受け取り、ステータスラインを表示します：

```bash
echo '{"model": {"display_name": "Sonnet 4.5"}, ...}' | cc-statusline
```

### スケジュールモード

```bash
cc-statusline --schedule
```

### 入力フォーマット

JSON形式で以下の情報を受け取ります：

```json
{
  "model": {
    "id": "claude-sonnet-4-5-20250929",
    "display_name": "Sonnet 4.5"
  },
  "cwd": "/path/to/project",
  "session_stats": {
    "message_count": 15,
    "total_input_tokens": 45000,
    "total_output_tokens": 12000,
    "block_count": 3,
    "total_cache_creation": 5000,
    "total_cache_read": 8000,
    "duration_seconds": 5400
  },
  "context_window": {
    "total_input_tokens": 45000,
    "total_output_tokens": 12000,
    "context_window_size": 200000,
    "used_percentage": 28
  },
  "cost": {
    "total_cost_usd": 0.52
  }
}
```

## 開発

### テストの実行

```bash
cargo test
```

### リリースビルド

```bash
cargo build --release
```

最適化されたバイナリが生成されます（サイズ最小化、LTO有効）。

## 技術スタック

- **言語**: Rust (Edition 2021)
- **主要ライブラリ**:
  - `serde` / `serde_json`: JSON処理
  - `chrono`: 日時処理
  - `unicode-width`: Unicode文字幅計算
  - `dirs`: ディレクトリパス取得
  - `libc`: ターミナル幅検出

## ライセンス

MIT License

## 作者

ronkovic
