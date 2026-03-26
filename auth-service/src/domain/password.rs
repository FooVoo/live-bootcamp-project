use validator::ValidationError;

pub type Password = PasswordStruct;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PasswordStruct(String);

impl PasswordStruct {
    pub(crate) fn parse(password: &str) -> Result<Password, ValidationError> {
        if password.len() < 8 {
            return Err(ValidationError::new("too short"));
        }

        Ok(PasswordStruct(password.to_string()))
    }
}

impl AsRef<str> for PasswordStruct {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
