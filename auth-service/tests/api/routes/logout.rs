use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::JWT_COOKIE_NAME;
use reqwest::Url;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
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

    let response = app.post_logout().await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
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

    let response = app.post_logout().await;
    assert_eq!(response.status(), 200);

    let response = app.post_logout().await;
    assert_eq!(response.status(), 400);
}
