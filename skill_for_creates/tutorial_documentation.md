# 📚 Lumina Framework - Tutorial & Documentation Skill

## 🎯 Goal
Membuat tutorial lengkap penggunaan Lumina Framework dari nol hingga aplikasi CRUD berjalan,
dan mengintegrasikannya ke dalam DOCUMENTATION.md dan README.md.

---

## 📋 Scope Tutorial

Tutorial mencakup:
1. Setup & Konfigurasi awal
2. Membuat Controller pertama
3. Mendefinisikan Route (Web & API)
4. Membuat Model + Database
5. Validasi Request
6. Template Engine (Tera)
7. Autentikasi JWT
8. Middleware
9. Service Layer pattern

---

## 📁 File yang Dibuat/Diperbarui

```
lumina/
├── DOCUMENTATION.md        ← [NEW] Dokumentasi lengkap & tutorial
├── README.md               ← [UPDATE] Quick start + link ke DOCUMENTATION.md
└── skill_for_creates/
    └── tutorial_documentation.md  ← [NEW] Skill ini
```

---

## ✅ Checklist Pembuatan

- [x] Skill file dibuat (file ini)
- [x] DOCUMENTATION.md dibuat
- [x] README.md diperbarui
- [x] GitHub issue dibuat

---

## 📝 Konten DOCUMENTATION.md

DOCUMENTATION.md harus mencakup:

### Struktur
```
1. Pendahuluan & Arsitektur
2. Instalasi
3. Konfigurasi (.env)
4. Routing
   - Web Routes
   - API Routes
   - Route Parameters
   - Route Groups & Middleware
5. Controller
   - Membuat Controller
   - HTML Response
   - JSON Response
6. Model & Database
   - Model Trait
   - CRUD Operations
   - Soft Delete
7. Validasi
   - ValidatedJson
   - ValidatedForm
   - Custom Rules
8. Template Engine (Tera)
   - Rendering View
   - Context & Variables
   - Looping & Conditionals
9. Autentikasi
   - Register
   - Login (JWT)
   - Protected Routes
10. Service Layer
11. Middleware
12. CLI Tools
```

---

## 🔖 Referensi Implementasi

Semua contoh kode diambil dari source aktual:
- `src/core/router.rs` → Router API
- `src/core/validation.rs` → Validasi
- `src/core/view.rs` → Template engine
- `src/core/auth/` → JWT & hashing
- `src/app/controllers/` → Contoh controller
- `src/app/models/user.rs` → Contoh model
- `routes/web.rs` & `routes/api.rs` → Contoh routing
- `src/app/services/auth_service.rs` → Contoh service

---

**End of Skill** 🚀
