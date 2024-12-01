pub mod create_user;
pub mod delete_user;
pub mod find_one_user;
pub mod find_user;
pub mod update_user;
pub mod user_error;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use create_user::create_user;
use delete_user::delete_user;
use find_one_user::find_one_user;
use find_user::find_user;
use update_user::update_user;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::business::user::{
    dtos::{UserFindRequest, UserFindResponse},
    model::user::UserError,
    User, UserRepositoryTrait,
};

use super::setup::AppState;

pub async fn init_route<
    U: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
>() -> Router<AppState<U>> {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(find_user))
        .route("/:user_id", put(update_user))
        .route("/:user_id", get(find_one_user))
        .route("/:user_id", delete(delete_user))
}

pub fn api_docs() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(paths(
        crate::inbound::axum_adapter::user::create_user::create_user,
        crate::inbound::axum_adapter::user::update_user::update_user,
        crate::inbound::axum_adapter::user::delete_user::delete_user,
        crate::inbound::axum_adapter::user::find_one_user::find_one_user,
        crate::inbound::axum_adapter::user::find_user::find_user
    ))]
    struct ApiDocs;
    ApiDocs::openapi()
}
