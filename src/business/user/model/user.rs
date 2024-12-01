use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref};
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

lazy_static! {
    static ref EMAIL_REGEX: regex::Regex =
        regex::Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct User {
    id: uuid::Uuid,
    firstname: Name,
    lastname: Name,
    email: EmailAddress,
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("The id {id1} in the request differ the id {id2}")]
    MismatchUserId { id1: Uuid, id2: Uuid },
    #[error("Email {email} already used")]
    EmailAlreadyUsed { email: EmailAddress },
    #[error("User with id {id} does not exists")]
    UserNotExists { id: uuid::Uuid },
    #[error("Email {email} already used by other user")]
    EmailAlreadyUsedByOther { email: EmailAddress },
    #[error("offset value {offset} cannot be less than 1, please choose higher value")]
    PerPageOffsetTooLow { offset: u64 },
    #[error("per_page value {per_page} cannot be less than 1, please choose higher value")]
    PerPageValueTooLow { per_page: u16 },
    #[error("per_page value {per_page} is too high, please choose lower value")]
    PerPageValueTooHigh { per_page: u16 },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl User {
    pub fn new(id: &uuid::Uuid, firstname: &Name, lastname: &Name, email: &EmailAddress) -> Self {
        Self {
            id: *id,
            firstname: firstname.clone(),
            lastname: lastname.clone(),
            email: email.clone(),
        }
    }

    pub fn get_id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn get_firstname(&self) -> &Name {
        &self.firstname
    }

    pub fn get_lastname(&self) -> &Name {
        &self.lastname
    }

    pub fn get_email(&self) -> &EmailAddress {
        &self.email
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema)]
#[serde(try_from = "&str")]
pub struct Name(String);

impl Name {
    pub fn new(raw: &str) -> Result<Self, NameError> {
        let raw_trimed = raw.trim();
        Ok(Self(Self::validate_name(raw_trimed)?.to_string()))
    }

    fn validate_name(raw: &str) -> Result<&str, NameError> {
        if let Some(first_char) = raw.chars().next() {
            if first_char.is_uppercase() {
                return Ok(raw);
            }
        }
        Err(NameError {
            invalid_name: raw.to_string(),
        })
    }
}

impl TryFrom<&str> for Name {
    type Error = NameError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Error)]
#[error(
    "{invalid_name} is not a valid name. Name should not be empty and must begin with capital."
)]
pub struct NameError {
    pub invalid_name: String,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema)]
#[serde(try_from = "&str")]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email: &str) -> Result<Self, EmailAddressError> {
        let trimed = email.trim();
        Self::validate_email(trimed).map(|_| Self(trimed.to_string()))
    }

    fn validate_email(email: &str) -> Result<(), EmailAddressError> {
        if EMAIL_REGEX.is_match(email) {
            return Ok(());
        }
        Err(EmailAddressError {
            invalid_email: email.to_string(),
        })
    }
}

impl Deref for EmailAddress {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = EmailAddressError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Error)]
#[error("{invalid_email} is not a valid email address")]
pub struct EmailAddressError {
    pub invalid_email: String,
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use uuid::Uuid;

    use crate::business::user::EmailAddress;

    use super::{Name, User};

    #[test]
    fn test_name_ok() {
        let name_str = "RANDRIANASOLO";
        let name_valid = Name::new(name_str);
        assert!(name_valid.is_ok())
    }

    #[test]
    fn test_name_ko() {
        let name_str = "rakotobe";
        let name_valid = Name::new(name_str);
        assert!(name_valid.is_err())
    }

    #[test]
    fn test_name_try_from_str() {
        let name_str = "RANDRIANASOLO";
        let name_1 = Name::new(name_str).unwrap();
        let name_2 = Name::try_from(name_str).unwrap();
        assert_eq!(name_1, name_2)
    }

    #[test]
    fn test_name_deref() {
        let name = Name::new("RANDRIANASOLO").unwrap();
        assert_eq!(name.deref(), "RANDRIANASOLO");
    }

    #[test]
    fn test_name_display() {
        let name = Name::new("RANDRIANASOLO").unwrap();
        assert_eq!(&name.to_string(), "RANDRIANASOLO");
    }

    #[test]
    fn test_email_ok() {
        let email_str = "test@example.com";
        let email_valid = EmailAddress::new(email_str);
        assert!(email_valid.is_ok())
    }

    #[test]
    fn test_email_ko() {
        let email_str = "myemail@myemail";
        let email_valid = EmailAddress::new(email_str);
        assert!(email_valid.is_err())
    }

    #[test]
    fn test_email_try_from_str() {
        let email_str = "test@example.com";
        let email_1 = EmailAddress::new(email_str).unwrap();
        let email_2 = EmailAddress::try_from(email_str).unwrap();
        assert_eq!(email_1, email_2)
    }

    #[test]
    fn test_email_deref() {
        let email = EmailAddress::new("test@example.com").unwrap();
        assert_eq!(&*email, "test@example.com");
    }

    #[test]
    fn test_email_display() {
        let email = EmailAddress::new("test@example.com").unwrap();
        assert_eq!(&email.to_string(), "test@example.com");
    }

    #[test]
    fn test_user() {
        let user_id = Uuid::new_v4();
        let firstname = Name::new("John").unwrap();
        let lastname = Name::new("Doe").unwrap();
        let email = EmailAddress::new("test@example.com").unwrap();
        let user = User::new(&user_id, &firstname, &lastname, &email);
        assert_eq!(user.get_id(), &user_id);
        assert_eq!(user.get_lastname(), &lastname);
        assert_eq!(user.get_firstname(), &firstname);
        assert_eq!(user.get_email(), &email);
    }
}
