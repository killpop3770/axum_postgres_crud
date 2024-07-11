use axum::{
    Router,
    routing::{get, patch},
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use crate::services::{create_task, delete_task, get_task, update_task};

mod dao;
mod services;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Can not find .env file!");

    let server_address = std::env::var("SERVER_URL").unwrap_or("127.0.0.1:3000".to_string());
    let database_address = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file!");

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_address)
        .await
        .expect("Can not connect to database!");

    let listener = TcpListener::bind(&server_address)
        .await
        .expect("Could not create TCP listener!");

    println!("Listening on : {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/", get(|| async { "Hello, async world!" }))
        .route("/tasks", get(get_task).post(create_task))
        .route("/tasks/:tasks_id", patch(update_task).delete(delete_task))
        .with_state(db_pool);

    axum::serve(listener, app).await.expect("Error to run application!");
}