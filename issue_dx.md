# Improvement: DX (dd, dump) & Controller Beautification

## Deskripsi
Developer Experience (DX) adalah kunci framework yang baik. Isu ini bertujuan untuk mengimplementasikan fitur debugging yang human-readable mirip `dd()` dan `dump()` di Laravel, baik di kode Rust maupun di layer View (Tera). Selain itu, merapikan `AuthController` agar lebih mudah dibaca dan menerapkan *Clean Code*.

## Kebutuhan Teknis
1. **Macro Debugging:** Buat `dd!()` dan `dump!()` di `src/support/debug.rs`.
2. **Panic Handler:** Cegat `panic!("__LUMINA_DD__...")` di `CatchPanicLayer` dan render HTML yang cantik (Dark Mode).
3. **View Debugging:** Tambahkan fungsi `dump` ke mesin Tera (`{{ dump(var=data) }}`).
4. **Beautification:** Refactor `AuthController.rs` menjadi logika yang *linear* dan hapus *boilerplate* duplikatif.
