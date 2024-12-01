use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

use crate::business::user::{
    dtos::{UserFindRequest, UserFindResponse},
    model::user::UserError,
    service::user_service::UserService,
    User, UserRepositoryTrait,
};

use super::user;

#[derive(Debug, Clone)]
pub struct AppState<
    U: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
> {
    pub user_service: Arc<UserService<U>>,
}

pub async fn setup<
    U: UserRepositoryTrait<
        Id = Uuid,
        Entity = User,
        Error = UserError,
        FindOptions = UserFindRequest,
        FindResult = UserFindResponse,
    >,
>(
    app_state: AppState<U>,
) -> Router<()> {
    #[derive(OpenApi)]
    #[openapi(info(title = "Api documentation"))]
    struct ApiDocs;
    let mut api_docs = ApiDocs::openapi();
    api_docs.merge(user::api_docs());
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_docs))
        .nest("/user", user::init_route().await)
        .with_state(app_state)
}
