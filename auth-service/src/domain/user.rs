use crate::domain::{Email, Password};

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub request_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, request_2fa: bool) -> Self {
        User {
            email,
            password,
            request_2fa,
        }
    }
}
