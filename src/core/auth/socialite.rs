use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, 
    AuthorizationCode, CsrfToken, Scope, PkceCodeChallenge, TokenResponse,
};
use serde::{Serialize, Deserialize};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialUser {
    pub id:      String,
    pub name:    String,
    pub email:   String,
    pub picture: Option<String>,
}

pub struct Socialite;

impl Socialite {
    pub fn build_client(provider: &str) -> Option<BasicClient> {
        let prefix = provider.to_uppercase();
        let client_id = std::env::var(format!("{}_CLIENT_ID", prefix)).ok()?;
        let client_secret = std::env::var(format!("{}_CLIENT_SECRET", prefix)).ok()?;
        let redirect_url = std::env::var(format!("{}_REDIRECT_URL", prefix)).ok()?;

        let (auth_url, token_url) = match provider {
            "google" => (
                "https://accounts.google.com/o/oauth2/v2/auth",
                "https://oauth2.googleapis.com/token",
            ),
            "github" => (
                "https://github.com/login/oauth/authorize",
                "https://github.com/login/oauth/access_token",
            ),
            _ => return None,
        };

        Some(
            BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(auth_url.to_string()).unwrap(),
                Some(TokenUrl::new(token_url.to_string()).unwrap()),
            )
            .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
        )
    }

    pub fn get_redirect_url(provider: &str) -> Option<(String, String)> {
        let client = Self::build_client(provider)?;
        let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let mut auth_request = client.authorize_url(CsrfToken::new_random);
        
        match provider {
            "google" => {
                auth_request = auth_request
                    .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
                    .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()));
            }
            "github" => {
                auth_request = auth_request.add_scope(Scope::new("user:email".to_string()));
            }
            _ => {}
        }

        let (url, csrf_token) = auth_request.set_pkce_challenge(pkce_challenge).url();
        Some((url.to_string(), csrf_token.secret().to_string()))
    }

    pub async fn get_user(provider: &str, code: String) -> Result<SocialUser, Box<dyn std::error::Error + Send + Sync>> {
        let client = Self::build_client(provider).ok_or("Provider not configured")?;
        
        // Note: In real implementation, you'd want to use the PKCE verifier stored in session
        // For simplicity, we skip PKCE verification here or use a simplified flow
        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await?;

        let access_token = token_result.access_token().secret();
        let http_client = reqwest::Client::new();

        match provider {
            "google" => {
                let res = http_client
                    .get("https://www.googleapis.com/oauth2/v2/userinfo")
                    .bearer_auth(access_token)
                    .send()
                    .await?
                    .json::<serde_json::Value>()
                    .await?;

                Ok(SocialUser {
                    id:      res["id"].as_str().unwrap_or_default().to_string(),
                    name:    res["name"].as_str().unwrap_or_default().to_string(),
                    email:   res["email"].as_str().unwrap_or_default().to_string(),
                    picture: res["picture"].as_str().map(|s| s.to_string()),
                })
            }
            "github" => {
                let mut headers = HeaderMap::new();
                headers.insert(USER_AGENT, HeaderValue::from_static("lumina-framework"));
                headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", access_token))?);

                let res = http_client
                    .get("https://api.github.com/user")
                    .headers(headers)
                    .send()
                    .await?
                    .json::<serde_json::Value>()
                    .await?;

                Ok(SocialUser {
                    id:      res["id"].to_string(),
                    name:    res["name"].as_str().unwrap_or_else(|| res["login"].as_str().unwrap_or_default()).to_string(),
                    email:   res["email"].as_str().unwrap_or_default().to_string(),
                    picture: res["avatar_url"].as_str().map(|s| s.to_string()),
                })
            }
            _ => Err("Unsupported provider".into()),
        }
    }
}
