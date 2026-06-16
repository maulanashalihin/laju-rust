use std::sync::atomic::{AtomicU64, Ordering};
use axum::{extract::Request, middleware::Next, response::Response};

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(1);

pub async fn middleware(mut req: Request, next: Next) -> Response {
    let request_id = REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    req.extensions_mut().insert(request_id);

    let mut response = next.run(req).await;
    response.headers_mut().insert(
        "X-Request-Id",
        format!("{}", request_id).parse().unwrap(),
    );
    response
}
