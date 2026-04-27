pub struct Request {
    pub method: String,
    pub uri: String,
    pub body: String,
}

impl Request {
    pub fn new(method: String, uri: String) -> Self {
        Self {
            method,
            uri,
            body: String::new(),
        }
    }
}
