use axum::{
    extract::{Query, State},
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
    path = "/user",
    params(
        UserFindRequest
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
pub async fn find_user<
    U: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
>(
    State(app_state): State<AppState<U>>,
    Query(user_find_request): Query<UserFindRequest>,
) -> impl IntoResponse {
    app_state
        .user_service
        .find_user(&user_find_request)
        .await
        .map(|u| (StatusCode::OK, Json(u)).into_response())
        .unwrap_or_else(|e| AxumUserError(e).into_response())
}
