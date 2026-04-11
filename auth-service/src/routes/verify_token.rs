use crate::app_state::AppState;
use crate::domain::AuthAPIError;
use crate::utils::validate_token;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use log::error;
use serde::Deserialize;

pub async fn verify_token_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<VerifyTokenRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let token = request.token;

    if state.banned_tokens.read().await.is_token_banned(&token) {
        error!("Attempt to use a banned token: {}", token);
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    let result = validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken);

    if result.is_err() {
        error!("Token is invalid: {}", token);
        return (jar, Err(result.err().unwrap()));
    }

    (jar, Ok(StatusCode::OK))
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
