use std::sync::Arc;

use axum::{
    routing::{get, post, delete, put},
    Router,
};

use crate::{
    handlers::{create_user_handler, health_checker_handler, login_handler, verify_email},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router{
    Router::new()
        .route("/api/health_checker", get(health_checker_handler))
        .route("/api/user/register", post(create_user_handler))
        .route("/api/user/login", post(login_handler))
        .route("/api/user/verify_email", post(verify_email))
        .with_state(app_state)
}