use crate::helpers::{get_random_email, TestApp};
use auth_service::domain::ErrorResponse;
use auth_service::routes::SignupResponse;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "password": "",
            "requires2FA": false
        }),
        serde_json::json!({
            "password": "397469834@%#%@#%@",
            "requires2FA": true
        }),
        serde_json::json!({
            "password": "parkour123",
            "requires2FA": false
        }),
        serde_json::json!({
            "password": "exec(rm -rf /)",
            "requires2FA": false
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let test_user = serde_json::json!({
        "email": get_random_email(),
        "password": "397469834@%#%@#%@",
        "requires2FA": true
    });

    let response = app.post_signup(&test_user).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody")
            .message,
        expected_response.message
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [serde_json::json!({
        "email": "asdd.sdd",
        "password": "wrong",
        "requires2FA": false
    })];

    for i in test_cases.iter() {
        let response = app.post_signup(i).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let user = serde_json::json!({
       "email": get_random_email(),
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&user).await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app.post_signup(&user).await;
    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
