use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

mod html_escape {
    pub fn encode_safe(input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}

#[derive(Clone, Debug, Default)]
pub struct BladeContext {
    data: HashMap<String, Value>,
}

impl BladeContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert<T: serde::Serialize>(&mut self, key: &str, value: T) {
        if let Ok(val) = serde_json::to_value(value) {
            self.data.insert(key.to_string(), val);
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        if !key.contains('.') {
            return self.data.get(key);
        }

        let parts: Vec<&str> = key.split('.').collect();
        let mut curr = self.data.get(parts[0])?;
        for part in &parts[1..] {
            if let Some(obj) = curr.as_object() {
                curr = obj.get(*part)?;
            } else if let Ok(idx) = part.parse::<usize>() {
                if let Some(arr) = curr.as_array() {
                    curr = arr.get(idx)?;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        Some(curr)
    }
}

pub struct BladeEngine {
    templates: HashMap<String, String>,
}

impl BladeEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            templates: HashMap::new(),
        };
        engine.reload_templates("resources/views");
        engine
    }

    pub fn reload_templates(&mut self, views_path: &str) {
        self.templates.clear();
        if Path::new(views_path).is_dir() {
            Self::collect_templates(views_path, "", &mut self.templates);
        }
    }

    fn collect_templates(base_path: &str, prefix: &str, out: &mut HashMap<String, String>) {
        let path = if prefix.is_empty() {
            base_path.to_string()
        } else {
            format!("{}/{}", base_path, prefix)
        };

        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                let file_name = entry.file_name().into_string().unwrap_or_default();

                if file_path.is_dir() {
                    let new_prefix = if prefix.is_empty() {
                        file_name
                    } else {
                        format!("{}/{}", prefix, file_name)
                    };
                    Self::collect_templates(base_path, &new_prefix, out);
                } else if file_name.ends_with(".blade.rs") || file_name.ends_with(".html") {
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        let template_name = if prefix.is_empty() {
                            file_name
                        } else {
                            format!("{}/{}", prefix, file_name)
                        };
                        out.insert(template_name, content);
                    }
                }
            }
        }
    }

    pub fn render(&self, template_name: &str, context: &BladeContext) -> String {
        let mut name = template_name.replace('.', "/");
        if !name.ends_with(".blade.rs") && !name.ends_with(".html") {
            name.push_str(".blade.rs");
        }

        let raw = match self.templates.get(&name) {
            Some(content) => content.clone(),
            None => return format!("Template error: Template '{}' not found", name),
        };

        self.process_template(&raw, context, 0)
    }

    fn process_template(&self, content: &str, context: &BladeContext, depth: usize) -> String {
        if depth > 20 {
            return "Template recursion limit exceeded".to_string();
        }

        let mut s = content.to_string();

        // 0. {{-- Comment --}}
        let re_comment = Regex::new(r"(?s)\{\{--.*?--\}\}").unwrap();
        s = re_comment.replace_all(&s, "").to_string();

        // 1. Check for @extends('layout')
        let re_extends = Regex::new(r#"@extends\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
        if let Some(caps) = re_extends.captures(&s) {
            let layout_name = caps[1].replace('.', "/");
            let layout_file = if layout_name.ends_with(".blade.rs") || layout_name.ends_with(".html") {
                layout_name
            } else {
                format!("{}.blade.rs", layout_name)
            };

            // Extract @section('name') ... @endsection
            let mut sections: HashMap<String, String> = HashMap::new();
            let re_section = Regex::new(r#"(?s)@section\s*\(\s*['"]([^'"]+)['"]\s*\)(.*?)@(?:endsection|stop|endblock)"#).unwrap();
            for sec_caps in re_section.captures_iter(&s) {
                let sec_name = sec_caps[1].to_string();
                let sec_content = sec_caps[2].to_string();
                sections.insert(sec_name, sec_content);
            }

            // Load parent layout template
            if let Some(parent_content) = self.templates.get(&layout_file) {
                let mut layout_processed = parent_content.clone();
                // Replace @yield('name') with section content
                let re_yield = Regex::new(r#"@yield\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
                layout_processed = re_yield.replace_all(&layout_processed, |y_caps: &regex::Captures| {
                    let y_name = &y_caps[1];
                    sections.get(y_name).cloned().unwrap_or_default()
                }).to_string();

                s = layout_processed;
            }
        }

        // 2. @include('path.subpath')
        let re_include = Regex::new(r#"@include\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
        s = re_include.replace_all(&s, |caps: &regex::Captures| {
            let path = caps[1].replace('.', "/");
            let inc_name = if path.ends_with(".blade.rs") || path.ends_with(".html") {
                path
            } else {
                format!("{}.blade.rs", path)
            };
            if let Some(inc_content) = self.templates.get(&inc_name) {
                self.process_template(inc_content, context, depth + 1)
            } else {
                format!("<!-- Include '{}' not found -->", inc_name)
            }
        }).to_string();

        // 3. Directives: @csrf, @method, @is, @script
        s = s.replace("@csrf", &format!("<input type=\"hidden\" name=\"csrf_token\" value=\"{}\">", context.get("csrf_token").and_then(|v| v.as_str()).unwrap_or("")));

        let re_method = Regex::new(r#"@method\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
        s = re_method.replace_all(&s, "<input type=\"hidden\" name=\"_method\" value=\"$1\">").to_string();

        let re_is = Regex::new(r#"(?:@is|@script)\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
        s = re_is.replace_all(&s, |caps: &regex::Captures| {
            let script_name = caps[1].trim();
            let mut file_path = format!("resources/scripts/{}", script_name);
            if !file_path.ends_with(".is") {
                file_path.push_str(".is");
            }

            let source = if let Ok(content) = fs::read_to_string(&file_path) {
                content
            } else {
                let alt_path = format!("resources/js/{}", script_name);
                let alt_path_is = if alt_path.ends_with(".is") { alt_path.clone() } else { format!("{}.is", alt_path) };
                fs::read_to_string(&alt_path_is).unwrap_or_else(|_| format!("// File script '.is' tidak ditemukan: {}", file_path))
            };

            let compiled_js = indonesian_script::compile_is(&source);
            format!("<script>\n{}\n</script>", compiled_js)
        }).to_string();

        // 4. Interpolation: {!! raw !!} and {{ escaped }}
        let re_raw = Regex::new(r"\{!!\s*\$?([\w.]+)\s*!!\}").unwrap();
        s = re_raw.replace_all(&s, |caps: &regex::Captures| {
            let key = &caps[1];
            self.eval_val(key, context)
        }).to_string();

        let re_var = Regex::new(r"\{\{\s*\$?([\w.]+)\s*\}\}").unwrap();
        s = re_var.replace_all(&s, |caps: &regex::Captures| {
            let key = &caps[1];
            html_escape::encode_safe(&self.eval_val(key, context)).to_string()
        }).to_string();

        s
    }

    fn eval_val(&self, key: &str, context: &BladeContext) -> String {
        if let Some(val) = context.get(key) {
            match val {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "".to_string(),
                v => v.to_string(),
            }
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_blade_engine() {
        let mut engine = BladeEngine {
            templates: HashMap::new(),
        };
        engine.templates.insert("layouts/app.blade.rs".to_string(), "<html><body>@yield('content')</body></html>".to_string());
        engine.templates.insert("home.blade.rs".to_string(), "@extends('layouts.app')\n@section('content')<h1>Halo {{ name }}</h1>@endsection".to_string());

        let mut ctx = BladeContext::new();
        ctx.insert("name", "Lumina Native Blade");

        let html = engine.render("home", &ctx);
        assert!(html.contains("<html><body><h1>Halo Lumina Native Blade</h1></body></html>"));
    }
}
