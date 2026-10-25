use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;
use crate::core::sandbox::SandboxError;

/// Representasi fungsi Indonesian Script yang siap dieksekusi di backend.
#[derive(Clone, Debug)]
pub struct IsFunction {
    pub name: String,
    pub params: Vec<String>,
    pub body: String,
}

/// Evaluator runtime (interpreter) aman untuk menjalankan logika Indonesian Script (.is) di server.
#[derive(Clone, Debug, Default)]
pub struct IsScriptRunner {
    pub functions: HashMap<String, IsFunction>,
}

impl IsScriptRunner {
    /// Buat runner baru dari kode sumber Indonesian Script
    pub fn from_source(source: &str) -> Result<Self, SandboxError> {
        let mut runner = Self::default();
        runner.parse_source(source)?;
        Ok(runner)
    }

    /// Buat runner dari file .is di disk
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, SandboxError> {
        let content = std::fs::read_to_string(path)?;
        Self::from_source(&content)
    }

    /// Ekstrak deklarasi fungsi: fungsi nama(arg1, arg2) { ... }
    fn parse_source(&mut self, source: &str) -> Result<(), SandboxError> {
        let re_func_header = Regex::new(r#"(?:fungsi|pranata|tugas)\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\(([^)]*)\)\s*\{"#).unwrap();

        for caps in re_func_header.captures_iter(source) {
            let full_match = caps.get(0).unwrap();
            let func_name = caps[1].to_string();
            let raw_params = &caps[2];

            let params: Vec<String> = raw_params
                .split(',')
                .map(|p| p.trim().trim_start_matches('$').to_string())
                .filter(|p| !p.is_empty())
                .collect();

            // Ekstrak blok body dengan menghitung kurung kurawal berpasangan { ... }
            let open_brace_pos = full_match.end() - 1;
            if let Some(body) = Self::extract_balanced_braces(source, open_brace_pos) {
                self.functions.insert(
                    func_name.clone(),
                    IsFunction {
                        name: func_name,
                        params,
                        body: body.trim().to_string(),
                    },
                );
            }
        }

        Ok(())
    }

    fn extract_balanced_braces(s: &str, open_pos: usize) -> Option<&str> {
        let bytes = s.as_bytes();
        if open_pos >= bytes.len() || bytes[open_pos] != b'{' {
            return None;
        }

        let mut depth = 0;
        let start = open_pos + 1;

        for (i, &b) in bytes.iter().enumerate().skip(open_pos) {
            if b == b'{' {
                depth += 1;
            } else if b == b'}' {
                depth -= 1;
                if depth == 0 {
                    return Some(&s[start..i]);
                }
            }
        }

        None
    }

    /// Panggil fungsi Indonesian Script dengan parameter JSON
    pub fn call(&self, func_name: &str, args: &[Value]) -> Result<Value, SandboxError> {
        let func = self
            .functions
            .get(func_name)
            .ok_or_else(|| SandboxError::FunctionNotFound(format!("Fungsi '{}' tidak ditemukan di skrip", func_name)))?;

        let mut scope: HashMap<String, Value> = HashMap::new();

        // Bind parameter ke argumen
        for (i, param_name) in func.params.iter().enumerate() {
            let val = args.get(i).cloned().unwrap_or(Value::Null);
            scope.insert(param_name.clone(), val);
        }

        // Eksekusi body fungsi
        let res = self.eval_block(&func.body, &mut scope)?;
        Ok(res.unwrap_or(Value::Null))
    }

    /// Evaluasi sebuah blok kode yang berisi deretan statement
    pub fn eval_block(&self, code: &str, scope: &mut HashMap<String, Value>) -> Result<Option<Value>, SandboxError> {
        let mut idx = 0;
        let len = code.len();

        while idx < len {
            // Lewati spasi dan newline
            while idx < len && code.as_bytes()[idx].is_ascii_whitespace() {
                idx += 1;
            }
            if idx >= len {
                break;
            }

            let remaining = &code[idx..];

            // 1. Percabangan: jika / kalau
            if remaining.starts_with("jika ") || remaining.starts_with("jika(") || remaining.starts_with("kalau ") || remaining.starts_with("kalau(") {
                let (res, consumed) = self.eval_if_statement(remaining, scope)?;
                idx += consumed;
                if let Some(ret) = res {
                    return Ok(Some(ret));
                }
                continue;
            }

            // 2. Perulangan: selama / suwene
            if remaining.starts_with("selama ") || remaining.starts_with("selama(") || remaining.starts_with("suwene ") || remaining.starts_with("suwene(") {
                let (res, consumed) = self.eval_while_statement(remaining, scope)?;
                idx += consumed;
                if let Some(ret) = res {
                    return Ok(Some(ret));
                }
                continue;
            }

            // Cari akhir statement (titik koma ';')
            let semi_rel = remaining.find(';');
            let stmt_str = if let Some(s_pos) = semi_rel {
                let s = &remaining[..s_pos];
                idx += s_pos + 1;
                s.trim()
            } else {
                idx = len;
                remaining.trim()
            };

            if stmt_str.is_empty() {
                continue;
            }

            // 3. Return statement: kembalikan / balekno / return
            if stmt_str.starts_with("kembalikan ") || stmt_str.starts_with("balekno ") || stmt_str.starts_with("return ") {
                let expr_str = if let Some(stripped) = stmt_str.strip_prefix("kembalikan ") {
                    stripped
                } else if let Some(stripped) = stmt_str.strip_prefix("balekno ") {
                    stripped
                } else {
                    stmt_str.strip_prefix("return ").unwrap()
                };
                let val = self.eval_expr(expr_str.trim(), scope)?;
                return Ok(Some(val));
            }

            // 4. Assignment: [let] var = expr
            let clean_stmt = stmt_str.trim_start_matches("let ").trim_start_matches("reaktif ").trim_start_matches("state ");
            if let Some(eq_idx) = clean_stmt.find('=') {
                if !clean_stmt[..eq_idx].contains("==") && !clean_stmt[eq_idx..].starts_with("==") {
                    let var_name = clean_stmt[..eq_idx].trim().trim_start_matches('$').to_string();
                    let expr_val = clean_stmt[eq_idx + 1..].trim();
                    let val = self.eval_expr(expr_val, scope)?;
                    scope.insert(var_name, val);
                    continue;
                }
            }

            // 5. Ekspresi tunggal (misal pemanggilan fungsi tanpa assignment)
            let _ = self.eval_expr(stmt_str, scope)?;
        }

        Ok(None)
    }

    /// Evaluasi struktur kondisional: jika (cond) { ... } [selain_itu jika ...]* [selain_itu { ... }]?
    fn eval_if_statement(&self, code: &str, scope: &mut HashMap<String, Value>) -> Result<(Option<Value>, usize), SandboxError> {
        let mut idx = 0;
        let mut branch_executed = false;
        let mut early_return = None;

        while idx < code.len() {
            let slice = &code[idx..].trim_start();
            idx += code[idx..].len() - slice.len();

            let is_if = slice.starts_with("jika") || slice.starts_with("kalau");
            let is_else_if = slice.starts_with("selain_itu jika") || slice.starts_with("utawa yen");
            let is_else = slice.starts_with("selain_itu") || slice.starts_with("liyane");

            if !is_if && !is_else_if && !is_else {
                break;
            }

            if is_else && !is_else_if {
                // Cabang selain_itu { ... }
                let open_brace = match slice.find('{') {
                    Some(p) => p,
                    None => break,
                };
                let body = match Self::extract_balanced_braces(slice, open_brace) {
                    Some(b) => b,
                    None => break,
                };
                let total_consumed = open_brace + 1 + body.len() + 1;
                idx += total_consumed;

                if !branch_executed {
                    let res = self.eval_block(body, scope)?;
                    return Ok((res, idx));
                }
                break;
            }

            // Cabang jika (cond) { ... } atau selain_itu jika (cond) { ... }
            let open_brace = match slice.find('{') {
                Some(p) => p,
                None => break,
            };
            let prefix_kw = if is_else_if {
                if slice.starts_with("selain_itu jika") { "selain_itu jika" } else { "utawa yen" }
            } else if slice.starts_with("jika") {
                "jika"
            } else {
                "kalau"
            };

            let cond_str = slice[prefix_kw.len()..open_brace].trim();
            let clean_cond = cond_str.trim_start_matches('(').trim_end_matches(')').trim();

            let body = match Self::extract_balanced_braces(slice, open_brace) {
                Some(b) => b,
                None => break,
            };

            let branch_consumed = open_brace + 1 + body.len() + 1;
            idx += branch_consumed;

            if !branch_executed {
                let cond_val = self.eval_expr(clean_cond, scope)?;
                if Self::is_truthy(&cond_val) {
                    branch_executed = true;
                    let res = self.eval_block(body, scope)?;
                    if res.is_some() {
                        early_return = res;
                    }
                }
            }
        }

        Ok((early_return, idx))
    }

    /// Evaluasi struktur perulangan: selama (cond) { ... }
    fn eval_while_statement(&self, code: &str, scope: &mut HashMap<String, Value>) -> Result<(Option<Value>, usize), SandboxError> {
        let open_brace = code.find('{').ok_or_else(|| SandboxError::Trap("Sintaks 'selama' tanpa kurung kurawal '{'".to_string()))?;
        let kw_len = if code.starts_with("suwene") { 6 } else { 6 };
        let cond_str = code[kw_len..open_brace].trim().trim_start_matches('(').trim_end_matches(')').trim();

        let body = Self::extract_balanced_braces(code, open_brace)
            .ok_or_else(|| SandboxError::Trap("Blok perulangan tidak seimbang".to_string()))?;

        let consumed = open_brace + 1 + body.len() + 1;

        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 5000;

        loop {
            iterations += 1;
            if iterations > MAX_ITERATIONS {
                return Err(SandboxError::Trap("Batas perulangan maksimum (5000) terlampaui (Infinite loop detected)".to_string()));
            }

            let cond_val = self.eval_expr(cond_str, scope)?;
            if !Self::is_truthy(&cond_val) {
                break;
            }

            if let Some(ret) = self.eval_block(body, scope)? {
                return Ok((Some(ret), consumed));
            }
        }

        Ok((None, consumed))
    }

    /// Evaluasi sebuah ekspresi menjadi Value JSON
    pub fn eval_expr(&self, expr: &str, scope: &HashMap<String, Value>) -> Result<Value, SandboxError> {
        let expr = expr.trim();
        if expr.is_empty() {
            return Ok(Value::Null);
        }

        // Hapus tanda kurung terluar jika berpasangan: ( expr )
        if expr.starts_with('(') && expr.ends_with(')') {
            let inner = &expr[1..expr.len() - 1].trim();
            if !inner.contains('(') || !inner.contains(')') {
                return self.eval_expr(inner, scope);
            }
        }

        // Operator Logika: dan / &&, atau / ||
        if let Some(idx) = expr.find(" atau ").or_else(|| expr.find(" || ")) {
            let sep_len = if expr[idx..].starts_with(" atau ") { 6 } else { 4 };
            let left = self.eval_expr(&expr[..idx], scope)?;
            if Self::is_truthy(&left) {
                return Ok(json!(true));
            }
            let right = self.eval_expr(&expr[idx + sep_len..], scope)?;
            return Ok(json!(Self::is_truthy(&right)));
        }

        if let Some(idx) = expr.find(" dan ").or_else(|| expr.find(" && ")) {
            let sep_len = if expr[idx..].starts_with(" dan ") { 5 } else { 4 };
            let left = self.eval_expr(&expr[..idx], scope)?;
            if !Self::is_truthy(&left) {
                return Ok(json!(false));
            }
            let right = self.eval_expr(&expr[idx + sep_len..], scope)?;
            return Ok(json!(Self::is_truthy(&right)));
        }

        // Operator Perbandingan: ==, !=, >=, <=, >, <
        for op in &["===", "==", "!=", ">=", "<=", ">", "<"] {
            if let Some(idx) = expr.find(op) {
                let left = self.eval_expr(&expr[..idx], scope)?;
                let right = self.eval_expr(&expr[idx + op.len()..], scope)?;
                let is_match = match *op {
                    "===" | "==" => left == right,
                    "!=" => left != right,
                    ">=" => Self::compare_num(&left, &right).map(|cmp| cmp >= 0).unwrap_or(false),
                    "<=" => Self::compare_num(&left, &right).map(|cmp| cmp <= 0).unwrap_or(false),
                    ">" => Self::compare_num(&left, &right).map(|cmp| cmp > 0).unwrap_or(false),
                    "<" => Self::compare_num(&left, &right).map(|cmp| cmp < 0).unwrap_or(false),
                    _ => false,
                };
                return Ok(json!(is_match));
            }
        }

        // Operator Aritmatika Penjumlahan (+) & Pengurangan (-)
        // Hati-hati jangan sampai memotong tanda kurung atau string
        if let Some(idx) = Self::find_operator_outside_strings(expr, '+') {
            let left = self.eval_expr(&expr[..idx], scope)?;
            let right = self.eval_expr(&expr[idx + 1..], scope)?;

            // Jika salah satu sisi string, lakukan penggabungan string (concatenation)
            if left.is_string() || right.is_string() {
                let left_s = left.as_str().map(|s| s.to_string()).unwrap_or_else(|| left.to_string());
                let right_s = right.as_str().map(|s| s.to_string()).unwrap_or_else(|| right.to_string());
                return Ok(json!(format!("{}{}", left_s, right_s)));
            }

            if let (Some(n1), Some(n2)) = (left.as_f64(), right.as_f64()) {
                return Ok(json!(n1 + n2));
            }
        }

        if let Some(idx) = Self::find_operator_outside_strings(expr, '-') {
            if idx > 0 { // Bukan tanda minus negatif angka
                let left = self.eval_expr(&expr[..idx], scope)?;
                let right = self.eval_expr(&expr[idx + 1..], scope)?;
                if let (Some(n1), Some(n2)) = (left.as_f64(), right.as_f64()) {
                    return Ok(json!(n1 - n2));
                }
            }
        }

        // Operator Aritmatika Perkalian (*), Pembagian (/), Modulo (%)
        if let Some(idx) = Self::find_operator_outside_strings(expr, '*') {
            let left = self.eval_expr(&expr[..idx], scope)?;
            let right = self.eval_expr(&expr[idx + 1..], scope)?;
            if let (Some(n1), Some(n2)) = (left.as_f64(), right.as_f64()) {
                return Ok(json!(n1 * n2));
            }
        }

        if let Some(idx) = Self::find_operator_outside_strings(expr, '/') {
            let left = self.eval_expr(&expr[..idx], scope)?;
            let right = self.eval_expr(&expr[idx + 1..], scope)?;
            if let (Some(n1), Some(n2)) = (left.as_f64(), right.as_f64()) {
                if n2 == 0.0 {
                    return Err(SandboxError::Trap("Pembagian dengan angka nol (Division by zero)".to_string()));
                }
                return Ok(json!(n1 / n2));
            }
        }

        // Pemanggilan Helper Bawaan / Fungsi: nama(arg1, arg2)
        if let Some(paren_idx) = expr.find('(') {
            if expr.ends_with(')') {
                let func_name = expr[..paren_idx].trim();
                let args_str = &expr[paren_idx + 1..expr.len() - 1].trim();

                let mut arg_values = Vec::new();
                if !args_str.is_empty() {
                    for arg in args_str.split(',') {
                        arg_values.push(self.eval_expr(arg.trim(), scope)?);
                    }
                }

                return self.call_builtin_or_custom(func_name, &arg_values, scope);
            }
        }

        // Literal String: "..." atau '...'
        if (expr.starts_with('"') && expr.ends_with('"') && expr.len() >= 2)
            || (expr.starts_with('\'') && expr.ends_with('\'') && expr.len() >= 2)
        {
            return Ok(json!(expr[1..expr.len() - 1]));
        }

        // Literal Boolean & Null
        match expr {
            "benar" | "bener" | "true" => return Ok(json!(true)),
            "salah" | "kliru" | "false" => return Ok(json!(false)),
            "kosong" | "suwung" | "null" => return Ok(Value::Null),
            _ => {}
        }

        // Literal Angka: Integer atau Float
        if let Ok(i) = expr.parse::<i64>() {
            return Ok(json!(i));
        }
        if let Ok(f) = expr.parse::<f64>() {
            return Ok(json!(f));
        }

        // Variabel Lookup di Scope
        let clean_var = expr.trim_start_matches('$');
        if let Some(val) = scope.get(clean_var) {
            return Ok(val.clone());
        }

        Ok(Value::Null)
    }

    /// Helper bawaan Indonesian Script (Standard Library)
    fn call_builtin_or_custom(&self, name: &str, args: &[Value], _scope: &HashMap<String, Value>) -> Result<Value, SandboxError> {
        match name {
            // panjang(val): Menghitung panjang karakter string atau item array
            "panjang" => {
                if let Some(arg) = args.first() {
                    let len = match arg {
                        Value::String(s) => s.chars().count(),
                        Value::Array(arr) => arr.len(),
                        _ => 0,
                    };
                    Ok(json!(len))
                } else {
                    Ok(json!(0))
                }
            }

            // rupiah(val): Format angka ke Rupiah
            "rupiah" | "formatRupiah" => {
                let num = args.first().and_then(|a| a.as_f64()).unwrap_or(0.0);
                let formatted = crate::core::blade::BladeEngine::format_rupiah(&num.to_string());
                Ok(json!(formatted))
            }

            // huruf_besar(val): Ubah ke UPPERCASE
            "huruf_besar" | "kapital" => {
                let s = args.first().and_then(|a| a.as_str()).unwrap_or("");
                Ok(json!(s.to_uppercase()))
            }

            // huruf_kecil(val): Ubah ke lowercase
            "huruf_kecil" => {
                let s = args.first().and_then(|a| a.as_str()).unwrap_or("");
                Ok(json!(s.to_lowercase()))
            }

            // potong(val, start, len)
            "potong" => {
                let s = args.first().and_then(|a| a.as_str()).unwrap_or("");
                let start = args.get(1).and_then(|a| a.as_u64()).unwrap_or(0) as usize;
                let len = args.get(2).and_then(|a| a.as_u64()).unwrap_or(s.len() as u64) as usize;
                let sub: String = s.chars().skip(start).take(len).collect();
                Ok(json!(sub))
            }

            // Panggilan fungsi user-defined di dalam skrip yang sama
            user_func => {
                if self.functions.contains_key(user_func) {
                    self.call(user_func, args)
                } else {
                    Err(SandboxError::FunctionNotFound(format!("Fungsi atau helper '{}' tidak dikenali", name)))
                }
            }
        }
    }

    fn find_operator_outside_strings(s: &str, op: char) -> Option<usize> {
        let mut in_quotes = false;
        let mut quote_char = ' ';

        for (i, c) in s.char_indices().rev() {
            if (c == '"' || c == '\'') && !in_quotes {
                in_quotes = true;
                quote_char = c;
            } else if in_quotes && c == quote_char {
                in_quotes = false;
            } else if !in_quotes && c == op {
                return Some(i);
            }
        }
        None
    }

    fn is_truthy(v: &Value) -> bool {
        match v {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => n.as_f64().map(|f| f != 0.0).unwrap_or(false),
            Value::String(s) => !s.is_empty() && s != "0" && s != "false",
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
        }
    }

    fn compare_num(v1: &Value, v2: &Value) -> Option<i32> {
        let n1 = v1.as_f64()?;
        let n2 = v2.as_f64()?;
        if (n1 - n2).abs() < f64::EPSILON {
            Some(0)
        } else if n1 > n2 {
            Some(1)
        } else {
            Some(-1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_evaluator_discount_calculation() {
        let source = r#"
        fungsi hitung_diskon(total, kupon) {
            jika kupon == "LUMINAHEMAT" dan total >= 100000 {
                kembalikan total * 0.15;
            } selain_itu jika kupon == "ONGKIR" {
                kembalikan 15000;
            }
            kembalikan 0;
        }
        "#;

        let runner = IsScriptRunner::from_source(source).expect("Parse gagal");

        // Uji kupon diskon 15%
        let res1 = runner.call("hitung_diskon", &[json!(200000), json!("LUMINAHEMAT")]).unwrap();
        assert_eq!(res1, json!(30000.0));

        // Uji kupon ongkir
        let res2 = runner.call("hitung_diskon", &[json!(50000), json!("ONGKIR")]).unwrap();
        assert_eq!(res2, json!(15000));

        // Uji tanpa kupon
        let res3 = runner.call("hitung_diskon", &[json!(50000), json!("LAIN")]).unwrap();
        assert_eq!(res3, json!(0));
    }

    #[test]
    fn test_is_evaluator_validation_rules() {
        let source = r#"
        fungsi validasi_ktp(nik, umur) {
            jika panjang(nik) != 16 {
                kembalikan "NIK wajib 16 digit";
            }
            jika umur < 17 {
                kembalikan "Umur minimal 17 tahun";
            }
            kembalikan benar;
        }
        "#;

        let runner = IsScriptRunner::from_source(source).expect("Parse gagal");

        // NIK kurang dari 16 digit
        let res_salah = runner.call("validasi_ktp", &[json!("12345"), json!(20)]).unwrap();
        assert_eq!(res_salah, json!("NIK wajib 16 digit"));

        // Umur kurang dari 17
        let res_umur = runner.call("validasi_ktp", &[json!("1234567890123456"), json!(15)]).unwrap();
        assert_eq!(res_umur, json!("Umur minimal 17 tahun"));

        // Valid
        let res_valid = runner.call("validasi_ktp", &[json!("1234567890123456"), json!(22)]).unwrap();
        assert_eq!(res_valid, json!(true));
    }

    #[test]
    fn test_is_evaluator_while_loop_and_string_helper() {
        let source = r#"
        fungsi buat_rangkuman(teks) {
            potongan = potong(teks, 0, 10);
            kembalikan huruf_besar(potongan) + "...";
        }
        "#;

        let runner = IsScriptRunner::from_source(source).expect("Parse gagal");
        let res = runner.call("buat_rangkuman", &[json!("selamat pagi indonesia")]).unwrap();
        assert_eq!(res, json!("SELAMAT PA..."));
    }
}
