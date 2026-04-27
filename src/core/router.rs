pub struct Router {
    routes: Vec<Route>,
}

pub struct Route {
    pub method: String,
    pub path: String,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }
    
    pub fn add_route(&mut self, method: &str, path: &str) {
        self.routes.push(Route {
            method: method.to_string(),
            path: path.to_string(),
        });
    }
}
