# Skill 4: Implementasi File Storage System

## Deskripsi / Tujuan
Membangun modul atau *service layer* untuk mengelola sistem penyimpanan file statis dan unggahan pengguna (*uploads*). Fasilitas ini harus meniru API `Storage::disk('local')->put()` ala Laravel dan menggunakan direktori `storage/` atau `public/uploads/` yang sudah ada di proyek.

## Kebutuhan Teknis
- Abstraksi modul *Storage* dengan driver minimal: `Local`.
- Harus bisa melayani (*serve*) file statis dengan aman ke klien web (konfigurasi `ServeDir` pada axum).
- Method minimal: `put()`, `get()`, `delete()`, `exists()`, `url()`.

## Langkah-langkah Implementasi
1. Buat file `src/core/storage.rs`.
2. Definisikan struct `Storage` atau `Disk` yang memiliki konfigurasi *base path* ke direktori lokal proyek (contoh: `./storage/app/public/`).
3. Implementasikan *method* untuk menulis *bytes* ke file lokal (`put`), mengecek eksistensi file (`exists`), dan menghapusnya (`delete`).
4. Di `application.rs` atau `router.rs`, daftarkan direktori storage/public tersebut sebagai direktori aset statis agar file bisa diakses lewat URL browser (contoh: `/storage/foto.png`).

## Instruksi Git & Pull Request
- Buat branch: `git checkout -b feature/file-storage-system`.
- Selesaikan modifikasi dan tambahkan file yang relevan.
- Buat Pull Request ke *repository* utama.

## Panduan User Test Manual
1. Tambahkan sebuah endpoint API khusus untuk upload file: `POST /api/upload`.
2. Controller `upload` membaca isi dari `multipart/form-data` *request*.
3. Panggil method `Storage::disk("local").put("avatar.png", bytes).await;`.
4. Kirim respon berupa teks berisi URL gambar tersebut.
5. Gunakan Postman untuk melakukan `POST` file gambar.
6. Cek manual di folder `storage/` pada sistem operasi, pastikan file gambar berhasil disimpan.
7. Akses URL gambar tersebut di browser dan pastikan gambar berhasil di-load dengan sukses (status 200).
