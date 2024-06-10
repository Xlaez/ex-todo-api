use std::{fmt::format, sync::Arc};

use axum::{body, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    models::UserModel, schemas::{CreateUserSchema, LoginSchema, UserResponse}, utils::{encode_jwt, hash_password, verify_password}, AppState
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
                "message": "Account created successfully"
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

pub async fn get_user_by_email(email: &str, pool: &PgPool) -> Option<UserModel> {
     match sqlx::query_as!(UserModel, "SELECT * FROM users WHERE email = $1", email.to_string()).fetch_one(pool).await   

    {
        Ok(user) => Some(user), Err(_) => None,
    }
}

pub async fn login_handler( State(data): State<Arc<AppState>>,
    Json(body): Json<LoginSchema>,) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
     let user = get_user_by_email(&body.email, &data.db).await; 

    match user {
        Some(user) => {
            match verify_password(&user.password, &body.password) {
                Ok(valid) => {
                    if !valid {
                        let error_response = serde_json::json!({"status": "fail", "message": "Incorrect credentials"});
                        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
                    }

                   let token = match encode_jwt(&body.email) {
                    Ok(token) => token,
                    Err(_) => {
                        let error_response = serde_json::json!({"status": "fail", "message": "Unable to generate auth token"});
                        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
                    }
                };

                let user_response: UserResponse = user.into();

                    let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                        "user": user_response,
                        "token": token,
                    })});

                    Ok(Json(user_response))
                }
                Err(_) => {
                    let error_response = serde_json::json!({"status": "fail", "message": "Password verification failed"});
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
                }
            }
        }
       None => {
            let error_response = serde_json::json!({"status": "fail", "message": "Incorrect credentials"});
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
    }