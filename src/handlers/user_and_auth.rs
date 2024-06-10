use std::{fmt::format, sync::Arc};

use axum::{body, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    models::UserModel, schemas::CreateUserSchema, utils::hash_password, AppState
};

pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{

    let hashed_password = hash_password(&body.password.to_string()).map_err(|_e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status": "fail", "message": format!("Cannot hash password")}))
        )
    })?;

    let query_result = sqlx::query_as!(
        UserModel,
        "INSERT INTO users (username,email,password,email_verified) VALUES ($1,$2,$3,$4) RETURNING *",
        body.username.to_string(),
        body.email.to_string(),
        hashed_password,  
        Some(false),
    ).fetch_one(&data.db).await;

    match query_result {
        Ok(user) => {
            let user_response = json!({"status": "success", "data": json!({
                "user": user
            })});

            return Ok((StatusCode::CREATED, Json(user_response)));
        }
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint"){
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "User with this email already exists",
                });

                return Err((StatusCode::BAD_REQUEST, Json(error_response)));
            }

            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status": "fail", "message": format!("{:?}", e)}))));
        }
    }
}