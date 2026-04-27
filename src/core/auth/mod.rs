use serde::{Serialize, Deserialize};

pub mod hash;

/// Payload yang tersimpan di dalam JWT token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub:   String,   // user_id
    pub email: String,
    pub role:  String,
    pub exp:   usize,    // Unix timestamp — kapan token expire
    pub iat:   usize,    // Unix timestamp — kapan token dibuat
}

impl Claims {
    pub fn new(user_id: i64, email: String, role: String, hours: usize) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        Self {
            sub:   user_id.to_string(),
            email,
            role,
            iat:   now,
            exp:   now + (hours * 3600),
        }
    }
}
