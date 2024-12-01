use std::sync::Arc;

use i_tantana::business::user::service::user_service::UserService;
use i_tantana::inbound::axum_adapter::setup::{setup, AppState};
use i_tantana::outbound::in_memory_repository_adapter::in_memory_user_repository::InMemoryUserRepository;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let user_repository = InMemoryUserRepository::new();
    let user_service = Arc::new(UserService::new(user_repository));
    let app_state = AppState { user_service };
    let router = setup(app_state).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
