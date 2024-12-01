use std::borrow::Borrow;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::{IntoParams, ToSchema};

use crate::{
    business::user::{model::user::UserError, User},
    outbound::repository_trait::{FindOptionTrait, FindResultTrait},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, IntoParams)]
pub struct UserFindRequest {
    filters: UserFindRequestFilter,
    order_by: String,
    per_page: u16,
    offset: u64,
}

impl UserFindRequest {
    pub fn new(
        filters: &UserFindRequestFilter,
        order_by: &str,
        per_page: &u16,
        offset: &u64,
    ) -> Result<Self, UserFindRequestError> {
        let per_page = Self::validate_per_page(per_page)?;
        let offset = Self::validate_offset(offset)?;
        Ok(Self {
            filters: filters.clone(),
            order_by: order_by.to_string(),
            per_page: *per_page,
            offset: *offset,
        })
    }

    fn validate_per_page(per_page: &u16) -> Result<&u16, UserFindRequestError> {
        if per_page < &1 {
            return Err(UserFindRequestError::PerPageValueTooLow {
                per_page: *per_page,
            });
        } else if per_page > &1000 {
            return Err(UserFindRequestError::PerPageValueTooHigh {
                per_page: *per_page,
            });
        }
        Ok(per_page)
    }

    fn validate_offset(offset: &u64) -> Result<&u64, UserFindRequestError> {
        if offset < &1 {
            return Err(UserFindRequestError::PerPageOffsetTooLow { offset: *offset });
        }
        Ok(offset)
    }

    pub fn set_filters(&mut self, filters: &UserFindRequestFilter) {
        self.filters = filters.clone();
    }

    pub fn set_per_page(&mut self, per_page: &u16) {
        self.per_page = *per_page;
    }

    pub fn set_offset(&mut self, offset: &u64) {
        self.offset = *offset;
    }
}

impl Default for UserFindRequest {
    fn default() -> Self {
        Self {
            filters: UserFindRequestFilter::default(),
            order_by: String::new(),
            per_page: 25,
            offset: 1,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, IntoParams, ToSchema)]
pub struct UserFindRequestFilter {
    pub id: Option<uuid::Uuid>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserFindResponse {
    users: Vec<User>,
    num_pages: u64,
}

impl UserFindResponse {
    pub fn new(users: Vec<User>, num_pages: u64) -> Self {
        Self { users, num_pages }
    }
}

impl FindResultTrait for UserFindResponse {
    type Entity = User;
    fn get_result(&self) -> impl Iterator<Item = Self::Entity> {
        self.users.clone().into_iter()
    }
    fn get_page_count(&self) -> u64 {
        self.num_pages
    }
}

impl<T> From<&T> for UserFindResponse
where
    T: FindResultTrait<Entity = User>,
{
    fn from(value: &T) -> Self {
        UserFindResponse::new(value.get_result().collect(), value.get_page_count())
    }
}

#[derive(Debug, Error)]
pub enum UserFindRequestError {
    #[error("offset value {offset} cannot be less than 1, please choose higher value")]
    PerPageOffsetTooLow { offset: u64 },
    #[error("per_page value {per_page} cannot be less than 1, please choose higher value")]
    PerPageValueTooLow { per_page: u16 },
    #[error("per_page value {per_page} is too high, please choose lower value")]
    PerPageValueTooHigh { per_page: u16 },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<UserFindRequestError> for UserError {
    fn from(val: UserFindRequestError) -> Self {
        match val {
            UserFindRequestError::PerPageOffsetTooLow { offset } => {
                UserError::PerPageOffsetTooLow { offset }
            }
            UserFindRequestError::PerPageValueTooHigh { per_page } => {
                UserError::PerPageValueTooHigh { per_page }
            }
            UserFindRequestError::PerPageValueTooLow { per_page } => {
                UserError::PerPageValueTooLow { per_page }
            }
            UserFindRequestError::Unknown(e) => UserError::Unknown(e),
        }
    }
}

impl FindOptionTrait for UserFindRequest {
    type QueryFilter = UserFindRequestFilter;
    fn get_query(&self) -> Self::QueryFilter {
        self.filters.clone()
    }
    fn get_order_by(&self) -> String {
        self.order_by.clone()
    }
    fn get_offset(&self) -> u64 {
        self.offset
    }
    fn get_limit(&self) -> u16 {
        self.per_page
    }
}

impl<T> From<&T> for UserFindRequest
where
    T: FindOptionTrait<QueryFilter = UserFindRequestFilter>,
{
    fn from(value: &T) -> Self {
        Self::new(
            value.get_query().borrow(),
            value.get_order_by().borrow(),
            value.get_limit().borrow(),
            value.get_offset().borrow(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::business::user::dtos::{
        user_find_request::UserFindRequestFilter, UserFindRequestError,
    };

    use super::UserFindRequest;

    #[test]
    fn test_user_find_request_default_ok() {
        let user_find_request = UserFindRequest::default();
        assert_eq!(user_find_request.per_page, 25);
        assert_eq!(user_find_request.offset, 1);
        assert_eq!(user_find_request.filters, UserFindRequestFilter::default());
    }

    #[test]
    fn validate_per_page_ok() {
        let per_page = 5;
        let validated_per_page = UserFindRequest::validate_per_page(&per_page).unwrap();
        assert_eq!(&per_page, validated_per_page);
    }

    #[test]
    fn validate_per_page_ko_too_low() {
        let per_page = 0;
        let error = UserFindRequestError::PerPageValueTooLow { per_page };
        let validated_per_page = UserFindRequest::validate_per_page(&per_page).err().unwrap();
        assert_eq!(error.to_string(), validated_per_page.to_string());
    }

    #[test]
    fn validate_per_page_ko_too_high() {
        let per_page = 1001;
        let error = UserFindRequestError::PerPageValueTooHigh { per_page };
        let validated_per_page = UserFindRequest::validate_per_page(&per_page).err().unwrap();
        assert_eq!(error.to_string(), validated_per_page.to_string());
    }

    #[test]
    fn validate_offset_ko_too_low() {
        let offset = 0;
        let error = UserFindRequestError::PerPageOffsetTooLow { offset };
        let validated_offset = UserFindRequest::validate_offset(&offset).err().unwrap();
        assert_eq!(error.to_string(), validated_offset.to_string());
    }

    #[test]
    fn test_user_find_request_new_ok() {
        let user_find_request_filter = UserFindRequestFilter {
            id: Some(Uuid::new_v4()),
            ..Default::default()
        };
        let user_find_request = UserFindRequest::new(&user_find_request_filter, "", &10, &1);
        assert!(user_find_request.is_ok());
    }

    #[test]
    fn test_user_find_request_set_filter_ok() {
        let user_find_request_filter = UserFindRequestFilter {
            id: Some(Uuid::new_v4()),
            ..Default::default()
        };
        let mut user_find_request = UserFindRequest::default();
        user_find_request.set_filters(&user_find_request_filter);
        assert_eq!(user_find_request.filters, user_find_request_filter);
    }

    #[test]
    fn test_user_find_request_set_offset_ok() {
        let mut user_find_request = UserFindRequest::default();
        user_find_request.set_offset(&2);
        assert_eq!(user_find_request.offset, 2);
    }

    #[test]
    fn test_user_find_request_set_per_page_ok() {
        let mut user_find_request = UserFindRequest::default();
        user_find_request.set_per_page(&2);
        assert_eq!(user_find_request.per_page, 2);
    }
}
