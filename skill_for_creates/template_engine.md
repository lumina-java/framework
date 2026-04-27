# 🎨 Lumina Framework — Template Engine Skill

## 🎯 Goal
Mengganti respons HTML *hardcoded* di controller dengan **Template Engine** sesungguhnya menggunakan `Tera` (terinspirasi dari Jinja2/Blade). Ini memisahkan *logic* (Controller) dengan *presentation* (View).

**Fitur yang dibangun:**
- Integrasi crate `tera`.
- `View` service di dalam `Application` state agar bisa diakses oleh controller.
- Struktur folder `resources/views/` dengan kapabilitas layouting (extends/blocks).
- Refaktor `HomeController` dan `UserController` agar mengembalikan respons *rendered template*.

---

## 📁 Perubahan File

```
Cargo.toml                              ← MODIFY: tambah tera

src/core/
└── view.rs                             ← NEW: Wrapper Tera engine

src/core/application.rs                 ← MODIFY: Inject Tera ke axum state

src/app/controllers/
├── home_controller.rs                  ← MODIFY: Return view() bukan string
└── user_controller.rs                  ← MODIFY: Return view() bukan string

resources/views/
├── layout.html                         ← NEW: Master layout (header, footer, css)
├── home/
│   ├── index.html                      ← NEW: Homepage template
│   └── about.html                      ← NEW: About template
└── users/
    ├── index.html                      ← NEW: Users list template
    └── show.html                       ← NEW: User detail template
```

---

## 📝 Konsep Implementasi

### 1. `Cargo.toml`
```toml
[dependencies]
tera = "1.19"
```

### 2. `src/core/view.rs`
Wrapper untuk menginisialisasi Tera, me-load semua file dari `resources/views/**/*.html`, dan me-rendernya dengan konteks (variabel).

### 3. Controller Rendering
Controller akan menerima `State` yang berisi engine rendering dan mengembalikan `Html`.
```rust
use axum::{extract::State, response::Html};
use tera::Context;

pub async fn index(State(view): State<ViewEngine>) -> Html<String> {
    let mut context = Context::new();
    context.insert("title", "Welcome to Lumina");
    
    let rendered = view.render("home/index.html", &context);
    Html(rendered)
}
```

### 4. Layouting HTML (Tera)
Di `resources/views/layout.html`:
```html
<!DOCTYPE html>
<html>
<head><title>{% block title %}Lumina{% endblock %}</title></head>
<body>
    {% block content %}{% endblock %}
</body>
</html>
```

Di `resources/views/home/index.html`:
```html
{% extends "layout.html" %}
{% block title %}Home - Lumina{% endblock %}
{% block content %}
    <h1>Selamat datang!</h1>
{% endblock %}
```

---

## ✅ Validation Checklist

1. Akses `http://localhost:8000/` di browser.
   - Harapan: Menampilkan HTML yang dirender oleh Tera (berasal dari `home/index.html` dan `layout.html`).
2. Akses `http://localhost:8000/users/42`.
   - Harapan: Menampilkan profil user ID 42 melalui `users/show.html`.
3. Verifikasi performa response time di terminal log tetap cepat.

---

## 📌 Urutan Eksekusi
Skill ini akan dieksekusi via pull request ke master setelah CLI Tool selesai.
