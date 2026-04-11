use crate::app_state::AppState;
use crate::domain::{AuthAPIError, Email, Password};
use crate::utils::generate_auth_cookie;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use log::error;
use serde::Deserialize;

pub async fn login_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let store = state.user_store.read().await;

    let email = Email::parse(&request.email);
    let password = Password::parse(&request.password);

    if email.is_err() || password.is_err() {
        error!(
            "Failed to parse email or password. Email error: {:?}, Password error: {:?}",
            email.err(),
            password.err()
        );
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let email = email.unwrap();

    let user = store.get_user(&email).await;

    if user.is_err() || user.is_ok_and(|u| !u.password.eq(&password.unwrap())) {
        error!("Failed login attempt for email: {}", email);
        return (jar, Err(AuthAPIError::UserIsNotAuthenticated));
    }

    let auth_cookie = generate_auth_cookie(&email);

    if auth_cookie.is_err() {
        error!(
            "Failed to generate auth cookie. Auth error: {:?}",
            auth_cookie.err()
        );
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    let updated_jar = jar.add(auth_cookie.unwrap());

    (updated_jar, Ok(StatusCode::OK))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}
