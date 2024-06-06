use std::sync::Arc;

use axum::{
    routing::{get, post, delete, put},
    Router,
};

use crate::{
    handlers::{health_checker_handler},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router{
    Router::new()
        .route("/api/health_checker", get(health_checker_handler))
        .with_state(app_state)
}