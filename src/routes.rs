use crate::{
    handlers::{
        add_list_handler, create_user_handler, get_user_by_username, get_users_lists_handler,
        health_checker_handler, login_handler, update_password, upload_img, verify_email,
    },
    middlewares::authorize_user,
    AppState,
};
use axum::{
    middleware::from_fn_with_state,
    routing::{get, patch, post},
    Router,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health_checker", get(health_checker_handler))
        .route("/api/user/register", post(create_user_handler))
        .route("/api/user/login", post(login_handler))
        .route("/api/user/verify_email", post(verify_email))
        .route(
            "/api/user/update/img",
            patch(upload_img).layer(from_fn_with_state(app_state.clone(), authorize_user)),
        )
        .route(
            "/api/user/update/password",
            patch(update_password).layer(from_fn_with_state(app_state.clone(), authorize_user)),
        )
        .route(
            "/api/user/:username",
            get(get_user_by_username).layer(from_fn_with_state(app_state.clone(), authorize_user)),
        )
        .route(
            "/api/lists/list",
            post(add_list_handler).layer(from_fn_with_state(app_state.clone(), authorize_user)),
        )
        .route(
            "/api/lists",
            get(get_users_lists_handler)
                .layer(from_fn_with_state(app_state.clone(), authorize_user)),
        )
        .with_state(app_state)
}
