use bcrypt::{hash, verify, DEFAULT_COST};

/// Hash password menggunakan bcrypt dengan default cost (12).
pub fn make(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Gagal melakukan hashing password")
}

/// Verifikasi apakah password cocok dengan hash.
pub fn check(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap_or(false)
}
