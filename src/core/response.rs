pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn ok(body: String) -> Self {
        Self {
            status: 200,
            body,
        }
    }
    
    pub fn json<T: serde::Serialize>(data: T) -> Self {
        let body = serde_json::to_string(&data).unwrap();
        Self {
            status: 200,
            body,
        }
    }
}
