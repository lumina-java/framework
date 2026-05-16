use crate::core::{request::Request, response::Response};
use async_trait::async_trait;

#[async_trait]
pub trait Middleware: Send + Sync {
    async fn handle(&self, request: Request) -> Result<Response, String>;
}
