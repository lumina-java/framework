use validator::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate)]
pub struct StoreUserRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,

    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
}
