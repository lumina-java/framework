use lumina_macros::lumina_form;

#[lumina_form]
pub struct RegisterRequest {
    #[rule("required|min:3")] pub name: String,
    #[rule("required|email")] pub email: String,
    #[rule("required|min:6")] pub password: String,
    pub password_confirmation: String,
    pub csrf_token: String,
}

#[lumina_form]
pub struct LoginRequest {
    #[rule("required|email")] pub email: String,
    #[rule("required")]       pub password: String,
    pub csrf_token: String,
}
