use axum::Router as ax_router;
use axum::{handler::Handler, routing};
use std::sync::{Arc, Mutex};

pub use axum::middleware::{from_fn, from_fn_with_state, Next};

/// Lumina Router — fluent builder API yang membungkus axum::Router.
#[derive(Clone)]
pub struct Router<S = ()> {
    inner: ax_router<S>,
    prefix: String,
    layers: Vec<Arc<dyn Fn(ax_router<S>) -> ax_router<S> + Send + Sync>>,
    routes_doc: Arc<Mutex<Vec<crate::core::openapi::RouteDoc>>>,
}

impl<S> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Buat router baru yang kosong.
    pub fn new() -> Self {
        Self {
            inner: ax_router::new(),
            prefix: String::new(),
            layers: Vec::new(),
            routes_doc: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Buat Lumina Router dari axum::Router yang sudah ada.
    pub fn from_axum(inner: ax_router<S>) -> Self {
        Self {
            inner,
            prefix: String::new(),
            layers: Vec::new(),
            routes_doc: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Tentukan prefix untuk semua route yang didaftarkan setelah ini.
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    fn normalize_path(&self, path: &str) -> String {
        if self.prefix.is_empty() {
            return path.to_string();
        }
        format!(
            "{}/{}",
            self.prefix.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    /// Daftarkan route GET.
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let full_path = self.normalize_path(path);
        if let Ok(mut docs) = self.routes_doc.lock() {
            docs.push(crate::core::openapi::RouteDoc::new("GET", &full_path));
        }
        self.inner = self.inner.route(&full_path, routing::get(handler));
        self
    }

    /// Daftarkan route POST.
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let full_path = self.normalize_path(path);
        if let Ok(mut docs) = self.routes_doc.lock() {
            docs.push(crate::core::openapi::RouteDoc::new("POST", &full_path));
        }
        self.inner = self.inner.route(&full_path, routing::post(handler));
        self
    }

    /// Daftarkan route PUT.
    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let full_path = self.normalize_path(path);
        if let Ok(mut docs) = self.routes_doc.lock() {
            docs.push(crate::core::openapi::RouteDoc::new("PUT", &full_path));
        }
        self.inner = self.inner.route(&full_path, routing::put(handler));
        self
    }

    /// Daftarkan route DELETE.
    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let full_path = self.normalize_path(path);
        if let Ok(mut docs) = self.routes_doc.lock() {
            docs.push(crate::core::openapi::RouteDoc::new("DELETE", &full_path));
        }
        self.inner = self.inner.route(&full_path, routing::delete(handler));
        self
    }

    /// Daftarkan route PATCH.
    pub fn patch<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let full_path = self.normalize_path(path);
        if let Ok(mut docs) = self.routes_doc.lock() {
            docs.push(crate::core::openapi::RouteDoc::new("PATCH", &full_path));
        }
        self.inner = self.inner.route(&full_path, routing::patch(handler));
        self
    }

    /// Tambahkan ringkasan (summary) untuk route terakhir yang didaftarkan (OpenAPI).
    pub fn summary(self, summary: impl Into<String>) -> Self {
        if let Ok(mut docs) = self.routes_doc.lock() {
            if let Some(last) = docs.last_mut() {
                last.summary = Some(summary.into());
            }
        }
        self
    }

    /// Tambahkan deskripsi lengkap untuk route terakhir yang didaftarkan (OpenAPI).
    pub fn description(self, description: impl Into<String>) -> Self {
        if let Ok(mut docs) = self.routes_doc.lock() {
            if let Some(last) = docs.last_mut() {
                last.description = Some(description.into());
            }
        }
        self
    }

    /// Tambahkan satu kategori/tag untuk route terakhir (OpenAPI).
    pub fn tag(self, tag: impl Into<String>) -> Self {
        if let Ok(mut docs) = self.routes_doc.lock() {
            if let Some(last) = docs.last_mut() {
                last.tags.push(tag.into());
            }
        }
        self
    }

    /// Tambahkan banyak tag untuk route terakhir (OpenAPI).
    pub fn tags(self, tags: &[&str]) -> Self {
        if let Ok(mut docs) = self.routes_doc.lock() {
            if let Some(last) = docs.last_mut() {
                for t in tags {
                    last.tags.push(t.to_string());
                }
            }
        }
        self
    }

    /// Tandai route terakhir sebagai usang / deprecated (OpenAPI).
    pub fn deprecated(self) -> Self {
        if let Ok(mut docs) = self.routes_doc.lock() {
            if let Some(last) = docs.last_mut() {
                last.deprecated = true;
            }
        }
        self
    }

    /// Tetapkan operationId unik untuk route terakhir (OpenAPI).
    pub fn operation_id(self, id: impl Into<String>) -> Self {
        if let Ok(mut docs) = self.routes_doc.lock() {
            if let Some(last) = docs.last_mut() {
                last.operation_id = Some(id.into());
            }
        }
        self
    }

    /// Daftarkan fallback handler (misal untuk menangani 404 Not Found).
    pub fn fallback<H, T>(mut self, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.inner = self.inner.fallback(handler);
        self
    }

    /// Grup route dengan prefix dan kustomisasi (seperti middleware).
    /// Contoh:
    /// ```rust,ignore
    /// router.group("/admin", |r| {
    ///     r.middleware(from_fn(admin_auth))
    ///      .get("/dashboard", dashboard_handler)
    /// })
    /// ```
    pub fn group<F>(mut self, prefix: &str, f: F) -> Self
    where
        F: FnOnce(Router<S>) -> Router<S>,
    {
        let sub_router = Router::new();
        let configured_sub = f(sub_router);

        if let Ok(sub_docs) = configured_sub.routes_doc.lock() {
            if let Ok(mut my_docs) = self.routes_doc.lock() {
                for mut doc in sub_docs.clone() {
                    doc.path = format!(
                        "{}/{}",
                        prefix.trim_end_matches('/'),
                        doc.path.trim_start_matches('/')
                    );
                    my_docs.push(doc);
                }
            }
        }

        self.inner = self.inner.nest(prefix, configured_sub.into_axum());
        self
    }

    /// Gabungkan dengan Router lain.
    pub fn merge(mut self, other: Router<S>) -> Self {
        if let Ok(other_docs) = other.routes_doc.lock() {
            if let Ok(mut my_docs) = self.routes_doc.lock() {
                my_docs.extend(other_docs.clone());
            }
        }
        self.inner = self.inner.merge(other.into_axum());
        self
    }

    /// Nest router lain di bawah prefix tertentu.
    pub fn nest(mut self, prefix: &str, other: Router<S>) -> Self {
        if let Ok(other_docs) = other.routes_doc.lock() {
            if let Ok(mut my_docs) = self.routes_doc.lock() {
                for mut doc in other_docs.clone() {
                    doc.path = format!(
                        "{}/{}",
                        prefix.trim_end_matches('/'),
                        doc.path.trim_start_matches('/')
                    );
                    my_docs.push(doc);
                }
            }
        }
        self.inner = self.inner.nest(prefix, other.into_axum());
        self
    }

    /// Tambahkan middleware (layer) ke router.
    /// Middleware ini akan diterapkan ke semua route yang ada di router ini saat dibuild.
    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        L::Service: tower::Service<axum::extract::Request> + Clone + Send + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Response:
            axum::response::IntoResponse + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Error:
            Into<std::convert::Infallible> + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Future: Send + 'static,
    {
        self.layers
            .push(Arc::new(move |router| router.layer(layer.clone())));
        self
    }

    /// Alias untuk layer agar lebih familiar bagi pengguna Laravel.
    pub fn middleware<L>(self, layer: L) -> Self
    where
        L: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        L::Service: tower::Service<axum::extract::Request> + Clone + Send + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Response:
            axum::response::IntoResponse + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Error:
            Into<std::convert::Infallible> + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Future: Send + 'static,
    {
        self.layer(layer)
    }

    /// Daftarkan async middleware function secara langsung tanpa perlu `from_fn(...)`.
    /// 
    /// Contoh:
    /// ```rust,ignore
    /// async fn auth_guard(req: Request, next: Next) -> Response {
    ///     next.run(req).await
    /// }
    /// 
    /// Router::new()
    ///     .guard(auth_guard)
    ///     .get("/protected", handler)
    /// ```
    pub fn guard<F, T>(self, f: F) -> Self
    where
        F: Clone + Send + Sync + 'static,
        axum::middleware::FromFnLayer<F, (), T>: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        <axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service:
            tower::Service<axum::extract::Request> + Clone + Send + 'static,
        <<axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service as tower::Service<axum::extract::Request>>::Response:
            axum::response::IntoResponse + 'static,
        <<axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service as tower::Service<axum::extract::Request>>::Error:
            Into<std::convert::Infallible> + 'static,
        <<axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service as tower::Service<axum::extract::Request>>::Future:
            Send + 'static,
    {
        self.layer(axum::middleware::from_fn(f))
    }

    /// Alias untuk `guard` agar konsisten dengan `middleware_fn`.
    pub fn middleware_fn<F, T>(self, f: F) -> Self
    where
        F: Clone + Send + Sync + 'static,
        axum::middleware::FromFnLayer<F, (), T>: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        <axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service:
            tower::Service<axum::extract::Request> + Clone + Send + 'static,
        <<axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service as tower::Service<axum::extract::Request>>::Response:
            axum::response::IntoResponse + 'static,
        <<axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service as tower::Service<axum::extract::Request>>::Error:
            Into<std::convert::Infallible> + 'static,
        <<axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service as tower::Service<axum::extract::Request>>::Future:
            Send + 'static,
    {
        self.guard(f)
    }

    /// Daftarkan async middleware function pada route-level tanpa perlu `from_fn(...)`.
    pub fn route_guard<F, T>(self, f: F) -> Self
    where
        F: Clone + Send + Sync + 'static,
        axum::middleware::FromFnLayer<F, (), T>: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        <axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service:
            tower::Service<axum::extract::Request> + Clone + Send + 'static,
        <<axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service as tower::Service<axum::extract::Request>>::Response:
            axum::response::IntoResponse + 'static,
        <<axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service as tower::Service<axum::extract::Request>>::Error:
            Into<std::convert::Infallible> + 'static,
        <<axum::middleware::FromFnLayer<F, (), T> as tower::Layer<axum::routing::Route>>::Service as tower::Service<axum::extract::Request>>::Future:
            Send + 'static,
    {
        self.route_layer(axum::middleware::from_fn(f))
    }

    /// Tambahkan route-level middleware (hanya berlaku untuk route pada router ini).
    pub fn route_layer<L>(mut self, layer: L) -> Self
    where
        L: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        L::Service: tower::Service<axum::extract::Request> + Clone + Send + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Response:
            axum::response::IntoResponse + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Error:
            Into<std::convert::Infallible> + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Future: Send + 'static,
    {
        self.layers
            .push(Arc::new(move |router| router.route_layer(layer.clone())));
        self
    }

    /// Alias untuk `route_layer` agar familiar bagi pengguna Laravel.
    pub fn route_middleware<L>(self, layer: L) -> Self
    where
        L: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        L::Service: tower::Service<axum::extract::Request> + Clone + Send + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Response:
            axum::response::IntoResponse + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Error:
            Into<std::convert::Infallible> + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Future: Send + 'static,
    {
        self.route_layer(layer)
    }

    /// Lindungi semua route pada router ini dengan pengecekan izin (Permission).
    /// Mengembalikan 401 Unauthorized jika belum login, atau 403 Forbidden jika tidak punya izin.
    pub fn can(self, permission: &'static str) -> Self {
        self.route_layer(axum::middleware::from_fn(
            move |req: axum::extract::Request, next: axum::middleware::Next| async move {
                let user = req.extensions().get::<crate::core::auth::AuthUser>();
                match user {
                    Some(u) if u.can(permission) => next.run(req).await,
                    Some(_) => crate::http::middleware::forbidden_json(&format!(
                        "Akses ditolak: Izin '{}' diperlukan.",
                        permission
                    )),
                    None => crate::http::middleware::unauthorized_json("Silakan login terlebih dahulu."),
                }
            },
        ))
    }

    /// Lindungi semua route pada router ini dengan pengecekan peran (Role).
    /// Mengembalikan 401 Unauthorized jika belum login, atau 403 Forbidden jika role tidak sesuai.
    pub fn has_role(self, role: &'static str) -> Self {
        self.route_layer(axum::middleware::from_fn(
            move |req: axum::extract::Request, next: axum::middleware::Next| async move {
                let user = req.extensions().get::<crate::core::auth::AuthUser>();
                match user {
                    Some(u) if u.has_role(role) => next.run(req).await,
                    Some(_) => crate::http::middleware::forbidden_json(&format!(
                        "Akses ditolak: Role '{}' diperlukan.",
                        role
                    )),
                    None => crate::http::middleware::unauthorized_json("Silakan login terlebih dahulu."),
                }
            },
        ))
    }

    /// Lindungi route dengan Zero-Panic Error Boundary.
    /// Menjamin bahwa panic di dalam handler tidak akan pernah meruntuhkan server.
    pub fn panic_boundary(self) -> Self {
        self.layer(axum::middleware::from_fn(
            crate::core::middleware::zero_panic_boundary,
        ))
    }

    /// Tambahkan security headers standar industri (OWASP) ke semua route pada router ini.
    pub fn security_headers(self) -> Self {
        self.layer(axum::middleware::from_fn(
            crate::http::middleware::security_headers,
        ))
    }

    /// Dapatkan daftar metadata route yang didaftarkan untuk OpenAPI
    pub fn routes_doc(&self) -> Vec<crate::core::openapi::RouteDoc> {
        self.routes_doc.lock().map(|d| d.clone()).unwrap_or_default()
    }

    /// Menghasilkan struktur OpenApiSpec lengkap dari seluruh route yang terdaftar
    pub fn openapi_spec(&self, title: &str, version: &str) -> crate::core::openapi::OpenApiSpec {
        let docs = self.routes_doc();
        crate::core::openapi::OpenApiSpec::from_routes(&docs, title, version)
    }

    /// Menghasilkan string JSON OpenAPI v3.0 yang rapi
    pub fn openapi_json(&self, title: &str, version: &str) -> String {
        self.openapi_spec(title, version).to_json_pretty().unwrap_or_default()
    }

    /// Menghasilkan string YAML OpenAPI v3.0
    pub fn openapi_yaml(&self, title: &str, version: &str) -> String {
        self.openapi_spec(title, version).to_yaml()
    }

    /// Ekspor spesifikasi OpenAPI ke file JSON
    pub fn export_openapi_json(&self, path: impl AsRef<std::path::Path>, title: &str, version: &str) -> std::io::Result<()> {
        let json = self.openapi_json(title, version);
        std::fs::write(path, json)
    }

    /// Ekspor spesifikasi OpenAPI ke file YAML
    pub fn export_openapi_yaml(&self, path: impl AsRef<std::path::Path>, title: &str, version: &str) -> std::io::Result<()> {
        let yaml = self.openapi_yaml(title, version);
        std::fs::write(path, yaml)
    }

    /// Ekspor koleksi Postman v2.1 ke file JSON
    pub fn export_postman(&self, path: impl AsRef<std::path::Path>, collection_name: &str, version: &str) -> std::io::Result<()> {
        let spec = self.openapi_spec(collection_name, version);
        let postman = spec.to_postman_collection(collection_name);
        let json = serde_json::to_string_pretty(&postman).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, json)
    }

    /// Pasang Swagger UI interaktif dan endpoint OpenAPI JSON secara otomatis.
    ///
    /// Menyediakan:
    /// - `GET {mount_path}` dan `GET {mount_path}/swagger`: Antarmuka visual Swagger UI
    /// - `GET {mount_path}/openapi.json`: Spesifikasi OpenAPI v3 mentah berformat JSON
    ///
    /// Contoh:
    /// ```rust,ignore
    /// router.swagger_ui("/docs", "Lumina API Documentation", "1.0.0")
    /// ```
    pub fn swagger_ui(mut self, mount_path: &str, title: &str, version: &str) -> Self
    where
        S: Clone + Send + Sync + 'static,
    {
        let base_mount = mount_path.trim_end_matches('/').to_string();
        let json_path = format!("{}/openapi.json", base_mount);
        let title_owned = title.to_string();
        let version_owned = version.to_string();
        let swagger_html = crate::core::openapi::generate_swagger_ui_html(&json_path, title);

        let routes_arc = self.routes_doc.clone();
        let json_handler = move || {
            let r_arc = routes_arc.clone();
            let t = title_owned.clone();
            let v = version_owned.clone();
            async move {
                let docs = r_arc.lock().map(|d| d.clone()).unwrap_or_default();
                let spec = crate::core::openapi::OpenApiSpec::from_routes(&docs, &t, &v);
                let json = spec.to_json_pretty().unwrap_or_default();
                ([(axum::http::header::CONTENT_TYPE, "application/json")], json)
            }
        };

        let html_clone1 = swagger_html.clone();
        let html_clone2 = swagger_html;

        let swagger_route = format!("{}/swagger", base_mount);
        let root_route = if base_mount.is_empty() { "/docs".to_string() } else { base_mount.clone() };

        self.inner = self.inner
            .route(&root_route, routing::get(move || {
                let h = html_clone1.clone();
                async move { axum::response::Html(h) }
            }))
            .route(&swagger_route, routing::get(move || {
                let h = html_clone2.clone();
                async move { axum::response::Html(h) }
            }))
            .route(&json_path, routing::get(json_handler));

        self
    }

    /// Konsumsi Router ini menjadi `axum::Router` beserta seluruh layer middleware-nya.
    pub fn into_axum(self) -> ax_router<S> {
        let mut inner = self.inner;
        for layer_fn in self.layers {
            inner = layer_fn(inner);
        }
        inner
    }
}

impl<S> Default for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
