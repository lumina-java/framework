use axum::Router as ax_router;
use axum::{handler::Handler, routing};

/// Lumina Router — fluent builder API yang membungkus axum::Router.
#[derive(Clone)]
pub struct Router<S = ()> {
    inner: ax_router<S>,
    prefix: String,
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
        }
    }

    /// Buat Lumina Router dari axum::Router yang sudah ada.
    pub fn from_axum(inner: ax_router<S>) -> Self {
        Self {
            inner,
            prefix: String::new(),
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
    /// ```
    /// router.group("/admin", |r| {
    ///     r.get("/dashboard", dashboard_handler)
    /// })
    /// ```
    pub fn group<F>(mut self, prefix: &str, f: F) -> Self
    where
        F: FnOnce(Router<S>) -> Router<S>,
    {
        let sub_router = Router::new();
        let configured_sub = f(sub_router);
        self.inner = self.inner.nest(prefix, configured_sub.inner);
        self
    }

    /// Gabungkan dengan Router lain.
    pub fn merge(mut self, other: Router<S>) -> Self {
        self.inner = self.inner.merge(other.inner);
        self
    }

    /// Nest router lain di bawah prefix tertentu.
    pub fn nest(mut self, prefix: &str, other: Router<S>) -> Self {
        self.inner = self.inner.nest(prefix, other.inner);
        self
    }

    /// Tambahkan middleware (layer) ke router.
    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: tower::Layer<axum::routing::Route> + Clone + Send + 'static,
        L::Service: tower::Service<axum::extract::Request> + Clone + Send + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Response:
            axum::response::IntoResponse + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Error:
            Into<std::convert::Infallible> + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Future: Send + 'static,
    {
        self.inner = self.inner.layer(layer);
        self
    }

    /// Alias untuk layer agar lebih familiar bagi pengguna Laravel.
    pub fn middleware<L>(self, layer: L) -> Self
    where
        L: tower::Layer<axum::routing::Route> + Clone + Send + 'static,
        L::Service: tower::Service<axum::extract::Request> + Clone + Send + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Response:
            axum::response::IntoResponse + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Error:
            Into<std::convert::Infallible> + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Future: Send + 'static,
    {
        self.layer(layer)
    }

    /// Konsumsi Router ini menjadi `axum::Router`.
    pub fn into_axum(self) -> ax_router<S> {
        self.inner
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
