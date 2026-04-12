extern crate pretty_env_logger;
use auth_service::app_state::AppState;
use auth_service::services::banned_store::HashsetBannedTokenStore;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::utils::prod::APP_ADDRESS;
use auth_service::Application;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::default();
    let banned_tokens = HashsetBannedTokenStore::default();
    let app_state = AppState::new(
        Arc::new(tokio::sync::RwLock::new(user_store)),
        Arc::new(tokio::sync::RwLock::new(banned_tokens)),
    );

    let app = Application::build(app_state, APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
