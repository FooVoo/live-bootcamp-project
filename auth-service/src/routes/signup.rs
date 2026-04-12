use crate::app_state::AppState;
use crate::domain::{AuthAPIError, Email, Password, User};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;
use serde::{Deserialize, Serialize};

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let email = Email::parse(&request.email);
    let password = Password::parse(&request.password);

    if email.is_err() || password.is_err() {
        error!(
            "Failed to parse email or password. Email error: {:?}, Password error: {:?}",
            email.err(),
            password.err()
        );
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let user = User::new(email.unwrap(), password.unwrap(), request.request_2fa);

    let mut user_store = state.user_store.write().await;

    let existing_user = user_store.get_user(&user.email).await;

    if let Ok(_error) = existing_user {
        error!(
            "Attempt to create a user that already exists: {}",
            user.email
        );
        return Err(AuthAPIError::UserAlreadyExists);
    }

    match user_store.add_user(user.clone()).await {
        Ok(_) => {}
        Err(_error) => {
            error!("Failed to create a user: {}", user.email);
            return Err(AuthAPIError::UnexpectedError);
        }
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub request_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupResponse {
    pub message: String,
}
