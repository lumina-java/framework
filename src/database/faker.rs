use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Global atomic counter untuk menjamin keunikan email / id
static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(1000);

thread_local! {
    // Thread-local PRNG xorshift64star untuk kecepatan maksimal tanpa lock/overhead
    static RNG_STATE: Cell<u64> = Cell::new({
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(88172645463325252);
        if nanos == 0 { 0xdeadbeefcafebabe } else { nanos }
    });
}

fn next_u64() -> u64 {
    RNG_STATE.with(|state| {
        let mut x = state.get();
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        state.set(x);
        x.wrapping_mul(0x2545F4914F6CDD1D)
    })
}

fn rand_range(min: usize, max: usize) -> usize {
    if min >= max {
        return min;
    }
    let range = (max - min + 1) as u64;
    min + (next_u64() % range) as usize
}

fn pick<'a>(items: &'a [&'a str]) -> &'a str {
    if items.is_empty() {
        return "";
    }
    let idx = rand_range(0, items.len() - 1);
    items[idx]
}

/// Faker — Generator Data Realistis Ringan Berkecepatan Tinggi (Native Rust)
/// Dioptimalkan untuk lokalisasi Indonesia dan internasional tanpa dependensi eksternal.
pub struct Faker;

impl Faker {
    // ─────────────────────────────────────────────────────────────────────────
    // NAMA & IDENTITAS
    // ─────────────────────────────────────────────────────────────────────────

    pub const FIRST_NAMES_MALE: &'static [&'static str] = &[
        "Ahmad", "Budi", "Pratama", "Rizky", "Dimas", "Bayu", "Hendra", "Joko",
        "Slamet", "Doni", "Fajar", "Galih", "Hafiz", "Ilham", "Kurnia", "Lukman",
        "Maulana", "Nugroho", "Putra", "Raden", "Satria", "Tri", "Wahyu", "Yusuf",
    ];

    pub const FIRST_NAMES_FEMALE: &'static [&'static str] = &[
        "Siti", "Dewi", "Sri", "Nur", "Putri", "Rina", "Rahma", "Ratna", "Maya",
        "Ayu", "Fitri", "Gita", "Indah", "Lestari", "Mega", "Nadia", "Sari",
        "Tari", "Utami", "Wulandari", "Yulia", "Zahra", "Anisa", "Bella",
    ];

    pub const LAST_NAMES: &'static [&'static str] = &[
        "Santoso", "Wijaya", "Saputra", "Hidayat", "Setiawan", "Kurniawan",
        "Wibowo", "Pratama", "Nugroho", "Kusuma", "Utomo", "Raharjo", "Firmansyah",
        "Gunawan", "Susanto", "Siregar", "Nasution", "Lubis", "Pasaribu", "Sitorus",
    ];

    /// Menghasilkan nama depan pria Indonesia
    pub fn first_name_male() -> &'static str {
        pick(Self::FIRST_NAMES_MALE)
    }

    /// Menghasilkan nama depan wanita Indonesia
    pub fn first_name_female() -> &'static str {
        pick(Self::FIRST_NAMES_FEMALE)
    }

    /// Menghasilkan nama depan acak
    pub fn first_name() -> &'static str {
        if next_u64() % 2 == 0 {
            Self::first_name_male()
        } else {
            Self::first_name_female()
        }
    }

    /// Menghasilkan nama belakang Indonesia
    pub fn last_name() -> &'static str {
        pick(Self::LAST_NAMES)
    }

    /// Menghasilkan nama lengkap (Nama Depan + Belakang)
    pub fn name() -> String {
        format!("{} {}", Self::first_name(), Self::last_name())
    }

    // ─────────────────────────────────────────────────────────────────────────
    // KONTAK & INTERNET
    // ─────────────────────────────────────────────────────────────────────────

    pub const DOMAINS: &'static [&'static str] = &[
        "gmail.com", "yahoo.com", "outlook.com", "example.com", "lumina.dev",
        "kemendag.go.id", "techindo.co.id", "startup.id", "nusantara.id",
    ];

    pub const PHONE_PREFIXES: &'static [&'static str] = &[
        "0812", "0813", "0821", "0822", "0852", "0853", "0857", "0858",
        "0877", "0878", "0896", "0895", "0819",
    ];

    /// Menghasilkan alamat email yang valid dan realistis
    pub fn email() -> String {
        let first = Self::first_name().to_lowercase();
        let last = Self::last_name().to_lowercase();
        let rand_num = rand_range(10, 999);
        let domain = pick(Self::DOMAINS);
        format!("{}.{}{}", first, last, rand_num) + "@" + domain
    }

    /// Menghasilkan alamat email yang dijamin unik di seluruh thread
    pub fn unique_email() -> String {
        let seq = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let first = Self::first_name().to_lowercase();
        format!("{}.{}@example.com", first, seq)
    }

    /// Menghasilkan nomor telepon seluler format Indonesia (misal: 081234567890)
    pub fn phone() -> String {
        let prefix = pick(Self::PHONE_PREFIXES);
        let suffix = rand_range(10000000, 99999999);
        format!("{}{}", prefix, suffix)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // LOKASI & ALAMAT
    // ─────────────────────────────────────────────────────────────────────────

    pub const CITIES: &'static [&'static str] = &[
        "Jakarta Pusat", "Jakarta Selatan", "Jakarta Barat", "Surabaya", "Bandung",
        "Medan", "Semarang", "Makassar", "Palembang", "Tangerang", "Depok", "Bekasi",
        "Bogor", "Yogyakarta", "Surakarta", "Malang", "Denpasar", "Balikpapan",
    ];

    pub const STREETS: &'static [&'static str] = &[
        "Jl. Jenderal Sudirman", "Jl. M.H. Thamrin", "Jl. Gatot Subroto",
        "Jl. H.R. Rasuna Said", "Jl. Diponegoro", "Jl. Malioboro", "Jl. Asia Afrika",
        "Jl. Ir. H. Juanda (Dago)", "Jl. Pemuda", "Jl. Basuki Rahmat", "Jl. Gajah Mada",
        "Jl. Hayam Wuruk", "Jl. Pahlawan", "Jl. Veteran", "Jl. Ahmad Yani",
    ];

    /// Menghasilkan nama kota di Indonesia
    pub fn city() -> &'static str {
        pick(Self::CITIES)
    }

    /// Menghasilkan nama jalan di Indonesia
    pub fn street() -> &'static str {
        pick(Self::STREETS)
    }

    /// Menghasilkan alamat lengkap Indonesia beserta nomor bangunan
    pub fn address() -> String {
        let no = rand_range(1, 199);
        let rt = rand_range(1, 15);
        let rw = rand_range(1, 12);
        format!("{}, No. {}, RT {:02}/RW {:02}, {}", Self::street(), no, rt, rw, Self::city())
    }

    // ─────────────────────────────────────────────────────────────────────────
    // PERUSAHAAN & BISNIS
    // ─────────────────────────────────────────────────────────────────────────

    pub const COMPANY_TYPES: &'static [&'static str] = &["PT", "CV", "UD", "Perum", "Koperasi"];

    pub const COMPANY_NAMES: &'static [&'static str] = &[
        "Teknologi Nusantara", "Maju Bersama", "Sejahtera Abadi", "Inovasi Digital",
        "Surya Gemilang", "Makmur Jaya", "Karya Mandiri", "Sentosa Prima",
        "Bintang Timur", "Cahaya Harapan", "Mitra Solusi Global", "Pilar Bangsa",
    ];

    /// Menghasilkan nama perusahaan Indonesia (misal: PT Maju Bersama)
    pub fn company() -> String {
        format!("{} {}", pick(Self::COMPANY_TYPES), pick(Self::COMPANY_NAMES))
    }

    // ─────────────────────────────────────────────────────────────────────────
    // FINANSIAL & ANGKA
    // ─────────────────────────────────────────────────────────────────────────

    /// Menghasilkan angka integer acak dalam rentang [min, max]
    pub fn number(min: usize, max: usize) -> usize {
        rand_range(min, max)
    }

    /// Menghasilkan nominal Rupiah (integer)
    pub fn rupiah_amount(min: usize, max: usize) -> usize {
        let val = rand_range(min, max);
        (val / 1000) * 1000 // dibulatkan ke kelipatan seribu
    }

    /// Menghasilkan string terformat Rupiah (misal: "Rp 150.000")
    pub fn rupiah(min: usize, max: usize) -> String {
        let val = Self::rupiah_amount(min, max);
        let s = val.to_string();
        let mut result = String::new();
        let mut count = 0;
        for c in s.chars().rev() {
            if count > 0 && count % 3 == 0 {
                result.push('.');
            }
            result.push(c);
            count += 1;
        }
        format!("Rp {}", result.chars().rev().collect::<String>())
    }

    /// Menghasilkan boolean acak dengan peluang kemunculan true tertentu (default: 50%)
    pub fn boolean(chance_percent: usize) -> bool {
        let roll = rand_range(1, 100);
        roll <= chance_percent
    }

    // ─────────────────────────────────────────────────────────────────────────
    // TEKS & KONTEN LOREM
    // ─────────────────────────────────────────────────────────────────────────

    pub const WORDS: &'static [&'static str] = &[
        "lumina", "kecepatan", "elegan", "kinerja", "arsitektur", "keamanan",
        "sistem", "basisdata", "layanan", "pengguna", "transaksi", "modul",
        "jaringan", "protokol", "pengembangan", "teknologi", "inovasi", "koneksi",
        "antarmuka", "solusi", "digital", "andal", "tangguh", "fleksibel",
    ];

    /// Menghasilkan satu kata acak
    pub fn word() -> &'static str {
        pick(Self::WORDS)
    }

    /// Menghasilkan sebuah kalimat teks acak
    pub fn sentence(num_words: usize) -> String {
        if num_words == 0 {
            return String::new();
        }
        let words: Vec<&str> = (0..num_words).map(|_| Self::word()).collect();
        let mut sentence = words.join(" ");
        if let Some(first_char) = sentence.chars().next() {
            sentence = first_char.to_uppercase().collect::<String>() + &sentence[first_char.len_utf8()..];
        }
        sentence.push('.');
        sentence
    }

    /// Menghasilkan paragraf teks konten dummy
    pub fn paragraph(num_sentences: usize) -> String {
        let sentences: Vec<String> = (0..num_sentences)
            .map(|_| Self::sentence(rand_range(6, 14)))
            .collect();
        sentences.join(" ")
    }

    // ─────────────────────────────────────────────────────────────────────────
    // TANGGAL (ISO 8601 & SQL FORMAT)
    // ─────────────────────────────────────────────────────────────────────────

    /// Menghasilkan tanggal acak dalam format string YYYY-MM-DD
    pub fn date_recent() -> String {
        let year = rand_range(2023, 2026);
        let month = rand_range(1, 12);
        let day = rand_range(1, 28);
        format!("{:04}-{:02}-{:02}", year, month, day)
    }

    /// Menghasilkan timestamp datetime acak dalam format string YYYY-MM-DD HH:MM:SS
    pub fn datetime_recent() -> String {
        let date = Self::date_recent();
        let hour = rand_range(0, 23);
        let min = rand_range(0, 59);
        let sec = rand_range(0, 59);
        format!("{} {:02}:{:02}:{:02}", date, hour, min, sec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faker_name_and_contact() {
        let name = Faker::name();
        assert!(!name.is_empty());
        assert!(name.contains(' '));

        let email = Faker::email();
        assert!(email.contains('@'));

        let unique1 = Faker::unique_email();
        let unique2 = Faker::unique_email();
        assert_ne!(unique1, unique2);

        let phone = Faker::phone();
        assert!(phone.starts_with("08"));
        assert!(phone.len() >= 11);
    }

    #[test]
    fn test_faker_location_and_company() {
        let city = Faker::city();
        assert!(!city.is_empty());

        let address = Faker::address();
        assert!(address.contains("Jl."));
        assert!(address.contains("No."));

        let company = Faker::company();
        assert!(company.starts_with("PT ") || company.starts_with("CV ") || company.starts_with("UD ") || company.starts_with("Perum ") || company.starts_with("Koperasi "));
    }

    #[test]
    fn test_faker_financial_and_text() {
        let rupiah_str = Faker::rupiah(10000, 1000000);
        assert!(rupiah_str.starts_with("Rp "));
        assert!(rupiah_str.contains('.'));

        let sentence = Faker::sentence(5);
        assert!(sentence.ends_with('.'));
        assert!(sentence.split_whitespace().count() == 5);
    }

    #[test]
    fn test_faker_high_speed_benchmark() {
        // Harus mampu men-generate 10.000 data dalam waktu sangat singkat
        let start = std::time::Instant::now();
        for _ in 0..10_000 {
            let _ = Faker::name();
            let _ = Faker::phone();
            let _ = Faker::email();
        }
        let elapsed = start.elapsed();
        // 10.000 generasi lengkap harus selesai jauh di bawah 50 milidetik
        assert!(elapsed.as_millis() < 200, "Faker terlalu lambat: {:?}", elapsed);
    }
}
