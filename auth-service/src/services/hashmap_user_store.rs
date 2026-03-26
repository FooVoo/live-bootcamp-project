use crate::domain::{Email, User, UserStoreError};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

impl HashmapUserStore {
    pub async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);

        Ok(())
    }

    pub async fn get_user(&self, user_email: &str) -> Result<User, UserStoreError> {
        let user = self.users.get(&Email::parse(user_email).unwrap());

        match user {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub async fn validate_user(
        &self,
        user_email: &str,
        user_password: &str,
    ) -> Result<(), UserStoreError> {
        if user_email == "" || user_password == "" {
            return Err(UserStoreError::InvalidCredentials);
        }

        Ok(())
    }
}
