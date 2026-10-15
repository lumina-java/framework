# Lumina Skill: Service Layer & Business Logic Isolation

## Overview
Lumina mengikuti pola arsitektur **Service-Controller** untuk memastikan logika bisnis terisolasi dari protokol HTTP (Axum).

## Standards & Patterns

### 1. Lokasi & Struktur
Semua logika bisnis diletakkan di `src/app/services/`.
- **Controller**: Menangani request/response, validasi input, dan memanggil Service.
- **Service**: Menangani operasi database kompleks, integrasi API eksternal, dan kalkulasi bisnis.

### 2. Implementasi Service
Setiap service biasanya merupakan struct yang memiliki akses ke `DatabasePool`.

```rust
pub struct PharmacyService {
    db: Arc<DatabasePool>,
}

impl PharmacyService {
    pub async fn calculate_stock(&self, drug_id: i64) -> i32 {
        // Logika FIFO / Average di sini
    }
}
```

### 3. Flash Messages (Notification)
Gunakan sistem Flash Message untuk memberikan feedback setelah redirect.
- **Success**: Notifikasi warna hijau (Pharmacy Green).
- **Error**: Notifikasi warna merah (Alert Red).

## Benefits
- **Testability**: Logika bisnis bisa ditest tanpa harus mensimulasikan HTTP request.
- **Maintainability**: Memudahkan migrasi atau perubahan alur bisnis tanpa menyentuh route/controller.
- **Compliance**: Sesuai dengan standar pengembangan SIMRS Pharmacy.
