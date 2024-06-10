mod routes;
mod handlers;
mod schemas;
mod models;
mod utils;

use std::sync::Arc;


use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};
use dotenv::dotenv;
use routes::create_router;
use sqlx::{pool, postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;


pub struct AppState{
    db: Pool<Postgres>,
}


#[tokio::main]
async fn main(){
    dotenv().ok();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE URL must have a value");
    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        {
            Ok(pool) => {
                println!("Postgre Database connection successful");
                pool
            }
            Err(err) => {
                eprintln!("Postgre Database connection failed: {:?}", err);
                std::process::exit(1);
            }
        };

        let cors = CorsLayer::new()
            // .allow_origin("*".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT, Method::PATCH])
            .allow_credentials(true)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

        let server = create_router(Arc::new(AppState {db: pool.clone()})).layer(cors);
        
        println!("🚀 Server started successfully");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        axum::serve(listener, server).await.unwrap();
}
