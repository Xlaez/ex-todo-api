use std::sync::Arc;

use axum::{body::Body, extract::{Request, State}, http::{header, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Json};
use serde_json::json;

use crate::{handlers::get_user_by_email, utils::decode_jwt, AppState};

pub async fn authorize_user(State(data): State<Arc<AppState>>, mut req: Request,  next: Next) -> Result<Response<Body>, impl IntoResponse>{
    let auth_header = req.headers_mut().get(header::AUTHORIZATION);
    let error_response = json!({"status": "fail", "message": format!("Provide a valid auth header")});
    
    let auth_header = match auth_header {

        Some(header) => header.to_str()
            .map_err(|_| (StatusCode::FORBIDDEN, Json(error_response.clone())))?,

        None => return Err((StatusCode::FORBIDDEN, Json(error_response))),
    };

    let mut header = auth_header.split_whitespace();
    let (bearer, token) = (header.next(), header.next());

    if bearer != Some("Bearer") || token.is_none() {
        return Err((StatusCode::FORBIDDEN, Json(error_response)));
    }

    let error_response = serde_json::json!({"status": "fail", "message": "Unable to decode JWT auth token"});

    let token_data = match decode_jwt(token.unwrap().to_string()){
        Ok(data) => data,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, Json(error_response))),
    };

    let current_user = match  get_user_by_email(&token_data.claims.email, &data.db).await {
        Some(user) => user,
        None => {
            let error_response = json!({"status": "fail", "message": "Not authorized"});
            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}