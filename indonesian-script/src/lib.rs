use regex::Regex;
use std::collections::HashSet;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Indonesian Script Engine (`.is`)
/// Super lightweight reactive JavaScript compiler in Rust with Indonesian & Javanese syntax helpers.
pub struct IsEngine;

impl IsEngine {
    pub fn compile(source: &str) -> String {
        compile_is(source)
    }
}

/// Standalone function for compiling Indonesian Script (.is) source code to Vanilla JavaScript.
#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = compile))]
pub fn compile_is(source: &str) -> String {
    let mut js = String::new();
    let mut reactive_vars = HashSet::new();

    // Header penanda compiler
    js.push_str("/* === Indonesian Script (.is) Compiled Output === */\n");
    js.push_str("(function() {\n");

    // Runtime bootstrap untuk reaktivitas & DOM binding dinamis
    js.push_str(r#"
  // Core Runtime Indonesian Script (.is)
  if (!window.__IS_RUNTIME__) {
    window.__IS_RUNTIME__ = true;
    window.__IS_STATE__ = window.__IS_STATE__ || {};
    window.__IS_LISTENERS__ = window.__IS_LISTENERS__ || {};

    window.__IS_PROXY__ = new Proxy(window.__IS_STATE__, {
      set(target, prop, value) {
        target[prop] = value;
        // Auto update DOM elements bound to this state
        document.querySelectorAll(`[is-bind="${prop}"], [is-text="${prop}"], [is-model="${prop}"]`).forEach(el => {
          if (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA' || el.tagName === 'SELECT') {
            if (el.value != value) el.value = value;
          } else {
            el.textContent = value;
          }
        });
        document.querySelectorAll(`[is-show="${prop}"]`).forEach(el => {
          el.style.display = value ? '' : 'none';
        });
        if (window.__IS_LISTENERS__[prop]) {
          window.__IS_LISTENERS__[prop].forEach(fn => fn(value));
        }
        return true;
      },
      get(target, prop) {
        return target[prop];
      }
    });

    // Auto setup DOM listeners for input bindings (is-model / is-bind)
    document.addEventListener('input', function(e) {
      const modelAttr = e.target.getAttribute('is-model') || e.target.getAttribute('is-bind');
      if (modelAttr) {
        window.__IS_PROXY__[modelAttr] = e.target.value;
      }
    });

    // Auto setup AJAX / Action attributes (is-jupuk, is-kirim, is-click)
    document.addEventListener('click', async function(e) {
      const btn = e.target.closest('[is-jupuk], [is-ambil], [is-kirim], [is-click]');
      if (!btn) return;

      const jupukUrl = btn.getAttribute('is-jupuk') || btn.getAttribute('is-ambil');
      const kirimUrl = btn.getAttribute('is-kirim');
      const targetSel = btn.getAttribute('is-target');

      if (jupukUrl) {
        e.preventDefault();
        try {
          const res = await fetch(jupukUrl);
          const text = await res.text();
          if (targetSel) {
            const targetEl = document.querySelector(targetSel);
            if (targetEl) targetEl.innerHTML = text;
          }
        } catch (err) {
          console.error('[IndonesianScript] Error fetching:', err);
        }
      } else if (kirimUrl) {
        e.preventDefault();
        const form = btn.closest('form');
        const data = form ? Object.fromEntries(new FormData(form)) : {};
        try {
          const res = await fetch(kirimUrl, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data)
          });
          const text = await res.text();
          if (targetSel) {
            const targetEl = document.querySelector(targetSel);
            if (targetEl) targetEl.innerHTML = text;
          }
        } catch (err) {
          console.error('[IndonesianScript] Error posting:', err);
        }
      }
    });
  }
  const $state = window.__IS_PROXY__;
"#);

    let re_reaktif = Regex::new(r#"(?m)^\s*(reaktif|state)\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=\s*(.*?);"#).unwrap();
    let mut processed_source = source.to_string();
    let mut decl_replacements = Vec::new();

    // 1. Ekstrak dan kompilasi reaktif / state variables (gaya Svelte)
    for caps in re_reaktif.captures_iter(source) {
        let var_name = caps[2].to_string();
        let initial_val = caps[3].to_string();
        reactive_vars.insert(var_name.clone());

        let placeholder = format!("__IS_STATE_DECL_{}__;", var_name);
        let final_decl = format!(
            "/* [Indonesian Script] State Reaktif: {} */\n  $state.{} = {};\n  let {} = $state.{};",
            var_name, var_name, initial_val, var_name, var_name
        );
        decl_replacements.push((placeholder.clone(), final_decl));

        processed_source = processed_source.replace(&caps[0], &placeholder);
    }

    // 2. Transpilasi Kontrol Alur & Kata Kunci Bahasa Indonesia / Daerah
    // -------------------------------------------------------------
    // selain_itu jika / utawa yen -> else if
    let re_else_if = Regex::new(r"\b(selain_itu\s+jika|utawa\s+yen)\b").unwrap();
    processed_source = re_else_if.replace_all(&processed_source, "else if").to_string();

    // selain_itu / liyane -> else
    let re_else = Regex::new(r"\b(selain_itu|liyane)\b").unwrap();
    processed_source = re_else.replace_all(&processed_source, "else").to_string();

    // jika / kalau / yen -> if
    let re_if = Regex::new(r"\b(jika|kalau|yen)\b").unwrap();
    processed_source = re_if.replace_all(&processed_source, "if").to_string();

    // fungsi / pranata / tugas -> function
    let re_fungsi = Regex::new(r"\b(fungsi|pranata|tugas)\b").unwrap();
    processed_source = re_fungsi.replace_all(&processed_source, "function").to_string();

    // kembalikan / balekno -> return
    let re_kembalikan = Regex::new(r"\b(kembalikan|balekno)\b").unwrap();
    processed_source = re_kembalikan.replace_all(&processed_source, "return").to_string();

    // selama / suwene -> while
    let re_selama = Regex::new(r"\b(selama|suwene)\b").unwrap();
    processed_source = re_selama.replace_all(&processed_source, "while").to_string();

    // untuk / kanggo -> for
    let re_untuk = Regex::new(r"\b(untuk|kanggo)\b").unwrap();
    processed_source = re_untuk.replace_all(&processed_source, "for").to_string();

    // berhenti / mandek -> break
    let re_berhenti = Regex::new(r"\b(berhenti|mandek)\b").unwrap();
    processed_source = re_berhenti.replace_all(&processed_source, "break").to_string();

    // lanjut / terusno -> continue
    let re_lanjut = Regex::new(r"\b(lanjut|terusno)\b").unwrap();
    processed_source = re_lanjut.replace_all(&processed_source, "continue").to_string();

    // benar / bener -> true
    let re_benar = Regex::new(r"\b(benar|bener)\b").unwrap();
    processed_source = re_benar.replace_all(&processed_source, "true").to_string();

    // salah / kliru -> false
    let re_salah = Regex::new(r"\b(salah|kliru)\b").unwrap();
    processed_source = re_salah.replace_all(&processed_source, "false").to_string();

    // kosong / suwung -> null
    let re_kosong = Regex::new(r"\b(kosong|suwung)\b").unwrap();
    processed_source = re_kosong.replace_all(&processed_source, "null").to_string();

    // 3. Transpilasi Sintaks & Helper Bawaan
    // -------------------------------------------------------------

    // dudohno / cetak / tulis -> console.log
    let re_cetak = Regex::new(r"\b(dudohno|cetak|tulis)\s*\(").unwrap();
    processed_source = re_cetak.replace_all(&processed_source, "console.log(/* [Indonesian Script: cetak/dudohno] */ ").to_string();

    // jupuk / ambil -> fetch json
    let re_jupuk = Regex::new(r"\b(jupuk|ambil)\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_jupuk.replace_all(&processed_source, "(await fetch($2).then(r => r.json()))").to_string();

    // kirim / kirimkan -> fetch post json ($2 is URL, $3 is payload)
    let re_kirim = Regex::new(r"\b(kirim|kirimkan)\s*\(\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_kirim.replace_all(&processed_source, "(await fetch($2, { method: 'POST', headers: {'Content-Type': 'application/json'}, body: JSON.stringify($3) }).then(r => r.json()))").to_string();

    // Reaksi otomatis pada re-assignment variabel reaktif (misal: skor = skor + 10 => $state.skor = skor + 10)
    for var_name in &reactive_vars {
        let re_assign = Regex::new(&format!(r"\b{}\s*=\s*([^;]+);", regex::escape(var_name))).unwrap();
        let replacement = format!("$state.{} = $1;", var_name);
        processed_source = re_assign.replace_all(&processed_source, &replacement).to_string();
    }

    // Kembalikan deklarasi $state.var_name dari placeholder
    for (placeholder, final_decl) in decl_replacements {
        processed_source = processed_source.replace(&placeholder, &final_decl);
    }

    // siji / pilih -> document.querySelector
    let re_pilih = Regex::new(r"\b(siji|pilih)\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_pilih.replace_all(&processed_source, "document.querySelector($2)").to_string();

    // kabeh / pilihSemua -> document.querySelectorAll
    let re_pilih_semua = Regex::new(r"\b(kabeh|pilihSemua)\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_pilih_semua.replace_all(&processed_source, "document.querySelectorAll($2)").to_string();

    // saat / ketika -> addEventListener
    let re_saat = Regex::new(r"\b(saat|ketika)\s*\(\s*([^,]+)\s*,\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_saat.replace_all(&processed_source, "(typeof $2 === 'string' ? document.querySelector($2) : $2)?.addEventListener($3, $4)").to_string();

    // gantiHtml / tumpuk -> innerHTML
    let re_ganti_html = Regex::new(r"\b(gantiHtml|tumpuk)\s*\(\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_ganti_html.replace_all(&processed_source, "(typeof $2 === 'string' ? document.querySelector($2) : $2).innerHTML = $3").to_string();

    // sembunyikan / umpetno -> style.display = 'none'
    let re_umpet = Regex::new(r"\b(sembunyikan|umpetno)\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_umpet.replace_all(&processed_source, "(typeof $2 === 'string' ? document.querySelector($2) : $2).style.display = 'none'").to_string();

    // tampilkan / tuduhno -> style.display = ''
    let re_tampil = Regex::new(r"\b(tampilkan|tuduhno)\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_tampil.replace_all(&processed_source, "(typeof $2 === 'string' ? document.querySelector($2) : $2).style.display = ''").to_string();

    // Dialog & Notifikasi: peringatan, konfirmasi, masukan
    let re_peringatan = Regex::new(r"\b(peringatan|elingno)\s*\(").unwrap();
    processed_source = re_peringatan.replace_all(&processed_source, "alert(").to_string();

    let re_konfirmasi = Regex::new(r"\b(konfirmasi|takon)\s*\(").unwrap();
    processed_source = re_konfirmasi.replace_all(&processed_source, "confirm(").to_string();

    let re_masukan = Regex::new(r"\b(masukan|jaluk)\s*\(").unwrap();
    processed_source = re_masukan.replace_all(&processed_source, "prompt(").to_string();

    // Asinkron & Waktu: tunggu, setiap, nanti
    let re_tunggu = Regex::new(r"\btunggu\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_tunggu.replace_all(&processed_source, "(await new Promise(r => setTimeout(r, $1)))").to_string();

    let re_setiap = Regex::new(r"\b(setiap|saben)\s*\(\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_setiap.replace_all(&processed_source, "setInterval($3, $2)").to_string();

    let re_nanti = Regex::new(r"\b(nanti|mengko)\s*\(\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_nanti.replace_all(&processed_source, "setTimeout($3, $2)").to_string();

    // Clipboard & Storage: salin, simpan, ambilPenyimpanan, hapusPenyimpanan
    let re_salin = Regex::new(r"\b(salin|salinTeks)\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_salin.replace_all(&processed_source, "navigator.clipboard?.writeText($2)").to_string();

    let re_simpan = Regex::new(r"\b(simpan|simpen)\s*\(\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_simpan.replace_all(&processed_source, "localStorage.setItem($2, JSON.stringify($3))").to_string();

    let re_ambil_penyimpanan = Regex::new(r"\b(ambilPenyimpanan|jupukSimpenan)\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_ambil_penyimpanan.replace_all(&processed_source, "JSON.parse(localStorage.getItem($2) || 'null')").to_string();

    let re_hapus_penyimpanan = Regex::new(r"\b(hapusPenyimpanan|busakSimpenan)\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_hapus_penyimpanan.replace_all(&processed_source, "localStorage.removeItem($2)").to_string();

    // Format Rupiah, Angka & Tanggal
    let re_rupiah = Regex::new(r"\b(formatRupiah|rupiah)\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_rupiah.replace_all(&processed_source, "(new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', maximumFractionDigits: 0 }).format($2))").to_string();

    let re_format_angka = Regex::new(r"\bformatAngka\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_format_angka.replace_all(&processed_source, "(new Intl.NumberFormat('id-ID').format($2))").to_string();

    let re_format_tanggal = Regex::new(r"\bformatTanggal\s*\(\s*([^)]+)\s*\)").unwrap();
    processed_source = re_format_tanggal.replace_all(&processed_source, "(new Intl.DateTimeFormat('id-ID', { dateStyle: 'full' }).format(new Date($2)))").to_string();

    // Manipulasi Class DOM
    let re_tambah_kelas = Regex::new(r"\btambahKelas\s*\(\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_tambah_kelas.replace_all(&processed_source, "((typeof $1 === 'string' ? document.querySelector($1) : $1)?.classList?.add($2))").to_string();

    let re_hapus_kelas = Regex::new(r"\bhapusKelas\s*\(\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_hapus_kelas.replace_all(&processed_source, "((typeof $1 === 'string' ? document.querySelector($1) : $1)?.classList?.remove($2))").to_string();

    let re_ganti_kelas = Regex::new(r"\b(gantiKelas|putarKelas)\s*\(\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_ganti_kelas.replace_all(&processed_source, "((typeof $1 === 'string' ? document.querySelector($1) : $1)?.classList?.toggle($2))").to_string();

    let re_ada_kelas = Regex::new(r"\badaKelas\s*\(\s*([^,]+)\s*,\s*([^)]+)\s*\)").unwrap();
    processed_source = re_ada_kelas.replace_all(&processed_source, "((typeof $1 === 'string' ? document.querySelector($1) : $1)?.classList?.contains($2))").to_string();

    js.push_str("  ");
    js.push_str(&processed_source);
    js.push_str("\n})();\n");

    js
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_engine_compiler_basic() {
        let code = r#"
reaktif jumlah = 0;
fungsi tambah() {
    jumlah = jumlah + 1;
    dudohno('Jumlah saiki:', jumlah);
}
"#;
        let compiled = compile_is(code);
        assert!(compiled.contains("State Reaktif: jumlah"));
        assert!(compiled.contains("$state.jumlah = 0;"));
        assert!(compiled.contains("console.log"));
        assert!(compiled.contains("function tambah()"));
    }

    #[test]
    fn test_is_engine_control_flow_and_helpers() {
        let code = r#"
fungsi cek_status(skor) {
    jika skor >= 80 {
        kembalikan benar;
    } selain_itu jika skor >= 60 {
        cetak("Cukup");
        kembalikan benar;
    } selain_itu {
        kembalikan salah;
    }
}

fungsi bayar(nominal) {
    let harga_rp = rupiah(nominal);
    salin(harga_rp);
    peringatan("Disalin: " + harga_rp);
}
"#;
        let compiled = compile_is(code);
        assert!(compiled.contains("function cek_status(skor)"));
        assert!(compiled.contains("if skor >= 80"));
        assert!(compiled.contains("return true;"));
        assert!(compiled.contains("else if skor >= 60"));
        assert!(compiled.contains("else"));
        assert!(compiled.contains("return false;"));
        assert!(compiled.contains("Intl.NumberFormat('id-ID'"));
        assert!(compiled.contains("navigator.clipboard?.writeText"));
        assert!(compiled.contains("alert("));
    }

    #[test]
    fn test_is_engine_dom_and_storage_helpers() {
        let code = r##"
fungsi init_ui() {
    tambahKelas("#card", "aktif");
    simpan("pengaturan", { tema: "gelap" });
    let p = ambilPenyimpanan("pengaturan");
}
"##;
        let compiled = compile_is(code);
        assert!(compiled.contains("classList?.add"));
        assert!(compiled.contains("localStorage.setItem"));
        assert!(compiled.contains("localStorage.getItem"));
    }
}


