use crate::domain::User;
use crate::services::hashmap_user_store::HashmapUserStore;
use async_trait::async_trait;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, user_email: &str) -> Result<User, UserStoreError>;
    async fn validate_user(
        &self,
        user_email: &str,
        user_password: &str,
    ) -> Result<(), UserStoreError>;
}

#[async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        self.add_user(user).await
    }

    async fn get_user(&self, user_email: &str) -> Result<User, UserStoreError> {
        self.get_user(user_email).await
    }

    async fn validate_user(
        &self,
        user_email: &str,
        user_password: &str,
    ) -> Result<(), UserStoreError> {
        self.validate_user(user_email, user_password).await
    }
}
