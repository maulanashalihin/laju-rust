use axum::{Router, routing::get, routing::post};

pub fn register<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    axum_inertia::InertiaConfig: axum::extract::FromRef<S>,
{
    router
        .route("/", get(crate::handlers::home::index))
        .route("/about", get(crate::handlers::about::index))
        .route("/register", get(crate::handlers::auth::register_page).post(crate::handlers::auth::register_submit))
        .route("/login", get(crate::handlers::auth::login_page).post(crate::handlers::auth::login_submit))
        .route("/logout", post(crate::handlers::auth::logout))
        .route("/dashboard", get(crate::handlers::auth::dashboard))
        .route("/profile", get(crate::handlers::profile::edit_page).post(crate::handlers::profile::update))
}
