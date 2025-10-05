# UefiEditor



## 概要
Rust + UEFI で動作するUEFI 変数エディタ

---

## ファイル構成

```
esp/
└── EFI/
    └── BOOT/
        └── BOOTX64.efi   ← build 済みの実行ファイル
```

---

##  実行方法


```
BOOTX64.efi "VariableName" "Guid"
```

例：
```
Shell> BOOTX64.efi MyVar 12345678-1234-5678-1234-56789ABCDEF0
```

---

##  操作方法

| 操作 | 内容 |
|------|------|
| **Ctrl + S** | 現在編集中の変数を保存（`SetVariable()` 呼び出し） |
| **Esc**  | アプリケーション終了 |

---

##  ビルド方法

1. Rust とターゲットをインストール
   ```bash
   rustup target add x86_64-unknown-uefi
   ```

2. ビルド
   ```bash
   cargo build --target x86_64-unknown-uefi --release
   ```

3. 出力ファイル
   ```
   target/x86_64-unknown-uefi/release/UefiEditor.efi
   ```

4. コピーして配置
   ```bash
   cp target/x86_64-unknown-uefi/release/UefiEditor.efi esp/EFI/BOOT/BOOTX64.efi
   ```

---


##  今後の拡張予定

- バイナリデータ編集（16進ビューア）
- 新規変数作成
- 変数属性（BootServiceAccess / RuntimeAccess）の切替
- エラーメッセージ改善
- ユーザインタフェースの強化（カラー表示、メニューUI）

---

