use crate::app_state::AppState;
use crate::domain::AuthAPIError;
use crate::utils::{validate_token, JWT_COOKIE_NAME};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use log::{error, info};

pub async fn logout_handler(
    State(state): State<AppState>,
    mut jar: CookieJar,
) -> impl IntoResponse {
    let cookie = &jar.get(JWT_COOKIE_NAME);

    if cookie.is_none() {
        error!("JWT cookie is missing in logout request");
        return (jar, Err(AuthAPIError::MissingToken));
    }

    if let Err(err) = validate_token(cookie.unwrap().value()).await {
        error!("Invalid token: {}", err);
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    state
        .banned_tokens
        .write()
        .await
        .add_token(cookie.unwrap().value().to_string());
    info!("Token added to banned list: {}", cookie.unwrap().value());
    jar = jar.remove(JWT_COOKIE_NAME);

    (jar, Ok(StatusCode::OK))
}
