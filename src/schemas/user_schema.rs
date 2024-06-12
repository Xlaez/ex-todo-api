use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions{
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
   pub  id: Uuid,
    pub username: String,
    pub email: String,
    pub email_verified: Option<bool>,
    pub img: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct  UpdateUserSchema{
    pub email: Option<String>,
    pub password: Option<String>,
    pub img: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyEmailSchema {
    pub email: String,
    pub otp: String,
}
