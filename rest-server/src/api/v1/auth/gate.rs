use axum::Extension;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use http::HeaderMap;
use crate::AppState;

pub async fn protected(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    mut req: Request,
    next: Next
) -> impl IntoResponse {
}
