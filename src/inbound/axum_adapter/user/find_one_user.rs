use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    business::user::{
        dtos::{UserFindRequest, UserFindResponse},
        model::user::UserError,
        User, UserRepositoryTrait, UserServiceTrait,
    },
    inbound::axum_adapter::setup::AppState,
};

use super::user_error::AxumUserError;

#[utoipa::path(
    get,
    tag = "User",
    path = "/user/{user_id}",
    params(
        (
            "user_id" = Uuid,
            Path,
            description = "User identifier"
        )
    ),
    responses(
        (
            status = 200,
            description = "Users list succeed"
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
pub async fn find_one_user<
    U: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
>(
    State(app_state): State<AppState<U>>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    app_state
        .user_service
        .find_one_user(&user_id)
        .await
        .map(|u| (StatusCode::OK, Json(u)).into_response())
        .unwrap_or_else(|e| AxumUserError(e).into_response())
}
