use async_trait::async_trait;
use fake::Fake;
use fake::faker::name::raw::*;
use fake::faker::internet::raw::*;
use fake::locales::*;
use crate::database::{connection::DatabasePool, factory::Factory, model::Model};
use crate::app::models::user::User;
use crate::core::auth::Auth;

pub struct UserFactory;

#[async_trait]
impl Factory for UserFactory {
    type Model = User;

    fn definition(&self) -> Self::Model {
        User {
            name: Name(EN).fake(),
            email: SafeEmail(EN).fake(),
            password: Auth::make_hash("password"),
            role: "user".to_string(),
            ..Default::default()
        }
    }

    async fn create(&self, db: &DatabasePool) -> Result<Self::Model, String> {
        let mut model = self.definition();
        match model.save(db).await {
            Ok(id) => {
                model.id = id;
                Ok(model)
            },
            Err(e) => Err(format!("Gagal menyimpan user ke database: {}", e)),
        }
    }
}
