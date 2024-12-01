use serde::Deserialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct UserDeleteRequest(uuid::Uuid);

impl UserDeleteRequest {
    pub fn new(user_id: &uuid::Uuid) -> Self {
        Self(*user_id)
    }

    pub fn get_user_id(&self) -> &uuid::Uuid {
        &self.0
    }
}

#[derive(Debug, Error)]
pub enum UserDeleteRequestError {
    #[error("User with id {id} does not exists")]
    UserNotExists { id: uuid::Uuid },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
