# Skill 3: Implementasi Session & Flash Messages

## Deskripsi / Tujuan
Untuk pengembangan web berbasis HTML (non-SPA), *framework* harus memiliki manajemen sesi (*Session*) agar bisa menyimpan data otentikasi login klasik (cookie) dan *Flash Messages* (pesan notifikasi sekali tayang seperti "Data berhasil disimpan").

## Kebutuhan Teknis
- Gunakan crate seperti `tower-sessions` atau implementasi custom *middleware* berbasis `axum`.
- Integrasikan *session storage* bawaan (Memory atau SQLite `sqlx` store).
- Sediakan helper yang ramah untuk set dan get *Flash Messages* (bisa dipanggil di Controller dan langsung diteruskan ke Tera views).

## Langkah-langkah Implementasi
1. Tambahkan dependensi `tower-sessions` atau crate session sejenis di `Cargo.toml`.
2. Tambahkan layer middleware session di `src/core/application.rs` saat inisialisasi `AxumRouter`.
3. Buat helper/ekstraktor di `src/core/session.rs` untuk memudahkan memanggil `request.session().set_flash("success", "Berhasil!")`.
4. Inject data *Flash Message* secara otomatis ke `Context` Tera setiap kali *view* dirender agar *view* bisa menampilkan elemen HTML notifikasi.

## Instruksi Git & Pull Request
- Buat *branch* baru: `git checkout -b feature/session-flash-messages`.
- Tulis implementasi di *branch* tersebut.
- Ajukan Pull Request saat selesai, jangan langsung *merge* ke `master`.

## Panduan User Test Manual
1. Buat rute `GET /set-flash` yang mengisi flash message: "Item berhasil dihapus" lalu melakukan *redirect* ke `/show-flash`.
2. Buat rute `GET /show-flash` yang merender sebuah view HTML biasa.
3. Di dalam view HTML tersebut, tambahkan blok pengecekan `{% if flash.success %} <div class="alert">{{ flash.success }}</div> {% endif %}`.
4. Akses `http://localhost:3000/set-flash` dari browser.
5. Anda akan di-redirect ke `/show-flash` dan melihat alert "Item berhasil dihapus".
6. Refresh halaman `/show-flash`. Pesan tersebut harus *hilang* (karena sifatnya flash message yang hanya berlaku satu kali request).
