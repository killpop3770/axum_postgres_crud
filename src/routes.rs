use axum::Router;
use axum::routing::{get, patch};

use crate::AppState;
use crate::handlers::{create_task, delete_task, get_task, handler_404, update_task};

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(default))
        .nest("/v1/tasks", tasks(state.clone()))
        .fallback(handler_404)
}

async fn default() -> &'static str {
    "Hello, async world!"
}

fn tasks(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_task).post(create_task))
        .route("/:tasks_id", patch(update_task).delete(delete_task))
        .with_state(state)
}