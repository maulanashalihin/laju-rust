use std::sync::Arc;
use axum::{extract::Request, middleware::Next, response::Response};
use crate::app::AppState;
use crate::services::auth::AuthService;

pub async fn resolve_user(
    mut req: Request,
    next: Next,
) -> Response {
    let session_id = req.headers()
        .get("Cookie")
        .and_then(|c| c.to_str().ok())
        .and_then(|c| {
            c.split(';')
                .map(|s| s.trim())
                .filter_map(|s| s.split_once('='))
                .find(|(k, _)| k.trim() == "session_id")
                .map(|(_, v)| v.to_string())
        });

    let user = if let Some(sid) = session_id {
        if let Some(state) = req.extensions().get::<Arc<AppState>>() {
            let svc = AuthService::new(state.db.clone());
            svc.get_user_by_session(&sid).ok().flatten()
        } else { None }
    } else { None };

    req.extensions_mut().insert(user);
    next.run(req).await
}
