use axum::extract::{Path, State};
use axum::Json;

use crate::AppState;
use crate::errors::TaskApiError;
use crate::models::{CreateTask, CreateTaskRequest, UpdateTaskRequest};
use crate::responses::TaskApiResponse;
use crate::task_repository::{delete, get_all, insert, update};

pub async fn get_task(
    State(state): State<AppState>,
) -> Result<TaskApiResponse, TaskApiError> {
    let tasks = get_all(&state.db_pool).map_err(TaskApiError::DBError)?;
    Ok(TaskApiResponse::Data(tasks))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(task): Json<CreateTaskRequest>,
) -> Result<TaskApiResponse, TaskApiError> {
    let task_id = insert(&state.db_pool, task).map_err(TaskApiError::DBError)?;
    Ok(TaskApiResponse::Created(CreateTask { task_id }))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
    Json(task_request): Json<UpdateTaskRequest>,
) -> Result<TaskApiResponse, TaskApiError> {
    update(&state.db_pool, task_id, task_request).map_err(TaskApiError::DBError)?;
    Ok(TaskApiResponse::Ok)
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
) -> Result<TaskApiResponse, TaskApiError> {
    delete(&state.db_pool, task_id).map_err(TaskApiError::DBError)?;
    Ok(TaskApiResponse::Ok)
}

pub async fn handler_404() -> TaskApiError {
    TaskApiError::NotFound
}