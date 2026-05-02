use axum::response::IntoResponse;
use crate::core::response::Redirect;
use crate::core::view::View;
use serde_json::json;
use crate::core::request::Request;
use lumina_macros::lumina_form;
use crate::app::models::patient::Patient;
use crate::database::model::Model;

#[lumina_form]
pub struct PatientForm {
    #[rule("required|min:3")]
    pub name: String,
    // Tambahkan field lainnya di sini
    pub csrf_token: String,
}

pub struct PatientController;

impl PatientController {
    /// GET /patients
    pub async fn index(req: Request) -> impl IntoResponse {
        let items = Patient::query(req.state.db()).get().await.unwrap_or_default();

        View::make("patient.index")
            .with("items", &items)
            .render(&req)
            .await
    }

    /// GET /patients/create
    pub async fn create(req: Request) -> impl IntoResponse {
        View::make("patient.create")
            .with("csrf_token", req.token.authenticity_token().unwrap())
            .render(&req)
            .await
    }

    /// POST /patients
    pub async fn store(
        req: Request,
        crate::core::validation::ValidatedForm(form): crate::core::validation::ValidatedForm<PatientForm>,
    ) -> impl IntoResponse {
        if req.token.verify(&form.csrf_token).is_err() {
            return Redirect::to("/patients/create").with_error("Invalid CSRF Token").go(&req).await;
        }

        let result = sqlx::query("INSERT INTO patients (name, created_at, updated_at) VALUES (?, NOW(), NOW())")
            .bind(&form.name)
            .execute(&req.state.db().pool).await;

        match result {
            Ok(_) => Redirect::to("/patients").with_success("Data berhasil disimpan!").go(req.token, &req.session).await,
            Err(e) => Redirect::to("/patients/create")
                .with_input(json!({"name": form.name}))
                .with_error(format!("Gagal menyimpan data: {}", e))
                .go(&req).await
        }
    }

    /// GET /patients/:id/edit
    pub async fn edit(req: Request, axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
        let item = Patient::find(req.state.db(), id).await.expect("Data tidak ditemukan");

        View::make("patient.edit")
            .with("item", &item)
            .with("csrf_token", req.token.authenticity_token().unwrap())
            .render(&req)
            .await
    }

    /// POST /patients/:id/update
    pub async fn update(
        req: Request,
        axum::extract::Path(id): axum::extract::Path<i64>,
        crate::core::validation::ValidatedForm(form): crate::core::validation::ValidatedForm<PatientForm>,
    ) -> impl IntoResponse {
        if req.token.verify(&form.csrf_token).is_err() {
            return Redirect::to(&format!("/patients/{}/edit", id)).with_error("Invalid CSRF Token").go(&req).await;
        }

        let result = sqlx::query("UPDATE patients SET name = ?, updated_at = NOW() WHERE id = ?")
            .bind(&form.name)
            .bind(id)
            .execute(&req.state.db().pool).await;

        match result {
            Ok(_) => Redirect::to("/patients").with_success("Data berhasil diupdate!").go(req.token, &req.session).await,
            Err(e) => Redirect::to(&format!("/patients/{}/edit", id))
                .with_input(json!({"name": form.name}))
                .with_error(format!("Gagal update data: {}", e))
                .go(&req).await
        }
    }

    /// POST /patients/:id/delete
    pub async fn destroy(req: Request, axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
        let _ = sqlx::query("DELETE FROM patients WHERE id = ?")
            .bind(id)
            .execute(&req.state.db().pool).await;

        Redirect::to("/patients").with_success("Data berhasil dihapus!").go(&req).await
    }
}
