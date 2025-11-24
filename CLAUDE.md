# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリのコードを扱う際のガイダンスを提供します。

## プロジェクト概要

`saba`は、YAML仕様に基づいてコードを生成する、Rustで書かれた宣言的開発フレームワークです。現在のv2実装は、Rust、Go、Python、TypeScript、JavaScriptを含む複数のプログラミング言語をサポートし、シンプルなYAML設定に基づいてプロジェクト構造を生成します。

## アーキテクチャ (v2)

v2実装はクリーンアーキテクチャパターンに従います:

- **CLIレイヤー** (`src/cli/`): コマンドラインインターフェース
  - `command/new.rs`: デュアルモード（対話型/AI）によるプロジェクト初期化
  - `command/up.rs`: CodeGeneratorを使用したコード生成実行
  - `command/analyze.rs`: プロジェクト分析機能
  - `command/ai_init.rs`: Claude Codeカスタムコマンド生成（.claude/commands/saba_init.mdを出力）
  - `command/completion.rs`: シェル補完スクリプト生成（Bash、Zsh、Fish、PowerShell、Elvish対応）
- **コード生成レイヤー** (`src/code_generation/`): コア生成ロジック
  - `core/generator.rs`: 全言語対応の統一CodeGenerator
  - `language/`: 言語固有のジェネレーター (rust, go, python, typescript, javascript)
    - 各言語には専用のジェネレーターとハンドラーがある
    - 単一プロジェクトとマルチプロジェクト構成の両方をサポート
  - `service/`: 生成サービス
- **プロジェクト管理** (`src/project_management/`): 設定処理
  - `config/`: YAML設定のパースと検証
  - `repository/`: 設定リポジトリパターン
  - `service/`: プロジェクト管理サービス
- **Claude Code統合** (`src/claude_code_integration/`): AI支援機能
- **共通** (`src/shared/`): 共通ユーティリティとエラー処理

## 主要なコマンド

### 開発
```bash
# プロジェクトをビルド
cargo build

# リリースバージョンをビルド
cargo build --release

# ビルドして_testディレクトリにコピー（テスト用）
./build.sh

# アプリケーションを実行
cargo run -- <subcommand>
```

### アプリケーション使用方法
```bash
# 対話的な言語選択で新規プロジェクトを初期化（Humanモード）
saba new

# 指定した言語で新規プロジェクトを初期化（AIモード）
saba new --lang rust
saba new --lang typescript
saba new --lang go
saba new --lang python
saba new --lang javascript

# saba.yml仕様からコードを生成
saba up

# Claude Codeカスタムコマンドを生成（.claude/commands/saba_init.mdを出力）
saba ai-init

# シェル補完スクリプトを生成
saba completion bash
saba completion zsh
saba completion fish
saba completion powershell
saba completion elvish
```

### テスト
```bash
# 全言語ジェネレーターをテスト（先にbuild.shが必要）
./build.sh
cd _test

# Goプロジェクトをテスト
echo "- name: test-go-project
  root: true
  lang: go
  upstream:
    - name: pkg
      upstream:
        - name: models
          codefile:
            - name: user
  codefile:
    - name: main" > saba.yml
./saba up

# Pythonプロジェクトをテスト
echo "- name: test-python-project
  root: true
  lang: python
  upstream:
    - name: src
      upstream:
        - name: models
          codefile:
            - name: user
  codefile:
    - name: main" > saba.yml
./saba up

# JavaScriptプロジェクトをテスト
echo "- name: test-js-project
  root: true
  lang: javascript
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: Button
            - name: Header.jsx
  codefile:
    - name: index" > saba.yml
./saba up
```

## 設定 (v2)

- **saba.yml**: 以下を定義するプロジェクト仕様ファイル:
  - `name`: プロジェクト名
  - `root`: これがルートプロジェクトかどうかを示すブール値
  - `lang`: プログラミング言語 (rust, go, python, typescript, javascript)
  - `upstream`: 階層的なモジュール構造
  - `codefile`: モジュール内の個別のコードファイル

例:
```yaml
- name: my-project
  root: true
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: Button.tsx    # 拡張子が保持される
            - name: Modal.vue     # 拡張子が保持される
            - name: utils         # .ts拡張子が付与される
```

## コード生成フロー (v2)

1. `saba new` が `saba.yml` 仕様ファイルを作成:
   - **Humanモード**: `--lang`オプションがない場合、対話的な言語選択プロンプトを表示
   - **AIモード**: `--lang`オプションによる直接言語指定
   - **プロジェクト命名**: 連番のプロジェクト名を自動生成 (`app_1`, `app_2`, etc.)
   - **マルチプロジェクトサポート**: 既存の`saba.yml`に自動追加し、以前のプロジェクトから`root: true`を削除
2. `saba up` が統一された`CodeGenerator`を使用して仕様を処理:
   - **単一プロジェクトモード**: `root: true`で現在のディレクトリに直接生成
   - **マルチプロジェクトモード**: 各プロジェクトごとに別々のディレクトリを作成
   - **ワークスペース生成**: マルチプロジェクトRust構成用のワークスペースファイルを自動作成
3. 各言語ジェネレーターが作成:
   - モジュール階層に基づくディレクトリ構造
   - 適切な拡張子と内容を持つコードファイル
   - 言語固有のプロジェクトファイル:
     - **Rust**: `Cargo.toml` (個別プロジェクト用)、ワークスペース `Cargo.toml` + `Makefile.toml` (マルチプロジェクト用)
     - **TypeScript**: `package.json`, `tsconfig.json`
     - **Go**: `go.mod`, `go.sum`, `main.go`
     - **Python**: `setup.py`, `requirements.txt`, `main.py`
     - **JavaScript**: `package.json`, `index.js`

## 言語固有の機能

### Rust
- 適切なモジュール宣言を持つ`mod.rs`、`main.rs`、または`lib.rs`を生成
- モジュールとファイルに対する`pub`可視性制御をサポート
- `lib.rs`の存在に基づいてライブラリvsバイナリプロジェクトを自動検出
- **ワークスペースサポート**: マルチプロジェクト構成用のワークスペース`Cargo.toml`と`Makefile.toml`を生成

### TypeScript
- 各モジュールにバレルエクスポートを持つ`index.ts`ファイルを生成
- `package.json`と`tsconfig.json`設定ファイルを作成
- **ファイル拡張子の優先順位**: `codefile.name`にドット(.)が含まれる場合、既存の拡張子が保持される (.tsx, .vue, .jsx, etc.)。それ以外の場合、.ts拡張子を追加
- バレルエクスポートは適切なTypeScriptモジュール解決のために拡張子を削除

### Go
- 依存関係管理のための`go.mod`と`go.sum`を生成
- mainファイルが明示的に定義されていない場合、"Hello, world!"テンプレートを持つ`main.go`を作成
- **パッケージ命名**: ルートレベルファイルは`package main`を使用、モジュールファイルはディレクトリ名をパッケージとして使用
- 適切なパッケージサニタイゼーション（ハイフンをアンダースコアに、小文字化）

### Python
- プロジェクトメタデータを持つ`setup.py`と依存関係用の`requirements.txt`を生成
- 各モジュールディレクトリに自動的に`__init__.py`ファイルを作成
- **バレルインポート**: `__init__.py`ファイルに`from .module import *`文を含む
- mainファイルが明示的に定義されていない場合、適切なエントリーポイントテンプレートを持つ`main.py`を作成

### JavaScript
- ESモジュール設定とNode.jsスクリプトを持つ`package.json`を生成
- ESモジュールバレルエクスポート（`export * from`）を持つ`index.js`ファイルを作成
- **混合拡張子**: インポート文で元の拡張子（.jsx, .mjs）を保持
- 適切なファイル拡張子処理を伴うモダンなESモジュール構文をサポート

## 重要な実装メモ

### ファイル拡張子の処理
`CodeFile`の`filename_with_extension()`メソッドは拡張子の優先順位を実装:
- 既存の拡張子を持つ名前（例: "Button.tsx", "Modal.vue", "Header.jsx"）は保持される
- 拡張子のない名前は言語固有の拡張子が付与される（例: "utils" → "utils.ts", "utils.py", "utils.go", "utils.js"）
- これにより、TypeScript/React/Vue開発で一般的な混合拡張子プロジェクトが可能になる

### ビルドとテストのワークフロー
**重要**: テスト前には必ず`build.sh`を実行してください:
1. `build.sh`が`_test`ディレクトリの内容をクリア（`rm -rf ./_test/*`）
2. リリースビルドを実行（`cargo build --release`）
3. 実行ファイルを`_test`ディレクトリにコピー
4. これにより、テストが最新の実装を使用することが保証される

### マルチプロジェクト vs 単一プロジェクト
- **単一プロジェクト** (`root: true`): 現在のディレクトリに直接生成
- **マルチプロジェクト**: 各プロジェクトごとに別々のサブディレクトリを作成
- **ワークスペース検出**: 複数のRustプロジェクトが存在する場合、Rustワークスペースファイルを自動生成

### saba new コマンドの機能
- **デュアルモード動作**:
  - `saba new` (引数なし): 人間のユーザー向けの対話的な言語選択
  - `saba new --lang <language>`: AI自動化のための直接言語指定
- **連番プロジェクト名**: `app_1`, `app_2`, `app_3`, などを自動生成
- **スマートマルチプロジェクト処理**:
  - 最初のプロジェクト: `root: true`を持つ新しい`saba.yml`を作成
  - 以降のプロジェクト: 既存の`saba.yml`に追加し、すべてのプロジェクトから`root: true`を削除
- **言語別ディレクトリ構造**:
  - **Rustプロジェクト**: `src/`ディレクトリ構造で生成
  - **その他すべての言語**: ルートレベルのファイル構造で生成

## ファイル保護システム

**重要**: sabaフレームワークは、コードの損失を防ぐために3段階のファイル保護システムを実装しています:

### 1. コードファイル（完全保護）
- **ファイル**: `.rs`, `.go`, `.py`, `.js`, `.ts`, `.jsx`, `.tsx`, `.vue`, など
- **動作**: 一度作成されると、これらのファイルは**決して上書きされない**
- **実装**: 書き込み前に`if !file_path.exists()`チェックを使用
- **目的**: 開発者の実装コードを保護

### 2. 管理ファイル（部分更新）
- **ファイル**: `mod.rs`, `lib.rs`, `main.rs`, `__init__.py`, `index.js`, `index.ts`
- **動作**: ヘッダー/フッターマーカー間のsaba管理セクションのみを更新
- **実装**: 正規表現パターンマッチングを伴う`ContentUpdater`を使用
- **ヘッダー/フッター**:
  - Rust: `// start auto exported by saba.` ... `// end auto exported by saba.`
  - Python: `# start auto exported by saba.` ... `# end auto exported by saba.`
  - JS/TS: `// start auto exported by saba.` ... `// end auto exported by saba.`
- **目的**: sabaがインポート/エクスポートを管理しながらカスタムコードを保持できるようにする

### 3. プロジェクト設定ファイル（初回作成のみ）
- **ファイル**: `package.json`, `tsconfig.json`, `Cargo.toml`, `go.mod`, `go.sum`, `requirements.txt`, `setup.py`
- **動作**: 存在しない場合のみ作成
- **実装**: `if !file_path.exists()`チェックを使用
- **目的**: ユーザーのプロジェクト設定変更の上書きを防ぐ

### モジュール管理ファイルは常に生成
- **mod.rs** (Rust): `src`を除くすべてのモジュールに生成
- **index.js/index.ts** (JS/TS): すべてのモジュールに生成
- **__init__.py** (Python): すべてのモジュールに生成
- **目的**: 空のディレクトリでも適切なモジュール構造を保証

## Gitコミットガイドライン

**重要**: すべてのコミットメッセージは日本語で記述する必要があります。

### コミットメッセージ形式
```
<動作の概要>

<詳細な説明>
- 実装した機能や修正内容
- 変更の理由や背景
- 影響範囲

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### 例
```bash
# 新機能追加
git commit -m "ファイル保護システムの実装

- 3層ファイル保護システムを追加
- ContentUpdaterユーティリティで部分更新を実装
- 既存コードの上書きを防ぐ仕組みを構築

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"

# バグ修正
git commit -m "mod.rsの空モジュール対応を修正

- 空のモジュールでもmod.rsが生成されるように修正
- v1仕様との互換性を確保
- 全言語で管理ファイルが常に生成されるよう統一

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

### コミットタイプ（日本語）
- `実装`: 新機能の実装
- `修正`: バグ修正
- `改善`: 既存機能の改良
- `リファクタリング`: コード構造の改善
- `テスト`: テストの追加・修正
- `ドキュメント`: ドキュメントの更新
- `設定`: 設定ファイルの変更

## ai-initコマンド

**最終更新**: 2025-11-23

`saba ai-init`コマンドは、Claude Codeのカスタムコマンド（`.claude/commands/saba_init.md`）を生成します。これにより、Claude Code上で`/saba_init`コマンドを実行することで、自動的にプロジェクトのCLAUDE.mdにsaba.ymlの書き方を追記できるようになります。

### 目的
- 他のプロジェクトでsabaを使用する際、Claude Codeカスタムコマンド経由で自動的にCLAUDE.mdを更新できるようにする
- 手動でのコピペを不要にし、チーム全体で統一されたsabaガイドラインを共有できるようにする

### 使い方

#### 1. カスタムコマンドの生成
```bash
# プロジェクトディレクトリで実行
saba ai-init
```

このコマンドにより、`.claude/commands/saba_init.md`ファイルが生成されます。

#### 2. Claude Codeでカスタムコマンドを実行
```bash
# Claude Codeで実行
/saba_init
```

Claude Codeが自動的に以下を実行します：
- プロジェクトルートのCLAUDE.mdにsaba.ymlの書き方セクションを追記
- sabaフレームワークの概要説明を追加
- saba.ymlの基本構造とフィールド説明を追加
- サポート言語と各言語の特徴を追加
- 言語別のプロジェクト例を追加
- マルチプロジェクト構成の説明を追加
- ベストプラクティスを追加

### 出力されるカスタムコマンドの内容

`.claude/commands/saba_init.md`には、Claude Codeに対する詳細な指示が記載されます：
- sabaフレームワークの概要説明
- saba.ymlの構造と各フィールドの詳細
- サポートされている言語（Rust、Go、Python、TypeScript、JavaScript）
- 各言語の特徴とプロジェクト例
- マルチプロジェクト構成の方法
- ファイル保護システムの仕組み
- ベストプラクティス

### メリット

- **自動化**: `/saba_init`を実行するだけで、CLAUDE.mdが自動更新される
- **一貫性**: チーム全体で同じカスタムコマンドを使用できる
- **再利用性**: 一度`saba ai-init`を実行すれば、何度でも`/saba_init`を使用可能
- **メンテナンス性**: sabaのバージョンアップ時も、`saba ai-init`を再実行するだけで最新のガイドが提供される

### 実装の変遷
- **v2.1.4以前**: `guide`コマンドとして包括的なガイドコンテンツを直接出力
- **v2.1.4**: `describe`コマンドに改名し、標準出力にClaude Codeへの指示を出力
- **v2.2.1**: `ai-init`コマンドに改名し、`.claude/commands/saba_init.md`を生成するように変更。Claude Codeのカスタムコマンド機能を活用した、より洗練されたUXを実現

### Rustの生文字列リテラルの注意点
`ai_init.rs`の実装では、`r#"..."#`の中に`"#`という文字列が含まれるため、`r##"..."##`のように`#`を増やして対応しています。これは、パーサーが途中で文字列が終わったと誤認識するのを防ぐためです。

### 関連issue
- #183: saba describeコマンドをai-initコマンドに変更し、Claude Codeカスタムコマンドを生成

## completionコマンド

**最終更新**: 2025-11-20

`saba completion`コマンドは、各種シェルの補完スクリプトを生成します。これにより、sabaコマンドのサブコマンドやオプションをタブ補完できるようになります。

### サポートシェル
- **Bash**: Linuxで広く使用される標準シェル
- **Zsh**: macOSデフォルトシェル（macOS Catalina以降）
- **Fish**: ユーザーフレンドリーな対話型シェル
- **PowerShell**: Windows標準シェル
- **Elvish**: モダンなクロスプラットフォームシェル

### 使い方

#### 補完スクリプトの生成
```bash
# Bash用
saba completion bash > /usr/local/etc/bash_completion.d/saba

# Zsh用
saba completion zsh > ~/.zsh/completion/_saba

# Fish用
saba completion fish > ~/.config/fish/completions/saba.fish

# PowerShell用
saba completion powershell > saba.ps1
# その後、PowerShellプロファイルに以下を追加:
# . /path/to/saba.ps1

# Elvish用
saba completion elvish > ~/.elvish/lib/completions/saba.elv
```

#### インストール後の確認
補完スクリプトをインストールした後、新しいシェルセッションを開始するか、設定を再読み込みしてください：

```bash
# Bash
source ~/.bashrc

# Zsh
source ~/.zshrc

# Fish
# 自動的に読み込まれる
```

### 実装の詳細
- **実装**: `clap_complete`クレートを使用した自動補完スクリプト生成
- **カスタムEnum**: `CompletionShell`で各シェルタイプを定義し、`clap_complete::Shell`に変換
- **CLI構造再現**: `build_cli()`関数でmain.rsと同じCLI構造を再現し、補完スクリプト生成に使用
- **標準出力**: 補完スクリプトは標準出力に出力されるため、リダイレクトしてファイルに保存

### 技術的な課題と解決
1. **ValueEnum derive macro**: `clap`の`derive`機能を有効化する必要があった
   - 解決: `Cargo.toml`で`clap = { version = "4.4.6", features = ["derive"] }`に変更

2. **型推論エラー**: Generator traitのジェネリック型`G`が推論できなかった
   - 解決: `let shell_type: Shell = shell.clone().into();`で明示的な型注釈を追加

## 依存関係

主な依存関係:
- `clap`: CLIの引数パース（`derive`機能有効化）
- `clap_complete`: シェル補完スクリプト生成
- `anyhow`: エラー処理
- `askama`: テンプレートエンジン
- `inquire`: 対話型プロンプト
- `regex`: パターンマッチング（ContentUpdater用）
- `yaml-rust`: YAMLパース
