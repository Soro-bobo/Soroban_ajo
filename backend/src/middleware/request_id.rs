use axum::{extract::Request, middleware::Next, response::Response};
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let request_id = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    req.extensions_mut().insert(RequestId(request_id.clone()));

    let span = tracing::info_span!("request", request_id = %request_id);
    let _enter = span.enter();

    let mut response = next.run(req).await;
    response.headers_mut().insert(
        REQUEST_ID_HEADER,
        request_id.parse().expect("valid header value"),
    );
    response
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct RequestId(pub String);
