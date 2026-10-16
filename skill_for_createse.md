# Lumina Framework: Creator's Skill Plan (Global Install & Smart Scaffolding)

Plan ini mendokumentasikan alur instalasi global dan sistem scaffolding CRUD yang baru saja kita selesaikan.

## 🎯 Tujuan Utama
Menjadikan Lumina framework yang mudah diinstall dan digunakan oleh developer lain melalui ekosistem Cargo.

## 🛠️ Alur Instalasi Global
1. **User Command**: `cargo install lumina-framework`
2. **Setup**: Binary `lumina` masuk ke global PATH.
3. **Usage**: `lumina new my-app`
4. **Action**: CLI melakukan `git clone` dari skeleton resmi dan menyiapkan environment.

## 🪄 Smart Scaffolding CRUD
Mekanisme pembuatan modul lengkap dalam satu perintah:
`lumina make:crud Contact name:string:required email:string:email phone:string`

### Komponen yang Dihasilkan:
- **Model**: `src/app/models/contact.rs` (dengan SQL bindings).
- **Migration**: `database/migrations/YYYYMMDD_create_contacts_table.sql`.
- **Service**: `src/app/services/contact_service.rs` (Business logic).
- **Request**: `src/app/requests/contact_request.rs` (Validation rules).
- **Controller**: `src/app/controllers/contact_controller.rs` (MVC handler).
- **Views**: `resources/views/contact/*.blade.rs` (CRUD interface).

## 🐳 Deployment & Infrastructure
- `make:docker`: Konfigurasi kontainer siap produksi.
- `make:nginx`: Reverse proxy dengan standar keamanan tinggi.
- `make:supervisor`: Persistence management.
- `backup:run`: Keamanan data otomatis.

---
*Dibuat untuk referensi pengembangan Lumina di masa depan.*
