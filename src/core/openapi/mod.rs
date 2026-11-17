use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Representasi dokumentasi rute individual untuk OpenAPI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteDoc {
    pub method: String,
    pub path: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub deprecated: bool,
    pub operation_id: Option<String>,
}

impl RouteDoc {
    pub fn new(method: &str, path: &str) -> Self {
        Self {
            method: method.to_uppercase(),
            path: path.to_string(),
            summary: None,
            description: None,
            tags: Vec::new(),
            deprecated: false,
            operation_id: None,
        }
    }
}

/// Struktur data spesifikasi OpenAPI v3.0.3
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: OpenApiInfo,
    pub servers: Vec<OpenApiServer>,
    pub paths: BTreeMap<String, BTreeMap<String, OpenApiOperation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<OpenApiComponents>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenApiInfo {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenApiServer {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenApiOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub parameters: Vec<OpenApiParameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<OpenApiRequestBody>,
    pub responses: BTreeMap<String, OpenApiResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenApiParameter {
    pub name: String,
    pub r#in: String, // "path", "query", "header"
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenApiRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub required: bool,
    pub content: BTreeMap<String, OpenApiMediaType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenApiMediaType {
    pub schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenApiResponse {
    pub description: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub content: BTreeMap<String, OpenApiMediaType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenApiComponents {
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub schemas: BTreeMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub security_schemes: BTreeMap<String, serde_json::Value>,
}

impl OpenApiSpec {
    pub fn new(title: &str, version: &str) -> Self {
        let mut security_schemes = BTreeMap::new();
        security_schemes.insert(
            "BearerAuth".to_string(),
            serde_json::json!({
                "type": "http",
                "scheme": "bearer",
                "bearerFormat": "JWT",
                "description": "Masukkan token JWT dengan format Bearer <token>"
            }),
        );

        Self {
            openapi: "3.0.3".to_string(),
            info: OpenApiInfo {
                title: title.to_string(),
                version: version.to_string(),
                description: Some("Dokumentasi REST API Otomatis yang dihasilkan oleh Lumina Framework.".to_string()),
            },
            servers: vec![OpenApiServer {
                url: "/".to_string(),
                description: Some("Current Server".to_string()),
            }],
            paths: BTreeMap::new(),
            components: Some(OpenApiComponents {
                schemas: BTreeMap::new(),
                security_schemes,
            }),
        }
    }

    /// Mengkonversi koleksi RouteDoc ke format OpenAPI v3
    pub fn from_routes(routes: &[RouteDoc], title: &str, version: &str) -> Self {
        let mut spec = Self::new(title, version);

        for route in routes {
            // Normalisasi path Axum `:param` menjadi `{param}` standar OpenAPI
            let (openapi_path, path_params) = normalize_path_to_openapi(&route.path);

            let method_lower = route.method.to_lowercase();
            let mut operation = OpenApiOperation {
                summary: route.summary.clone(),
                description: route.description.clone(),
                tags: route.tags.clone(),
                operation_id: route.operation_id.clone(),
                deprecated: if route.deprecated { Some(true) } else { None },
                parameters: Vec::new(),
                request_body: None,
                responses: BTreeMap::new(),
            };

            // Tambahkan parameter path otomatis yang terdeteksi
            for param in path_params {
                operation.parameters.push(OpenApiParameter {
                    name: param.clone(),
                    r#in: "path".to_string(),
                    required: true,
                    description: Some(format!("Parameter path {}", param)),
                    schema: serde_json::json!({
                        "type": if param.ends_with("_id") || param == "id" { "integer" } else { "string" }
                    }),
                });
            }

            // Tambahkan default response
            operation.responses.insert(
                "200".to_string(),
                OpenApiResponse {
                    description: "Operasi berhasil".to_string(),
                    content: {
                        let mut m = BTreeMap::new();
                        m.insert(
                            "application/json".to_string(),
                            OpenApiMediaType {
                                schema: serde_json::json!({
                                    "type": "object",
                                    "properties": {
                                        "success": { "type": "boolean" },
                                        "message": { "type": "string" },
                                        "data": { "type": "object" }
                                    }
                                }),
                            },
                        );
                        m
                    },
                },
            );

            // Tambahkan 422 untuk method yang membawa payload (POST, PUT, PATCH)
            if matches!(method_lower.as_str(), "post" | "put" | "patch") {
                operation.responses.insert(
                    "422".to_string(),
                    OpenApiResponse {
                        description: "Validasi data gagal (Unprocessable Entity)".to_string(),
                        content: {
                            let mut m = BTreeMap::new();
                            m.insert(
                                "application/json".to_string(),
                                OpenApiMediaType {
                                    schema: serde_json::json!({
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean", "example": false },
                                            "message": { "type": "string", "example": "Validasi gagal" },
                                            "errors": { "type": "object" }
                                        }
                                    }),
                                },
                            );
                            m
                        },
                    },
                );

                // Tambahkan template request body
                operation.request_body = Some(OpenApiRequestBody {
                    description: Some("Data payload request".to_string()),
                    required: true,
                    content: {
                        let mut m = BTreeMap::new();
                        m.insert(
                            "application/json".to_string(),
                            OpenApiMediaType {
                                schema: serde_json::json!({
                                    "type": "object"
                                }),
                            },
                        );
                        m
                    },
                });
            }

            spec.paths
                .entry(openapi_path)
                .or_default()
                .insert(method_lower, operation);
        }

        spec
    }

    /// Serialisasi spesifikasi ke string JSON yang rapi
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialisasi spesifikasi ke format YAML
    pub fn to_yaml(&self) -> String {
        let json_val = serde_json::to_value(self).unwrap_or_default();
        json_to_yaml(&json_val, 0)
    }

    /// Ekspor ke koleksi Postman v2.1
    pub fn to_postman_collection(&self, collection_name: &str) -> serde_json::Value {
        let mut items = Vec::new();

        for (path, methods) in &self.paths {
            for (method, op) in methods {
                let name = op
                    .summary
                    .clone()
                    .unwrap_or_else(|| format!("{} {}", method.to_uppercase(), path));

                let path_segments: Vec<&str> = path.trim_matches('/').split('/').collect();

                let mut item = serde_json::json!({
                    "name": name,
                    "request": {
                        "method": method.to_uppercase(),
                        "header": [
                            {
                                "key": "Accept",
                                "value": "application/json",
                                "type": "text"
                            },
                            {
                                "key": "Content-Type",
                                "value": "application/json",
                                "type": "text"
                            }
                        ],
                        "url": {
                            "raw": format!("{{{{base_url}}}}{}", path),
                            "host": ["{{base_url}}"],
                            "path": path_segments
                        },
                        "description": op.description.clone().unwrap_or_default()
                    },
                    "response": []
                });

                if matches!(method.as_str(), "post" | "put" | "patch") {
                    item["request"]["body"] = serde_json::json!({
                        "mode": "raw",
                        "raw": "{\n  \n}",
                        "options": {
                            "raw": {
                                "language": "json"
                            }
                        }
                    });
                }

                items.push(item);
            }
        }

        serde_json::json!({
            "info": {
                "_postman_id": uuid::Uuid::new_v4().to_string(),
                "name": collection_name,
                "description": self.info.description.clone().unwrap_or_default(),
                "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
            },
            "item": items,
            "variable": [
                {
                    "key": "base_url",
                    "value": "http://127.0.0.1:3000",
                    "type": "string"
                }
            ]
        })
    }
}

/// Menghasilkan halaman HTML mandiri Swagger UI v5
pub fn generate_swagger_ui_html(spec_url: &str, title: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title} — Swagger UI</title>
  <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.18.2/swagger-ui.css" />
  <link rel="icon" type="image/png" href="https://unpkg.com/swagger-ui-dist@5.18.2/favicon-32x32.png" sizes="32x32" />
  <style>
    html {{
      box-sizing: border-box;
      overflow: -moz-scrollbars-vertical;
      overflow-y: scroll;
    }}
    *, *:before, *:after {{
      box-sizing: inherit;
    }}
    body {{
      margin: 0;
      background: #fafafa;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    }}
    .topbar {{
      display: none !important;
    }}
    .lumina-header {{
      background: linear-gradient(135deg, #1e1e2e 0%, #2d2b55 100%);
      color: #ffffff;
      padding: 1.25rem 2rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      box-shadow: 0 4px 12px rgba(0,0,0,0.1);
    }}
    .lumina-header h1 {{
      margin: 0;
      font-size: 1.35rem;
      font-weight: 700;
      letter-spacing: -0.02em;
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }}
    .lumina-badge {{
      background: rgba(255, 255, 255, 0.2);
      padding: 0.2rem 0.6rem;
      border-radius: 9999px;
      font-size: 0.75rem;
      font-weight: 600;
    }}
    .lumina-links a {{
      color: #a5b4fc;
      text-decoration: none;
      font-size: 0.875rem;
      font-weight: 500;
      margin-left: 1rem;
      transition: color 0.2s;
    }}
    .lumina-links a:hover {{
      color: #ffffff;
      text-decoration: underline;
    }}
  </style>
</head>
<body>
  <div class="lumina-header">
    <h1>
      <span>✨ {title}</span>
      <span class="lumina-badge">OpenAPI 3.0</span>
    </h1>
    <div class="lumina-links">
      <a href="{spec_url}" target="_blank">Raw JSON Spec</a>
      <a href="https://lumina-java.github.io/docs/" target="_blank">Dokumentasi Lumina</a>
    </div>
  </div>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5.18.2/swagger-ui-bundle.js" charset="UTF-8"></script>
  <script src="https://unpkg.com/swagger-ui-dist@5.18.2/swagger-ui-standalone-preset.js" charset="UTF-8"></script>
  <script>
    window.onload = function() {{
      const ui = SwaggerUIBundle({{
        url: "{spec_url}",
        dom_id: '#swagger-ui',
        deepLinking: true,
        presets: [
          SwaggerUIBundle.presets.apis,
          SwaggerUIStandalonePreset
        ],
        plugins: [
          SwaggerUIBundle.plugins.DownloadUrl
        ],
        layout: "StandaloneLayout",
        defaultModelsExpandDepth: -1,
        docExpansion: "list",
        filter: true
      }});
      window.ui = ui;
    }};
  </script>
</body>
</html>"#,
        title = title,
        spec_url = spec_url
    )
}

/// Normalisasi path Axum `:param` atau `*param` menjadi `{param}` untuk OpenAPI
fn normalize_path_to_openapi(path: &str) -> (String, Vec<String>) {
    let mut params = Vec::new();
    let segments: Vec<String> = path
        .split('/')
        .map(|segment| {
            if let Some(param) = segment.strip_prefix(':') {
                params.push(param.to_string());
                format!("{{{}}}", param)
            } else if let Some(param) = segment.strip_prefix('*') {
                params.push(param.to_string());
                format!("{{{}}}", param)
            } else if segment.starts_with('{') && segment.ends_with('}') {
                let param = &segment[1..segment.len() - 1];
                params.push(param.to_string());
                segment.to_string()
            } else {
                segment.to_string()
            }
        })
        .collect();

    (segments.join("/"), params)
}

/// Serializer ringan JSON-to-YAML murni tanpa dependensi eksternal tambahan
fn json_to_yaml(value: &serde_json::Value, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => {
            if s.contains('\n') || s.contains(':') || s.contains('#') || s.contains(' ') || s.is_empty() {
                format!("\"{}\"", s.replace('"', "\\\""))
            } else {
                s.clone()
            }
        }
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else {
                let mut out = String::new();
                for item in arr {
                    out.push('\n');
                    out.push_str(&format!("{}- {}", pad, json_to_yaml(item, indent + 1).trim_start()));
                }
                out
            }
        }
        serde_json::Value::Object(map) => {
            if map.is_empty() {
                "{}".to_string()
            } else {
                let mut out = String::new();
                for (k, v) in map {
                    out.push('\n');
                    match v {
                        serde_json::Value::Object(_) => {
                            out.push_str(&format!("{}{}:{}", pad, k, json_to_yaml(v, indent + 1)));
                        }
                        serde_json::Value::Array(_) => {
                            out.push_str(&format!("{}{}:{}", pad, k, json_to_yaml(v, indent + 1)));
                        }
                        _ => {
                            out.push_str(&format!("{}{}: {}", pad, k, json_to_yaml(v, indent)));
                        }
                    }
                }
                out
            }
        }
    }
}

