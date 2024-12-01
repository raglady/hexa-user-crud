use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::business::user::{EmailAddress, Name, User};

#[derive(Debug, PartialEq, Eq, Deserialize, ToSchema)]
pub struct UserAddRequest {
    firstname: Name,
    lastname: Name,
    email: EmailAddress,
}

impl From<&UserAddRequest> for User {
    fn from(val: &UserAddRequest) -> Self {
        User::new(&Uuid::nil(), &val.firstname, &val.lastname, &val.email)
    }
}

impl UserAddRequest {
    pub fn new(firstname: &Name, lastname: &Name, email: &EmailAddress) -> Self {
        Self {
            email: email.clone(),
            firstname: firstname.clone(),
            lastname: lastname.clone(),
        }
    }

    pub fn get_firstname(&self) -> &Name {
        &self.firstname
    }

    pub fn get_email(&self) -> &EmailAddress {
        &self.email
    }

    pub fn get_lastname(&self) -> &Name {
        &self.lastname
    }
}
