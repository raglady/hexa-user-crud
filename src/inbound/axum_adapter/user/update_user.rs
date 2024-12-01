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
        User, UserRepositoryTrait, UserServiceTrait, UserUpdateRequest,
    },
    inbound::axum_adapter::setup::AppState,
};

use super::user_error::AxumUserError;

#[utoipa::path(
    put,
    tag = "User",
    path = "/user/{user_id}",
    params(
        (
            "user_id" = Uuid,
            Path,
            description = "User identifier"
        )
    ),
    request_body = UserUpdateRequest,
    responses(
        (
            status = 200,
            description = "User update succeed"
        ),
        (
            status = 400,
            description = "Sent data not correct"
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
pub async fn update_user<
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
    Json(user_update_request): Json<UserUpdateRequest>,
) -> impl IntoResponse {
    app_state
        .user_service
        .update_user(&user_id, &user_update_request)
        .await
        .map(|u| (StatusCode::OK, Json(u)).into_response())
        .unwrap_or_else(|e| AxumUserError(e).into_response())
}
