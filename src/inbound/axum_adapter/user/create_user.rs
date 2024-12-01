use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;

use crate::{
    business::user::{
        dtos::{UserFindRequest, UserFindResponse},
        model::user::UserError,
        User, UserAddRequest, UserRepositoryTrait, UserServiceTrait,
    },
    inbound::axum_adapter::setup::AppState,
};

use super::user_error::AxumUserError;

#[utoipa::path(
    post,
    tag = "User",
    path = "/user",
    request_body = UserAddRequest,
    responses(
        (
            status = 201,
            description = "User creation succeed"
        ),
        (
            status = 400,
            description = "Data sent not correct"
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
pub async fn create_user<
    U: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
>(
    State(app_state): State<AppState<U>>,
    Json(user_add_request): Json<UserAddRequest>,
) -> impl IntoResponse {
    app_state
        .user_service
        .create_user(&user_add_request)
        .await
        .map(|u| (StatusCode::CREATED, Json(u)).into_response())
        .unwrap_or_else(|e| AxumUserError(e).into_response())
}
