# Saba v2.0 仕様書

## 概要

Saba v2.0は、Claude Codeが効率的にプロジェクト構造を生成するためのツールとして方針転換します。

## 基本コンセプト

### 1. Claude Codeによるsaba.yml作成
- Claude Codeがユーザーの要件を理解し、適切なsaba.ymlを生成
- 人間が手動で設定ファイルを書く必要がない
- Claude Codeの理解に基づいた最適な構造設計

### 2. Claude Codeによるsabaコマンド実行
- Claude CodeがBashツールを使用してsabaコマンドを実行
- 生成プロセスをClaude Codeが自動化
- エラーハンドリングもClaude Codeが対応

### 3. ディレクトリ・モジュール構造の生成
- プログラミング言語に応じた適切なディレクトリ構造
- モジュール間の依存関係を考慮した配置
- Claude Codeが自由に構造を設計可能（特定アーキテクチャに依存しない）

## v1からの主要な変更点

### 1. アーキテクチャ制約の撤廃
- **v1**: DDD等の特定アーキテクチャパターンに準拠
- **v2**: Claude Codeが要件に応じて自由に構造を設計
- **理由**: Claude Codeが任意の構造を理解・生成できるため、特定パターンに制限する必要がない

### 2. ファイル生成方針の変更
- **v1**: テンプレートに基づいたファイル内容を生成
- **v2**: 基本的に空ファイルを生成、Claude Codeが後から実装
- **例外**: 
  - Rust: `mod.rs`, `main.rs`, `lib.rs` → モジュール宣言等の必要な内容を生成
  - 他言語: 同様の構造ファイルのみ内容を生成

### 3. 設定ファイル形式の進化
- **マルチプロジェクト対応**: 複数のサブプロジェクトを一つのsaba.ymlで管理
- **言語混在プロジェクト**: backend（Rust）+ frontend（TypeScript）等の構成に対応
- **柔軟な階層構造**: upstreamによる任意の深さのネスト構造
- **フィールド名の統一**: `location` → `name`に統一

### 4. ディレクトリ生成の明示化
- **v1**: Rustで`src`ディレクトリを自動でスキップ
- **v2**: すべてのディレクトリを明示的に指定する必要
- **理由**: 言語固有の特別扱いを廃止し、Claude Codeが完全に制御可能に

## 設定ファイル仕様（v2）

### プロジェクト設定形式

#### シングルプロジェクト
```yaml
- name: [プロジェクト名]
  root: true
  lang: [言語名]
  upstream:
    - name: [ディレクトリ名]
      # 階層構造
```

#### マルチプロジェクト
```yaml
- name: [プロジェクト名]
  lang: [言語名]
  upstream:
    - name: [ディレクトリ名]
      # 階層構造

- name: [別プロジェクト名]
  lang: [別言語名]
  upstream:
    # 別プロジェクトの構造
```

### 重要な仕様変更

#### 1. プロジェクト指定方法
- **すべてのプロジェクト**: `name`フィールドが必須
- **シングルプロジェクト**: `root: true`を追加
- **マルチプロジェクト**: `root`フィールドなし（またはfalse）

#### 2. 言語設定の継続（モジュール構造生成のため）
- `lang:`フィールドは必須（モジュール構造ファイルの内容生成に必要）
- Rust: `mod.rs`, `main.rs`, `lib.rs`のモジュール宣言
- Go: `package`宣言
- Python: `__init__.py`のimport文
- 等、言語固有の構造ファイル内容を正しく生成

#### 3. ファイル拡張子の柔軟な指定
- **拡張子あり**: 明示的に指定された拡張子を採用（`button.vue`）
- **拡張子なし**: `lang`フィールドに基づいて自動付与（`model` → `model.rs`）
- **利点**: Claude Codeが状況に応じて拡張子を制御可能

#### ファイル拡張子処理ルール
1. codefileに拡張子が含まれている場合 → その拡張子を採用
2. codefileに拡張子がない場合 → langフィールドの言語に応じた拡張子を自動付与
   - `lang: rust` → `.rs`
   - `lang: go` → `.go`
   - `lang: python` → `.py`
   - `lang: javascript` → `.js`
   - `lang: typescript` → `.ts`
   - など

### 実際の例

#### シングルプロジェクト（Rust）
```yaml
- name: app
  root: true
  lang: rust
  upstream:
    - name: src
      upstream:
        - name: users
          upstream:
            - name: domain
              codefile:
                - name: model      # → model.rs として生成
                - name: repository # → repository.rs として生成
```

#### マルチプロジェクト（Rust + JavaScript）
```yaml
- name: backend
  lang: rust
  upstream:
    - name: src
      upstream:
        - name: users
          upstream:
            - name: domain
              codefile:
                - name: model      # → model.rs として生成
                - name: repository # → repository.rs として生成

- name: frontend
  lang: javascript  # JavaScriptプロジェクトだが...
  upstream:
    - name: src
      upstream:
        - name: components
          codefile:
            - name: button.vue # → 明示的な.vue拡張子を採用
```

#### 生成結果例
- `model` (lang: rust) → `model.rs`
- `repository` (lang: rust) → `repository.rs`
- `button.vue` (lang: javascript) → `button.vue` (拡張子指定優先)