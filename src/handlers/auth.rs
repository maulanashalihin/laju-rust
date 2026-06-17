use std::sync::Arc;
use axum::{response::{IntoResponse, Redirect, Response}, Extension, Form};
use axum_extra::extract::CookieJar;
use axum_inertia::Inertia;
use serde::Deserialize;
use serde_json::json;
use crate::app::AppState;
use crate::models::User;
use crate::services::auth::AuthService;

#[derive(Deserialize)]
pub struct RegisterForm { pub name: String, pub email: String, pub password: String, pub password_confirmation: String, }

#[derive(Deserialize)]
pub struct LoginForm { pub email: String, pub password: String, }

pub async fn register_page(
    inertia: Inertia,
    Extension(user): Extension<Option<User>>,
) -> Response {
    if user.is_some() { return Redirect::to("/dashboard").into_response(); }
    inertia.render("Register", json!({})).into_response()
}

pub async fn register_submit(
    inertia: Inertia,
    jar: CookieJar,
    Extension(state): Extension<Arc<AppState>>,
    Form(form): Form<RegisterForm>,
) -> Response {
    if form.password != form.password_confirmation {
        return inertia.render("Register", json!({
            "errors": {"password_confirmation": ["Password tidak cocok"]},
        })).into_response();
    }
    let svc = AuthService::new(state.db.clone());
    match svc.register(&form.name, &form.email, &form.password) {
        Ok(_) => {
            match svc.login(&form.email, &form.password) {
                Ok((sid, _)) => {
                    (jar.add(("session_id", sid)), Redirect::to("/dashboard")).into_response()
                }
                Err(_) => Redirect::to("/login").into_response(),
            }
        }
        Err(e) => inertia.render("Register", json!({
            "errors": {"email": [e]},
        })).into_response(),
    }
}

pub async fn login_page(
    inertia: Inertia,
    Extension(user): Extension<Option<User>>,
) -> Response {
    if user.is_some() { return Redirect::to("/dashboard").into_response(); }
    inertia.render("Login", json!({})).into_response()
}

pub async fn login_submit(
    inertia: Inertia,
    jar: CookieJar,
    Extension(state): Extension<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> Response {
    let svc = AuthService::new(state.db.clone());
    match svc.login(&form.email, &form.password) {
        Ok((sid, _)) => {
            (jar.add(("session_id", sid)), Redirect::to("/dashboard")).into_response()
        }
        Err(e) => inertia.render("Login", json!({
            "errors": {"email": [e]},
        })).into_response(),
    }
}

pub async fn logout(
    jar: CookieJar,
    Extension(state): Extension<Arc<AppState>>,
) -> Response {
    if let Some(c) = jar.get("session_id") {
        let svc = AuthService::new(state.db.clone());
        let _ = svc.logout(c.value());
    }
    (jar.remove("session_id"), Redirect::to("/")).into_response()
}

pub async fn dashboard(
    inertia: Inertia,
    Extension(user): Extension<Option<User>>,
) -> Response {
    match user {
        Some(u) => inertia.render("Dashboard", json!({"user": {"name": u.name, "email": u.email, "role": u.role}})).into_response(),
        None => Redirect::to("/login").into_response(),
    }
}
