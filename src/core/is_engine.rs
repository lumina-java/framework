use regex::Regex;
use std::collections::HashSet;

/// Indonesian Script Engine (`.is`)
/// Engine compiler super ringan, cepat, dan reaktif gaya Svelte
/// yang mendukung sintaks Bahasa Indonesia, Bahasa Jawa, serta Vanilla JS murni.
pub struct IsEngine;

impl IsEngine {
    /// Kompilasi kode sumber `.is` menjadi Vanilla JavaScript murni yang siap dieksekusi di browser.
    pub fn compile(source: &str) -> String {
        let mut js = String::new();
        let mut _reactive_vars = HashSet::new();

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
            _reactive_vars.insert(var_name.clone());

            let placeholder = format!("__IS_STATE_DECL_{}__;", var_name);
            let final_decl = format!(
                "/* [Indonesian Script] State Reaktif: {} */\n  $state.{} = {};\n  let {} = $state.{};",
                var_name, var_name, initial_val, var_name, var_name
            );
            decl_replacements.push((placeholder.clone(), final_decl));

            processed_source = processed_source.replace(&caps[0], &placeholder);
        }

        // 2. Transpilasi Sintaks & Helper Bahasa Indonesia / Jawa

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
        for var_name in &_reactive_vars {
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

        js.push_str("  ");
        js.push_str(&processed_source);
        js.push_str("\n})();\n");

        js
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_engine_compiler_basic() {
        let code = r#"
reaktif jumlah = 0;
function tambah() {
    jumlah = jumlah + 1;
    dudohno('Jumlah saiki:', jumlah);
}
"#;
        let compiled = IsEngine::compile(code);
        assert!(compiled.contains("State Reaktif: jumlah"));
        assert!(compiled.contains("$state.jumlah = 0;"));
        assert!(compiled.contains("console.log"));
        assert!(compiled.contains("function tambah()"));
    }

    #[test]
    fn test_is_engine_helpers_indonesia_jawa() {
        let code = r#"
let el = siji('#app');
cetak('Elemen:', el);
umpetno('#loader');
tuduhno('#content');
"#;
        let compiled = IsEngine::compile(code);
        assert!(compiled.contains("document.querySelector('#app')"));
        assert!(compiled.contains("console.log"));
        assert!(compiled.contains("style.display = 'none'"));
        assert!(compiled.contains("style.display = ''"));
    }
}
