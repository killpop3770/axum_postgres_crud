use axum::Router;
use axum::routing::{get, patch, post};

use crate::AppState;
use crate::handlers::{create_task, delete_task, get_info_after_login_handler, get_task, handler_404, login_handler, update_task};

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/v1", get(default))
        .route("/v1/login", post(login_handler))
        .route("/v1/login_info", get(get_info_after_login_handler))
        .nest("/v1/tasks", tasks(state.clone()))
        .fallback(handler_404)
}

async fn default() -> &'static str {
    "Hello, async world!"
}

// fn login() -> Router{
//     Router::new()
//         .route("/login", post())
//         .route("/login_info", get())
// }

fn tasks(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_task).post(create_task))
        .route("/:tasks_id", patch(update_task).delete(delete_task))
        .with_state(state)
}