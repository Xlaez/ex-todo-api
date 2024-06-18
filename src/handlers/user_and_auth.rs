use std::{fmt::format, io::Write, sync::Arc};

use axum::{body, extract::{Multipart, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use cloudinary::upload::result::UploadResult;
use serde_json::json;
use sqlx::PgPool;
use tempfile::NamedTempFile;

use crate::{
    models::{OtpModel, UserModel}, schemas::{CreateUserSchema, LoginSchema, OtpSchema, UserResponse, VerifyEmailSchema}, utils::{check_otp_expiry, encode_jwt, generate_otp, hash_password, send_otp_mail, upload_to_cloud, verify_password}, AppState
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
        Ok(_) => {
            let user_response = json!({"status": "success", "data": json!({
                "message": "Account created successfully"
            })});

            let otp_code = generate_otp(5);

            let otp_body = OtpSchema{
                email: body.email.clone().to_string(),
                otp: otp_code.clone().to_string(),
            };

            match otp_creator_service(State(data.clone()), Json((otp_body))).await{
                Ok(_) => {
                    send_otp_mail(&body.email, &otp_code, &body.username).await;

                     return Ok((StatusCode::CREATED, Json(user_response)));
                },
                Err(e)=>{
                return Err(e);
                }
            }
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

                if user.email_verified == Some(false){
                    let error_response = serde_json::json!({"status": "fail", "message": "Please verify your email first"});
                    return Err((StatusCode::BAD_REQUEST, Json(error_response)));
                }

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

    pub async fn verify_email(State(data): State<Arc<AppState>>,
    Json(body): Json<VerifyEmailSchema>,) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{

        let otp_doc = otp_fetch_service(&data.db, &body.otp).await;
        
            let error_response = serde_json::json!({"status": "fail", "message": "Invalid or expired otp"});


       match otp_doc{
         Some(otp_doc) => {
            if otp_doc.otp.len() == 0 || otp_doc.email != body.email.to_string(){
              return  Err((StatusCode::BAD_REQUEST, Json(error_response)));                
            }

           if let Some(created_at) = otp_doc.created_at{
             match check_otp_expiry(&created_at.to_rfc3339()){
                Ok(_) => {

                     let user = get_user_by_email(&body.email, &data.db).await; 

                     match user {
                        Some(_) => {

                            let now = chrono::Utc::now();

                            let query_result = sqlx::query_as!(UserModel, "UPDATE users SET email_verified=$1, updated_at=$2 WHERE email=$3 RETURNING *",
                                Some(true),
                                now,
                                &body.email,
                            ).fetch_one(&data.db).await;

                            match query_result {
                                Ok(_) => {
                                    let response = serde_json::json!({"status": "success", "data": serde_json::json!({
                                    "status": "success"
                                })});

                            Ok(Json(response))
                                }
                                Err(err) => {
                                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status": "fail", "message": format!("{:?}", err)})),))
                                }
                            }
                            
                        }
                        None =>{
                             let error_response = serde_json::json!({"status": "fail", "message": "Cannot verify this email"});
                                Err((StatusCode::BAD_REQUEST, Json(error_response)))
                        }
                     }
                },
                Err(_err) => Err((StatusCode::BAD_REQUEST, Json(error_response)))
            }
           }else {
              return  Err((StatusCode::BAD_REQUEST, Json(error_response)));                
           }
        }
        None => {
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
       }
    }

    pub async fn otp_fetch_service( pool: &PgPool, otp: &str) ->Option<OtpModel> {
        match sqlx::query_as!(
            OtpModel,
            "SELECT * FROM otps WHERE otp = $1", 
            otp.to_string(),
        ).fetch_one(pool).await

        {
            Ok(otp_docs) => Some(otp_docs),Err(_) => None,
        }
    }

    pub async fn otp_creator_service( State(data): State<Arc<AppState>>, Json(otp_body): Json<OtpSchema>)  -> Result<String, (StatusCode, Json<serde_json::Value>)> {
     let query_result = sqlx::query_as!(
         OtpModel,
         "INSERT INTO otps (email,otp) VALUES ($1,$2) RETURNING *",
         otp_body.email,
         otp_body.otp,
    ).fetch_one(&data.db).await;

    match  query_result {
        Ok(_) => 
            Ok(otp_body.otp),
        Err(_) => {
             let error_response = serde_json::json!({"status": "fail", "message": "Cannot create OTP"});
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
    }

    pub async fn upload_img(State(data): State<Arc<AppState>>, Extension(current_user): Extension<UserModel> ,mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
        while let Some(mut field) = multipart.next_field().await.unwrap() {
            let filename = if let Some(filename) = field.file_name(){
                filename.to_string()
            }else{
                continue;
            };

            let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file for upload");

            while let Some(chunk) = field.chunk().await.unwrap(){
                temp_file.write_all(&chunk).expect("Failed to write chunk to tempfile");
            }

            let temp_path = temp_file.into_temp_path();

          match upload_to_cloud(&filename, &temp_path).await {
            Ok(result) => {
                println!("Upload successful");

                let result_str = match result {
                    UploadResult::Success(success_data) => {
                        success_data.secure_url
                    },
                    UploadResult::Error(_) => {
                        let error_response = serde_json::json!({"status": "fail", "message": "cannot upload image to cloud"});
                        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
                    }
                };

                let now = chrono::Utc::now();

                let query_result = sqlx::query_as!(UserModel, "UPDATE users SET img=$1, updated_at=$2 WHERE email=$3 RETURNING *",
                    result_str,
                    now,
                    &current_user.email,
                ).fetch_one(&data.db).await;

                match query_result {
                    Ok(_) => {
                        let response = serde_json::json!({"status": "success", "data": serde_json::json!({
                            "status": "success"
                        })});

                        return Ok(response.to_string());
                    }
                    Err(err) => {
                        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status": "fail", "message": format!("{:?}", err)}))));
                    }
                }
            }
                Err(e) => {
                     let error_response = serde_json::json!({"status": "fail", "message": "Cannot upload image"});
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
                }
            }
        }

        let error_response = serde_json::json!({"status": "fail", "message": "No fields to process"});
    Err((StatusCode::BAD_REQUEST, Json(error_response)))
    }