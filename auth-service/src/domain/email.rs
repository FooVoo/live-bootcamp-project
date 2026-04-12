use serde::Deserialize;
use std::fmt::Display;
use validator::ValidationError;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct EmailStruct(String);

pub type Email = EmailStruct;

impl Email {
    pub fn parse(email: &str) -> Result<Email, ValidationError> {
        if !email.contains('@') {
            return Err(ValidationError::new(
                "Invalid email address: missing '@' symbol",
            ));
        }
        Ok(EmailStruct(email.to_string()))
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
