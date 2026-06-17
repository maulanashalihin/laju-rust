use std::sync::Arc;
use axum::{response::{IntoResponse, Redirect, Response}, Extension, Form};
use axum_inertia::Inertia;
use serde::Deserialize;
use serde_json::json;
use crate::app::AppState;
use crate::models::User;
use crate::services::auth::AuthService;

#[derive(Deserialize)]
pub struct ProfileForm {
    pub name: String,
    pub email: String,
}

pub async fn edit_page(
    inertia: Inertia,
    Extension(user): Extension<Option<User>>,
) -> Response {
    match user {
        Some(u) => inertia.render("Profile", json!({
            "user": {"name": u.name, "email": u.email, "role": u.role},
        })).into_response(),
        None => Redirect::to("/login").into_response(),
    }
}

pub async fn update(
    inertia: Inertia,
    Extension(state): Extension<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    Form(form): Form<ProfileForm>,
) -> Response {
    let current_user = match user {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let svc = AuthService::new(&state.db);
    match svc.update_profile(&current_user.id, &form.name, &form.email).await {
        Ok(u) => inertia.render("Profile", json!({
            "user": {"name": u.name, "email": u.email, "role": u.role},
            "flash": {"success": "Profil berhasil diupdate"},
        })).into_response(),
        Err(e) => inertia.render("Profile", json!({
            "user": {"name": current_user.name, "email": current_user.email, "role": current_user.role},
            "errors": {"email": [e]},
        })).into_response(),
    }
}
