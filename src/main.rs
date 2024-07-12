use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use tokio::net::TcpListener;

use crate::routes::app_router;

mod models;
mod handlers;
mod responses;
mod errors;
mod routes;
mod schemas;

#[derive(Clone)]
struct AppState {
    db_pool: Pool<ConnectionManager<PgConnection>>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Can not find .env file!");

    let server_address = std::env::var("SERVER_URL").unwrap_or("127.0.0.1:3000".to_string());
    let database_address = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file!");

    let manager = ConnectionManager::<PgConnection>::new(&database_address);
    let db_pool = Pool::builder().build(manager).expect("Failed to create pool!");

    let app_state = AppState { db_pool };

    let listener = TcpListener::bind(&server_address)
        .await
        .expect("Could not create TCP listener!");

    println!("Listening on : {}", listener.local_addr().unwrap());

    let app = app_router(app_state.clone()).with_state(app_state);

    axum::serve(listener, app).await.expect("Error to run application!");
}