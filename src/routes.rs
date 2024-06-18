use std::sync::Arc;

use axum::{
    middleware::{from_fn, from_fn_with_state}, routing::{delete, get, patch, post, put}, Router
};

use crate::{
    handlers::{create_user_handler, health_checker_handler, login_handler, update_password, upload_img, verify_email}, middlewares::authorize_user, AppState
};

pub fn create_router(app_state: Arc<AppState>) -> Router{
    Router::new()
        .route("/api/health_checker", get(health_checker_handler))
        .route("/api/user/register", post(create_user_handler))
        .route("/api/user/login", post(login_handler))
        .route("/api/user/verify_email", post(verify_email))
        .route("/api/user/update/img", patch(upload_img).layer(from_fn_with_state(app_state.clone(),authorize_user)))
        .route("/api/user/update/password", patch(update_password).layer(from_fn_with_state(app_state.clone(),authorize_user)))
        .with_state(app_state)
}