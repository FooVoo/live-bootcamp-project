use auth_service::domain::User;
use auth_service::services::hashmap_user_store::HashmapUserStore;

#[tokio::test]
async fn test_add_user() {
    let mut store = HashmapUserStore::default();
    let user_email = Box::new("test@mail.tt".to_owned());
    let user_password = Box::new("password11123".to_owned());
    let mock_user = User::new(user_email.to_string(), user_password.to_string(), false);

    store
        .add_user(User::new(
            user_email.to_string(),
            user_password.to_string(),
            false,
        ))
        .await
        .unwrap();

    assert_eq!(store.get_user(&user_email).await.unwrap(), mock_user);
}

#[tokio::test]
async fn test_get_user() {
    let mut store = HashmapUserStore::default();
    let user_email = Box::new("test@mail.tt".to_owned());
    let user_password = Box::new("password11123".to_owned());
    let mock_user = User::new(user_email.to_string(), user_password.to_string(), false);

    store
        .add_user(mock_user.clone())
        .await
        .expect("Failed to add user");

    let user = store.get_user(&mock_user.email).await.unwrap();

    assert_eq!(user.email, mock_user.email);
}

// #[tokio::test]
// async fn test_validate_user() {
//     todo!()
// }
