use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SocialUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub provider: String,
}

#[async_trait]
pub trait SocialProvider: Send + Sync {
    /// Nama provider (misal: "google", "github").
    fn name(&self) -> &str;

    /// Mendapatkan URL untuk redirect user ke provider OAuth.
    fn get_redirect_url(&self) -> String;

    /// Menangani callback dari provider dan mendapatkan data user.
    async fn get_user_by_code(&self, code: String) -> Result<SocialUser, String>;
}
