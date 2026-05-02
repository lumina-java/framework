use axum::response::IntoResponse;
use crate::core::response::Redirect;
use crate::core::view::View;
use serde_json::json;
use crate::core::request::Request;
use lumina_macros::lumina_form;
use crate::app::models::contact::Contact;
use crate::database::model::Model;

#[lumina_form]
pub struct ContactForm {
    #[rule("required|min:3")]
    pub name: String,
    // Tambahkan field lainnya di sini
    pub csrf_token: String,
}

pub struct ContactController;

impl ContactController {
    /// GET /contacts
    pub async fn index(req: Request) -> impl IntoResponse {
        let items = Contact::query(req.state.db()).get().await.unwrap_or_default();

        View::make("contact.index")
            .with("items", &items)
            .render(&req)
            .await
    }

    /// GET /contacts/create
    pub async fn create(req: Request) -> impl IntoResponse {
        View::make("contact.create")
            .with("csrf_token", req.token.authenticity_token().unwrap())
            .render(&req)
            .await
    }

    /// POST /contacts
    pub async fn store(
        req: Request,
        crate::core::validation::ValidatedForm(form): crate::core::validation::ValidatedForm<ContactForm>,
    ) -> impl IntoResponse {
        if req.token.verify(&form.csrf_token).is_err() {
            return Redirect::to("/contacts/create").with_error("Invalid CSRF Token").go(&req).await
        }

        let result = sqlx::query("INSERT INTO contacts (name, created_at, updated_at) VALUES (?, NOW(), NOW())")
            .bind(&form.name)
            .execute(&req.state.db().pool).await;

        match result {
            Ok(_) => Redirect::to("/contacts").with_success("Data berhasil disimpan!").go(&req).await,
            Err(e) => Redirect::to("/contacts/create")
                .with_input(json!({"name": form.name}))
                .with_error(&format!("Gagal menyimpan data: {}", e))
                .go(&req).await
        }
    }

    /// GET /contacts/:id/edit
    pub async fn edit(req: Request, axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
        let item = Contact::find(req.state.db(), id).await.expect("Data tidak ditemukan");

        View::make("contact.edit")
            .with("item", &item)
            .with("csrf_token", req.token.authenticity_token().unwrap())
            .render(&req)
            .await
    }

    /// POST /contacts/:id/update
    pub async fn update(
        req: Request,
        axum::extract::Path(id): axum::extract::Path<i64>,
        crate::core::validation::ValidatedForm(form): crate::core::validation::ValidatedForm<ContactForm>,
    ) -> impl IntoResponse {
        if req.token.verify(&form.csrf_token).is_err() {
            return Redirect::to(&format!("/contacts/{}/edit", id)).with_error("Invalid CSRF Token").go(&req).await
        }

        let result = sqlx::query("UPDATE contacts SET name = ?, updated_at = NOW() WHERE id = ?")
            .bind(&form.name)
            .bind(id)
            .execute(&req.state.db().pool).await;

        match result {
            Ok(_) => Redirect::to("/contacts").with_success("Data berhasil diupdate!").go(&req).await,
            Err(e) => Redirect::to(&format!("/contacts/{}/edit", id))
                .with_input(json!({"name": form.name}))
                .with_error(&format!("Gagal update data: {}", e))
                .go(&req).await
        }
    }

    /// POST /contacts/:id/delete
    pub async fn destroy(req: Request, axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
        let _ = sqlx::query("DELETE FROM contacts WHERE id = ?")
            .bind(id)
            .execute(&req.state.db().pool).await;

        Redirect::to("/contacts").with_success("Data berhasil dihapus!").go(&req).await
    }
}
