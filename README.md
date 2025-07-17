# saba

`saba`は、YAML仕様からコードを生成する宣言的開発フレームワークです。複数のプログラミング言語をサポートし、シンプルなYAML設定で効率的なプロジェクト構造を生成します。

## 特徴

- **宣言的開発**: シンプルなYAMLファイルでプロジェクト構造を定義
- **マルチ言語サポート**: Rust、Go、Python、TypeScript、JavaScript
- **デュアルモード**: インタラクティブな手動選択とAI向け自動化の両方に対応
- **マルチプロジェクト**: 単一設定ファイルで複数のプロジェクトを管理
- **ファイル保護**: 既存コードを保護しながら構造管理を実現

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
   
   **人間向けモード（インタラクティブ）**:
   ```bash
   saba new
   ```
   対話的な言語選択プロンプトが表示されます
   
   **AI向けモード（自動化）**:
   ```bash
   saba new --lang rust
   saba new --lang typescript
   saba new --lang go
   saba new --lang python
   saba new --lang javascript
   ```

2. **仕様からコードを生成**:
   ```bash
   saba up
   ```

## 設定

`saba.yml`ファイルでプロジェクト構造を定義します:

```yaml
- name: my-app
  root: true
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: Button.tsx
            - name: Modal.vue
            - name: utils
      codefile:
        - name: index
```

### 設定オプション

- **`name`**: プロジェクト名
- **`root`**: ルートプロジェクトかどうか（`true`の場合、現在のディレクトリに直接生成）
- **`lang`**: 対象プログラミング言語 (`rust`, `go`, `python`, `typescript`, `javascript`)
- **`upstream`**: サブモジュールやネストした構造
- **`codefile`**: 生成する個別ファイル（拡張子があれば保持、なければ言語に応じて付与）

### マルチプロジェクト構成

複数のプロジェクトを1つの`saba.yml`で管理できます:

```yaml
- name: frontend
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: App.tsx

- name: backend  
  lang: rust
  upstream:
    - name: src
      codefile:
        - name: main
      upstream:
        - name: handlers
          codefile:
            - name: user
```

- 最初のプロジェクトには`root: true`が自動設定
- 2つ目以降のプロジェクトは個別ディレクトリに生成
- Rustの場合、複数プロジェクトでワークスペース構成を自動生成

## コマンド

- `saba new`: 対話的プロンプトで新しいプロジェクトを初期化
- `saba new --lang <言語>`: 指定言語で新しいプロジェクトを初期化（AI向け）
- `saba up`: 現在の`saba.yml`仕様に基づいてコードを生成
- `saba guide`: Claude Code向けの包括的使用ガイドを表示
- `saba --help`: ヘルプ情報を表示
- `saba --version`: バージョン情報を表示

## 例

### Rustプロジェクト
```yaml
- name: my-rust-app
  root: true
  lang: rust
  upstream:
    - name: src
      codefile:
        - name: main
      upstream:
        - name: handlers
          codefile:
            - name: user
            - name: order
        - name: models
          codefile:
            - name: user
            - name: order
```

### TypeScriptプロジェクト
```yaml
- name: my-web-app
  root: true
  lang: typescript
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: Button.tsx
            - name: Modal.vue
            - name: utils.ts
      codefile:
        - name: index
```

### Goプロジェクト
```yaml
- name: my-go-app
  root: true
  lang: go
  upstream:
    - name: pkg
      upstream:
        - name: models
          codefile:
            - name: user
        - name: handlers
          codefile:
            - name: api
  codefile:
    - name: main
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

## ファイル保護システム

sabaは3層のファイル保護システムを実装しています:

1. **コードファイル（完全保護）**: `.rs`, `.go`, `.py`, `.js`, `.ts`, `.tsx`, `.vue`等
   - 一度作成されたら決して上書きされません
   
2. **管理ファイル（部分更新）**: `mod.rs`, `__init__.py`, `index.ts`等
   - sabaマーカー間のコンテンツのみ更新、カスタムコードは保護
   
3. **設定ファイル（初回のみ）**: `package.json`, `Cargo.toml`, `go.mod`等
   - 存在しない場合のみ作成

## バージョン

現在のバージョン: **v2.0.2**

v2.0では以下の重要な変更が行われました:
- シンプルで直感的なYAML構造に変更
- マルチプロジェクト対応
- ファイル保護システムの実装
- AI向け自動化機能の追加

## ライセンス

このプロジェクトはv2.0に達し、安定したAPIを提供しています。