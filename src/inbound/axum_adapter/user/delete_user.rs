use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    business::user::{
        dtos::{UserFindRequest, UserFindResponse},
        model::user::UserError,
        User, UserDeleteRequest, UserRepositoryTrait, UserServiceTrait,
    },
    inbound::axum_adapter::setup::AppState,
};

use super::user_error::AxumUserError;

#[utoipa::path(
    delete,
    tag = "User",
    path = "/user/{user_id}",
    params(
        (
            "user_id" = UserDeleteRequest,
            Path,
            description = "User identifier"
        )
    ),
    responses(
        (
            status = 204,
            description = "User deletion succeed"
        ),
        (
            status = 401,
            description = "Authentication required"
        ),
        (
            status = 403,
            description = "Operation forbidden"
        )
    ),
)]
pub async fn delete_user<
    U: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
>(
    State(app_state): State<AppState<U>>,
    Path(user_delete_request): Path<UserDeleteRequest>,
) -> impl IntoResponse {
    app_state
        .user_service
        .delete_user(&user_delete_request)
        .await
        .map(|_| (StatusCode::NO_CONTENT, ()).into_response())
        .unwrap_or_else(|e| AxumUserError(e).into_response())
}
