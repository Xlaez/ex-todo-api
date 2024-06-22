
use axum::{
    response::IntoResponse,
    Json,
};



pub async fn health_checker_handler() -> impl IntoResponse{
    const MESSAGE: &str = "A Simple Todo API Built With Rust and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}