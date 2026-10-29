use axum::Router as ax_router;
use axum::{handler::Handler, routing};
use std::sync::Arc;

pub use axum::middleware::{from_fn, from_fn_with_state, Next};

/// Lumina Router — fluent builder API yang membungkus axum::Router.
#[derive(Clone)]
pub struct Router<S = ()> {
    inner: ax_router<S>,
    prefix: String,
    layers: Vec<Arc<dyn Fn(ax_router<S>) -> ax_router<S> + Send + Sync>>,
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
        }
    }

    /// Buat Lumina Router dari axum::Router yang sudah ada.
    pub fn from_axum(inner: ax_router<S>) -> Self {
        Self {
            inner,
            prefix: String::new(),
            layers: Vec::new(),
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
        self.inner = self.inner.route(&full_path, routing::patch(handler));
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
        self.inner = self.inner.nest(prefix, configured_sub.into_axum());
        self
    }

    /// Gabungkan dengan Router lain.
    pub fn merge(mut self, other: Router<S>) -> Self {
        self.inner = self.inner.merge(other.into_axum());
        self
    }

    /// Nest router lain di bawah prefix tertentu.
    pub fn nest(mut self, prefix: &str, other: Router<S>) -> Self {
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
