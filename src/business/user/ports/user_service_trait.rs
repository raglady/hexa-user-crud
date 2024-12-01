use std::future::Future;

use uuid::Uuid;

use crate::business::user::{
    dtos::{UserFindRequest, UserFindResponse},
    model::user::UserError,
    User, UserAddRequest, UserDeleteRequest, UserUpdateRequest,
};

pub trait UserServiceTrait: Sync + Send + Clone + 'static {
    fn create_user(
        &self,
        req: &UserAddRequest,
    ) -> impl Future<Output = Result<User, UserError>> + Send;

    fn update_user(
        &self,
        user_id: &Uuid,
        req: &UserUpdateRequest,
    ) -> impl Future<Output = Result<User, UserError>> + Send;

    fn find_one_user(&self, user_id: &Uuid)
        -> impl Future<Output = Result<User, UserError>> + Send;

    fn find_user(
        &self,
        req: &UserFindRequest,
    ) -> impl Future<Output = Result<UserFindResponse, UserError>> + Send;

    fn delete_user(
        &self,
        req: &UserDeleteRequest,
    ) -> impl Future<Output = Result<(), UserError>> + Send;
}
