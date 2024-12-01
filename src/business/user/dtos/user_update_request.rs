use serde::Deserialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::business::user::{EmailAddress, Name, User};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct UserUpdateRequest {
    id: uuid::Uuid,
    firstname: Name,
    lastname: Name,
    email: EmailAddress,
}

impl UserUpdateRequest {
    pub fn new(id: &uuid::Uuid, firstname: &Name, lastname: &Name, email: &EmailAddress) -> Self {
        Self {
            id: *id,
            firstname: firstname.clone(),
            lastname: lastname.clone(),
            email: email.clone(),
        }
    }
}

impl From<&UserUpdateRequest> for User {
    fn from(val: &UserUpdateRequest) -> Self {
        User::new(&val.id, &val.firstname, &val.lastname, &val.email)
    }
}

#[derive(Debug, Error)]
pub enum UserUpdateRequestError {
    #[error("User with id {id} does not exists")]
    UserNotExists { id: uuid::Uuid },
    #[error("Email {email} already used by other user")]
    EmailAlreadyUsedByOther { email: EmailAddress },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
