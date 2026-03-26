use validator::{ValidateEmail, ValidationError};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
