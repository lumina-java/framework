## Summary
Implementasi Database Layer untuk Lumina Framework.

Closes #2

## Changes
- `src/database/connection.rs` — DatabasePool wrapper sqlx::Pool Sqlite
- `src/database/model.rs` — Model trait (find/all/save/delete + soft delete)
- `src/database/migration.rs` — Migration runner otomatis dari database/migrations
- `src/app/models/user.rs` — Contoh implementasi Model trait untuk tabel users
- `database/migrations/0001_create_users_table.sql` — Schema tabel users dengan soft delete
- `src/core/application.rs` — Init DB pool + auto-migrate saat startup
- `Cargo.toml` — Tambah sqlx 0.8 (sqlite, postgres, macros)
- `.env.example` — Tambah DATABASE_URL

## Testing
- cargo check: 0 errors, 0 warnings
- Server startup otomatis connect DB dan run migrations
- Skill file: skill_for_creates/database_layer.md
