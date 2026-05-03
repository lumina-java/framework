# Issue: Gagal Proses Buat Form Upload Image, View, dan Controller (Backend Layout Integration)

## Deskripsi Masalah
Ketika pertama kali menjalankan perintah CRUD CLI untuk `UploadImage` (`cargo run --bin lumina -- make:crud UploadImage`), terjadi beberapa masalah pada halaman backend:

1. **Error Rendering Variabel Context (`old.name` & `errors.name` tidak ditemukan)**:
   Pada saat mengakses halaman `/upload_images/create`, Tera template engine gagal merender form karena variabel `old.name` belum ada di dalam konteks session. Hal ini menyebabkan server crash dengan pesan error:
   ```
   Variable `old.name` not found in context while rendering 'upload_image/create.html'
   ```
2. **Template Backend Tidak Muncul (Unstyled & No Sidebar)**:
   Halaman CRUD bentukan CLI awalnya menggunakan `layout.html` (Bootstrap) sehingga ketika diakses melalui menu sidebar dashboard yang berbasis Tailwind CSS, halaman CRUD tidak memiliki sidebar dan tampilannya polos (unstyled).
3. **Redirect & Parameter `.go()` Tidak Cocok**:
   Rute CRUD controller sebelumnya menggunakan `.go(req.token, &req.session)` yang sudah deprecated dan tidak sesuai dengan parameter fungsi yang baru `.go(&req)`.

---

## Solusi yang Telah Diterapkan

### 1. Perbaikan pada View Engine (Thread-Local Context Fallbacks)
Kami memperbarui file `src/core/view.rs` untuk secara otomatis menyuntikkan fallback `old.name = ""` dan `errors.name = ""` jika data belum ada di session. Hal ini mencegah Tera dari error rendering:
```rust
let mut old_with_defaults = serde_json::json!({ "name": "" });
if let Some(obj) = old.as_object() {
    if let Some(merge) = old_with_defaults.as_object_mut() {
        for (k, v) in obj { merge.insert(k.clone(), v.clone()); }
    }
}
context.insert("old", &old_with_defaults);
```

### 2. Auto-Injection Informasi User ke Semua View
Menambahkan otomatisasi penyuntikan context `email`, `user_id`, dan `role` dari session user yang sedang aktif ke dalam semua view. Sehingga template dapat merender detail profile user di header layout backend tanpa perlu mempassing manual di setiap Controller.

### 3. Pembuatan `dashboard_layout.html` Baru
Membuat layout backend khusus berbasis Tailwind CSS pada `resources/views/dashboard_layout.html` yang mendukung HTMX Out-Of-Bounds (OOB) swap dan memiliki sidebar yang modern dan responsif.

### 4. Perubahan Stub CRUD (CLI)
Memperbarui stub template CLI (`view_index.stub`, `view_create.stub`, `view_edit.stub`) di `src/cli/stubs/` agar secara default langsung mengekstensi `dashboard_layout.html` dan memiliki tampilan premium berbasis Tailwind CSS.

---

## Rencana Pengembangan Selanjutnya (Next Steps)
Untuk menyempurnakan fitur Upload Image:
- [ ] Mengubah input field `name` atau menambahkan input `file` di `UploadImageForm` untuk menangani `multipart/form-data`.
- [ ] Menambahkan logika pemrosesan upload file pada `UploadImageController` (menyimpan file gambar ke disk/local storage).
- [ ] Menampilkan pratinjau (preview) gambar yang di-upload pada tabel `index.html`.
