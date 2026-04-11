use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::JWT_COOKIE_NAME;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let payload = serde_json::json!({});

    let result = app.post_login(&payload).await;

    assert_eq!(result.status(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let payload = serde_json::json!({
        "email": "invalid-email",
        "password": "short"
    });

    let result = app.post_login(&payload).await;

    assert_eq!(result.status(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let user = serde_json::json!({
       "email": get_random_email(),
        "password": "password123",
        "requires2FA": true
    });

    let result_for_signup = app.post_signup(&user).await;

    assert_eq!(result_for_signup.status(), 201);

    let login_payload = serde_json::json!({
    "email": user["email"],
    "password": "wrongpassword"
    });

    let result = app.post_login(&login_payload).await;

    assert_eq!(result.status(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}
