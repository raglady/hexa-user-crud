use std::future::Future;

use uuid::Uuid;

use crate::business::user::{
    dtos::{UserFindRequest, UserFindResponse},
    model::user::UserError,
    User, UserAddRequest, UserDeleteRequest, UserRepositoryTrait, UserServiceTrait,
    UserUpdateRequest,
};

#[derive(Debug, Clone)]
pub struct UserService<R>
where
    R: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
{
    user_repository: R,
}

impl<R> UserService<R>
where
    R: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

impl<R> UserServiceTrait for UserService<R>
where
    R: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
{
    fn create_user(
        &self,
        req: &UserAddRequest,
    ) -> impl Future<Output = Result<User, UserError>> + Send {
        Box::pin(async { self.user_repository.save(&req.into()).await })
    }

    fn update_user(
        &self,
        user_id: &uuid::Uuid,
        req: &UserUpdateRequest,
    ) -> impl Future<Output = Result<User, UserError>> + Send {
        Box::pin(async { self.user_repository.update(user_id, &req.into()).await })
    }

    fn find_one_user(
        &self,
        user_id: &Uuid,
    ) -> impl Future<Output = Result<User, UserError>> + Send {
        Box::pin(async { self.user_repository.find_by_id(user_id).await })
    }

    fn find_user(
        &self,
        req: &UserFindRequest,
    ) -> impl Future<Output = Result<UserFindResponse, UserError>> + Send {
        Box::pin(async { self.user_repository.find_all(req).await })
    }

    fn delete_user(
        &self,
        req: &UserDeleteRequest,
    ) -> impl Future<Output = Result<(), UserError>> + Send {
        Box::pin(async { self.user_repository.delete(req.get_user_id()).await })
    }
}
