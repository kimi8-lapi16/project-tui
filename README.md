# project-tui

GitHub ProjectsをTUI（Terminal User Interface）で操作できるアプリケーション

## 機能

- プロジェクト選択（インタラクティブ）
- チケット一覧表示
- チケット詳細表示・編集（タイトル、本文、ステータス）

## セットアップ

### 必要なもの

- Docker
- VSCode（推奨）
- VSCode Remote - Containers 拡張機能

### 開発環境の構築

1. このリポジトリをclone:

```bash
git clone <repository-url>
cd project-tui
```

2. VSCodeでフォルダを開く:

```bash
code .
```

3. VSCodeのコマンドパレット（Cmd+Shift+P / Ctrl+Shift+P）から「Dev Containers: Reopen in Container」を選択

4. コンテナが起動すると、自動的に`cargo build`が実行されます

### 設定ファイルの作成

初回実行時に、GitHub Personal Access Tokenを設定する必要があります。

1. 設定ファイルの場所を確認:

```bash
# コンテナ内で
cargo run
```

エラーメッセージに設定ファイルのパスが表示されます（通常は`~/.config/project-tui/config.toml`）

2. 設定ファイルを作成:

```bash
mkdir -p ~/.config/project-tui
cat > ~/.config/project-tui/config.toml << 'EOF'
[github]
token = "ghp_your_token_here"
api_url = "https://api.github.com/graphql"

[ui]
theme = "default"
EOF
```

3. GitHub Personal Access Tokenを取得:
   - https://github.com/settings/tokens にアクセス
   - "Generate new token (classic)" をクリック
   - 以下の権限を付与:
     - `repo` (全て)
     - `project` (read:project)
   - 生成されたトークンを設定ファイルの`token`に設定

## 実行

```bash
cargo run
```

## 開発

### ビルド

```bash
cargo build
```

### フォーマット

```bash
cargo fmt
```

保存時に自動フォーマットされます（VSCode設定済み）

### Linting

```bash
cargo clippy
```

### テスト

```bash
cargo test
```

## キーバインディング

### プロジェクト選択画面

- `j` / `↓`: 下に移動
- `k` / `↑`: 上に移動
- `Enter`: プロジェクトを選択
- `q` / `Ctrl+C`: 終了

### チケット一覧画面

- `j` / `↓`: 下に移動
- `k` / `↑`: 上に移動
- `Enter`: チケット詳細を表示
- `Esc`: プロジェクト選択に戻る
- `q` / `Ctrl+C`: 終了

### チケット詳細画面

- `e`: 編集モードに入る
- `Esc`: チケット一覧に戻る
- `q`: 終了

### チケット編集モード

- `Tab`: 次のフィールドに移動
- `Shift+Tab`: 前のフィールドに移動
- `Ctrl+S`: 保存
- `Esc`: キャンセル

## アーキテクチャ

- **言語**: Rust 1.82
- **TUIフレームワーク**: ratatui + crossterm
- **非同期ランタイム**: tokio
- **GitHub API**: GraphQL (Projects v2)

## その他
- Claude Codeでガッと書かせました
- 今後機能をいろいろ追加していく所存
