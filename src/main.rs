use std::env;

use axum::Router;
use tokio::net::TcpListener;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod controllers;
mod services;

struct AppState {
    db_pool: Pool<Postgres>,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {

    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    {
        Ok(db_pool) => {
            println!("✅Connection to database succesfully");
            db_pool
        },
        Err(e) => {
            println!("🔴 Connection to database error: {}", e);
            std::process::exit(1);
        }
    };

    let state = AppState {
        db_pool,
    };
    
    let routes = Router::new()
    .nest("/api", controllers::auth::login());

    let tcp_lister = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to connect to 127.0.0.1:8080");

    axum::serve(tcp_lister, routes)
    .await
    .expect("Failed to start server");
}