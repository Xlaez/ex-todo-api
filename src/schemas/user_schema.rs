use serde::{Deserialize, Serialize};

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
pub struct  UpdateUserSchema{
    pub email: Option<String>,
    pub password: Option<String>,
    pub img: Option<String>,
}
