use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::JWT_COOKIE_NAME;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let token = serde_json::json!({});

    let response = app.post_verify_token(&token).await;

    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let user = serde_json::json!({
        "email": get_random_email(),
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&user).await;
    assert_eq!(response.status(), 201);

    let response = app.post_login(&user).await;
    assert_eq!(response.status(), 200);

    let cookie = response
        .cookies()
        .find(|cookie| -> bool { cookie.name() == JWT_COOKIE_NAME });

    let token_body = serde_json::json!({
        "token": cookie.unwrap().value().to_string()
    });

    let response = app.post_verify_token(&token_body).await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let token = serde_json::json!({
        "token": "INVALID_TOKEN",
    });

    let response = app.post_verify_token(&token).await;
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;

    let user = serde_json::json!({
        "email": get_random_email(),
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&user).await;
    assert_eq!(response.status(), 201);

    let response = app.post_login(&user).await;
    assert_eq!(response.status(), 200);

    let cookie = response
        .cookies()
        .find(|cookie| -> bool { cookie.name() == JWT_COOKIE_NAME });

    let token_body = serde_json::json!({
        "token": cookie.unwrap().value().to_string()
    });

    let response = app.post_logout().await;
    assert_eq!(response.status(), 200);

    let response = app.post_verify_token(&token_body).await;
    assert_eq!(response.status(), 401);
}
