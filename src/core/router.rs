use axum::{
    Router as AxumRouter,
    handler::Handler,
    routing,
};

/// Lumina Router — fluent builder API yang membungkus axum::Router.
///
/// Desain terinspirasi oleh Laravel Router:
/// ```
/// Router::new()
///     .get("/",          HomeController::index)
///     .get("/users/:id", UserController::show)
///     .post("/users",    UserController::store)
/// ```
pub struct Router {
    inner: AxumRouter,
}

impl Router {
    /// Buat router baru yang kosong.
    pub fn new() -> Self {
        Self {
            inner: AxumRouter::new(),
        }
    }

    /// Daftarkan route GET.
    /// Mendukung path parameter: `"/users/:id"`, `"/posts/:slug"`
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::get(handler));
        self
    }

    /// Daftarkan route POST.
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::post(handler));
        self
    }

    /// Daftarkan route PUT.
    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::put(handler));
        self
    }

    /// Daftarkan route DELETE.
    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::delete(handler));
        self
    }

    /// Daftarkan route PATCH.
    pub fn patch<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::patch(handler));
        self
    }

    /// Gabungkan dengan Router lain (untuk merging web + api routes).
    pub fn merge(mut self, other: Router) -> Self {
        self.inner = self.inner.merge(other.inner);
        self
    }

    /// Nest router lain di bawah prefix tertentu.
    ///
    /// Contoh: `.nest("/api", api_router)` → semua route dalam `api_router`
    /// akan diakses sebagai `/api/...`
    pub fn nest(mut self, prefix: &str, other: Router) -> Self {
        self.inner = self.inner.nest(prefix, other.inner);
        self
    }

    /// Buat Lumina Router dari `axum::Router` yang sudah ada.
    /// Berguna ketika perlu menggunakan fitur Axum seperti `route_layer`
    /// yang tidak terekspos langsung oleh Lumina Router.
    pub fn from_axum(router: AxumRouter) -> Self {
        Self { inner: router }
    }

    /// Konsumsi Router ini menjadi `axum::Router` untuk dipakai oleh server.
    pub fn into_axum(self) -> AxumRouter {
        self.inner
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
