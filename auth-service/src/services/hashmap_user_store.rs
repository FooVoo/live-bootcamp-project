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

    pub async fn get_user(&self, user_email: &Email) -> Result<User, UserStoreError> {
        let user = self.users.get(&user_email);

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
        let email = Email::parse(user_email);
        let password = Email::parse(user_password);

        if email.is_err() || password.is_err() {
            return Err(UserStoreError::InvalidCredentials);
        }

        Ok(())
    }
}
