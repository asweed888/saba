# saba

`saba`は、YAML仕様からコードを生成する宣言的開発フレームワークです。複数のプログラミング言語をサポートし、オニオンアーキテクチャによるドメイン駆動設計（DDD）の原則に従います。

## 特徴

- **宣言的開発**: シンプルなYAMLファイルでプロジェクト構造を定義
- **マルチ言語サポート**: Rust、Go、Python、TypeScript、Bash、Lua
- **DDDアーキテクチャ**: オニオンアーキテクチャによるオプションのドメイン駆動設計
- **対話的セットアップ**: ガイド付きプロジェクト初期化
- **テンプレートベース生成**: テンプレートを使用した柔軟なコード生成

## インストール

インストールスクリプトを使用してインストール:

```bash
curl -sSL https://raw.githubusercontent.com/asweed888/saba/main/install.sh | bash && exec $SHELL -l
```

初回インストール後のアップデート:

```bash
saba_install
```

## クイックスタート

1. **新しいプロジェクトの初期化**:
   ```bash
   saba new
   ```
   以下の選択肢が表示されます:
   - プログラミング言語 (rust, go, python, typescript, bash, lua)
   - DDDアーキテクチャを使用するかどうか（対応言語の場合）

2. **仕様からコードを生成**:
   ```bash
   saba up
   ```

## 設定

`saba.yml`ファイルでプロジェクト構造を定義します:

```yaml
lang: rust
arch: ddd
spec:
- location: domain
  upstream:
    - name: model
      upstream:
        - name: fish
          codefile:
            - name: entity
    - name: repository
      codefile:
        - name: fish

- location: infrastructure
  upstream:
    - name: repository
      codefile:
        - name: fish

- location: usecase
  codefile:
    - name: fish

- location: presentation
  upstream:
    - name: http
      upstream:
        - name: handler
          codefile:
            - name: aquarium
```

### 設定オプション

- **`lang`**: 対象プログラミング言語 (`rust`, `go`, `python`, `typescript`, `bash`, `lua`)
- **`arch`**: アーキテクチャタイプ (`ddd` はドメイン駆動設計用)
- **`spec`**: プロジェクト構造定義
  - **`location`**: トップレベルディレクトリ/モジュール
  - **`upstream`**: サブモジュールやネストした構造
  - **`codefile`**: 生成する個別ファイル

### アーキテクチャタイプ

#### DDD（ドメイン駆動設計）
レイヤーを分離したクリーンアーキテクチャを生成:
- `domain`: コアビジネスロジックとエンティティ
- `infrastructure`: 外部関心事（データベース、API）
- `usecase`: アプリケーションビジネスロジック
- `presentation`: ユーザーインターフェースとコントローラー

#### シンプル（非DDD）
基本構造を生成:
- `greeter`: アビリティトレイトファイルを含むシンプルなモジュール

## コマンド

- `saba new`: 対話的プロンプトで新しいプロジェクトを初期化
- `saba up`: 現在の`saba.yml`仕様に基づいてコードを生成
- `saba --help`: ヘルプ情報を表示
- `saba --version`: バージョン情報を表示

## 例

### Rust DDDプロジェクト
```yaml
lang: rust
arch: ddd
spec:
- location: domain
  upstream:
    - name: model
      upstream:
        - name: user
          codefile:
            - name: entity
        - name: order
          codefile:
            - name: entity
    - name: repository
      codefile:
        - name: user
        - name: order
```

### シンプルなPythonプロジェクト
```yaml
lang: python
spec:
- location: greeter
  codefile:
    - name: abils
```

## 開発

```bash
# プロジェクトをビルド
cargo build

# リリースバージョンをビルド
cargo build --release

# テストを実行
cargo test

# 引数付きで実行
cargo run -- new
cargo run -- up
```

## ライセンス

このプロジェクトは現在活発に開発中で、まだv1.0に達していません。フレームワークの進化に伴い、APIや機能が変更される可能性があります。