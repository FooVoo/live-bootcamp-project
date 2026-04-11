use crate::domain::UserStore;
use crate::services::banned_store::BannedTokenStore;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;

pub type BannedStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_tokens: BannedStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_tokens: BannedStoreType) -> Self {
        Self {
            user_store,
            banned_tokens,
        }
    }
}
