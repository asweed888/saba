use clap::Command;
use anyhow::Result;

pub fn spec() -> Command {
    Command::new("describe")
        .about("Instructs Claude Code to document saba.yml specification in CLAUDE.md")
        .long_about(
            "Instructs Claude Code (claude.ai/code) to add or update the 'saba.ymlの書き方' section \
            in CLAUDE.md with the latest saba.yml specification and usage patterns."
        )
}

pub fn action() -> Result<()> {
    print_claude_code_instruction();
    Ok(())
}

fn print_claude_code_instruction() {
    let version = env!("CARGO_PKG_VERSION");
    println!(r#"
# Instruction for Claude Code

Claude Code (claude.ai/code) へ:

プロジェクトのCLAUDE.mdファイルに「## saba.ymlの書き方」という見出しで、
saba.yml設定ファイルの書き方に関する詳細なドキュメントを追加してください。

## sabaとは

sabaは、YAML仕様に基づいてコードを生成する宣言的開発フレームワークです。

### 主な機能
- **マルチ言語対応**: Rust、Go、Python、TypeScript、JavaScript、Markdownの6言語をサポート
- **プロジェクト構造の自動生成**: saba.ymlに記述したモジュール構造を元に、ディレクトリとコードファイルを自動生成
- **設定ファイルの自動生成**: 各言語に応じた設定ファイル（Cargo.toml、package.json、go.modなど）を自動作成
- **マルチプロジェクト対応**: 複数の言語のプロジェクトを一つのワークスペースで管理可能

### sabaを使うメリット
1. **構造設計に集中**: ファイル作成の手間を削減し、アーキテクチャ設計に集中できる
2. **一貫性の確保**: 命名規則やモジュール構成を統一的に管理
3. **迅速なプロトタイピング**: 新規プロジェクトやモジュールの追加が高速
4. **リファクタリングの容易さ**: saba.ymlを編集するだけで構造を再編成可能

### 基本的な使い方
```bash
# プロジェクト初期化（対話モード）
saba new

# プロジェクト初期化（言語指定）
saba new --lang rust

# saba.ymlに基づいてコードを生成
saba up
```

## 指示内容

1. **既存セクションの確認**
   - CLAUDE.mdに既に「saba.ymlの書き方」セクションが存在するか確認
   - 存在する場合は、内容が最新の仕様（v{}）と一致するか確認

2. **セクションの追加/更新**
   - セクションが存在しない場合：新規に追加
   - セクションが古い場合：最新の仕様に基づいて更新
   - セクションが最新の場合：更新不要

3. **記載すべき内容**

   以下の項目を含めてください：

   ### 基本構造
   - saba.ymlの基本的なYAML配列構造
   - 必須フィールド（name, lang）
   - オプションフィールド（root, upstream, codefile）

   ### サポート言語
   - rust, go, python, typescript, javascript, markdown
   - 各言語の特徴と生成されるファイル

   ### フィールド詳細
   - **name**: プロジェクト名（kebab-case推奨）
   - **root**: 単一プロジェクトモードのフラグ（true/false）
   - **lang**: プログラミング言語の指定
   - **upstream**: モジュール/ディレクトリの階層構造
   - **codefile**: 各レベルのコードファイル定義

   ### ファイル拡張子の処理
   - 拡張子あり（Button.tsx, Modal.vue）：そのまま保持
   - 拡張子なし（utils）：言語固有の拡張子を自動付与

   ### 言語別のプロジェクト例
   各サポート言語について、実際のsaba.yml例を記載：
   - **Rust**: src/ディレクトリ構造、Cargo.toml、ワークスペース対応
   - **TypeScript**: package.json、tsconfig.json、バレルエクスポート
   - **Go**: go.mod、パッケージ命名規則
   - **Python**: setup.py、__init__.py、バレルインポート
   - **JavaScript**: package.json、ESモジュール対応
   - **Markdown**: ドキュメント構造、README.md

   ### マルチプロジェクト構成
   - 複数言語の混在例
   - ワークスペース自動生成の仕組み
   - root: trueフラグの管理

   ### ベストプラクティス
   - プロジェクト命名規則
   - モジュール構成の推奨パターン
   - ファイル拡張子の指定方針

   ### saba.yml管理対象
   - **推奨対象**: コードファイル、ドキュメントファイル、モジュールディレクトリ
   - **非推奨対象**: アセットファイル、設定ファイル、ビルド成果物

## 参考情報

現在のsabaバージョン: v{}

既存のguideコマンド出力内容も参考にしてください。
特に「saba.yml Configuration Guide」セクション以降の内容が有用です。

## 実行後の確認

- CLAUDE.mdに「saba.ymlの書き方」セクションが正しく追加/更新されたか確認
- 日本語で記載されているか確認（プロジェクトのCLAUDE.mdは日本語で記載）
- コード例が適切に記載されているか確認

---

上記の指示に従って、CLAUDE.mdを更新してください。
"#, version, version);
}
