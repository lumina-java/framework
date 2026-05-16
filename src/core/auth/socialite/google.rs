use crate::core::auth::socialite::provider::{SocialProvider, SocialUser};
use async_trait::async_trait;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::Deserialize;

pub struct GoogleProvider {
    client: BasicClient,
    http_client: Client,
}

impl GoogleProvider {
    pub fn new(client_id: String, client_secret: String, redirect_url: String) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

        Self {
            client,
            http_client: Client::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct GoogleUserResponse {
    id: String,
    name: String,
    email: String,
    picture: Option<String>,
}

#[async_trait]
impl SocialProvider for GoogleProvider {
    fn name(&self) -> &str {
        "google"
    }

    fn get_redirect_url(&self) -> String {
        let (auth_url, _csrf_token) = self
            .client
            .authorize_url(oauth2::CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ))
            .url();

        auth_url.to_string()
    }

    async fn get_user_by_code(&self, code: String) -> Result<SocialUser, String> {
        // Exchange code for token
        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| format!("Gagal exchange token: {}", e))?;

        let access_token = token_result.access_token().secret();

        // Fetch user info from Google API
        let user_info = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v1/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| format!("Gagal memanggil Google API: {}", e))?
            .json::<GoogleUserResponse>()
            .await
            .map_err(|e| format!("Gagal parse data user Google: {}", e))?;

        Ok(SocialUser {
            id: user_info.id,
            name: user_info.name,
            email: user_info.email,
            avatar: user_info.picture,
            provider: "google".to_string(),
        })
    }
}
