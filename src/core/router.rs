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
pub struct Router<S = ()> {
    inner: AxumRouter<S>,
}

impl<S> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Buat router baru yang kosong.
    pub fn new() -> Self {
        Self {
            inner: AxumRouter::new(),
        }
    }

    /// Daftarkan route GET.
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::get(handler));
        self
    }

    /// Daftarkan route POST.
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::post(handler));
        self
    }

    /// Daftarkan route PUT.
    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::put(handler));
        self
    }

    /// Daftarkan route DELETE.
    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::delete(handler));
        self
    }

    /// Daftarkan route PATCH.
    pub fn patch<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.inner = self.inner.route(path, routing::patch(handler));
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

    /// Buat Lumina Router dari `axum::Router` yang sudah ada.
    pub fn from_axum(router: AxumRouter<S>) -> Self {
        Self { inner: router }
    }

    /// Konsumsi Router ini menjadi `axum::Router`.
    pub fn into_axum(self) -> AxumRouter<S> {
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
