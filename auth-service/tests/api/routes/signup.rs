use crate::helpers::{get_random_email, TestApp};

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
