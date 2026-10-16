# ✨ Lumina Framework Extension for VS Code & Antigravity IDE

Official developer tooling for **Lumina Framework** — bringing the elegant Developer Experience of Laravel to Rust!

## 🚀 Fitur Utama

### 1. Rust Controller, Model & ORM Snippets
Di dalam file `.rs`:

| Shortcut | Hasil Snippet | Deskripsi |
| :--- | :--- | :--- |
| `l-prelude` | `use crate::app::prelude::*;` | Import seluruh model, request, dan core Lumina dalam 1 baris |
| `l-ctrl` | `ResourceController` | Template Controller lengkap dengan `req: Request` & `LuminaResult` |
| `l-model` | `#[derive(LuminaModel)]` | Struct Model Eloquent Active Record lengkap |
| `l-action` | `pub async fn name(req: Request) -> LuminaResult` | Handler tunggal ekspresif |
| `l-paginate` | `let (page, per_page) = req.pagination();` | Paginasi otomatis |
| `l-find` | `match Model::find(req.db(), id).await?` | Pencarian record dengan 404 otomatis |
| `l-save` | `item.save(req.db()).await?;` | Active record INSERT/UPDATE |
| `l-destroy` | `Model::destroy(req.db(), id).await?;` | Soft delete record |
| `l-routes-crud`| `Router::new().get(...).post(...)` | Definisi RESTful router |

### 2. Blade + Tera Template Support (`.blade.rs`)
- Syntax highlighting untuk file template `.blade.rs` dan `.html`.
- Snippets direktif: `@if`, `@foreach`, `@extends`, `@section`, `@yield`, `@csrf`.
- Snippets Tera: `{{ variable }}`, `{% tag %}`.

---

## 🛠️ Cara Pasang di VS Code / Antigravity IDE

### Opsi A: Pasang Langsung untuk Development Lokal (Instan)
Salin folder `vscode-lumina` ke direktori ekstensi VS Code pengguna:

```bash
# Di PowerShell:
Copy-Item -Recurse "d:\FRAMEWORK RUST\framework\extensions\vscode-lumina" "$HOME\.vscode\extensions\lumina-vscode"
```
Setelah disalin, restart atau reload jendela VS Code (`Ctrl+Shift+P` -> `Developer: Reload Window`). Ekstensi dan seluruh snippet langsung aktif!

### Opsi B: Package ke File `.vsix`
Jika sudah menginstal `vsce` (`npm install -g @vscode/vsce`):
```bash
cd extensions/vscode-lumina
vsce package
```
Lalu instal file `.vsix` yang dihasilkan lewat menu Extensions -> `Install from VSIX...`.
