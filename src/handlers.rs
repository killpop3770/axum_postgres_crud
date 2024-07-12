use axum::extract::{Path, State};
use axum::Json;
use sqlx::PgPool;

use crate::errors::TaskApiError;
use crate::models::{CreateTask, CreateTaskRequest, Task, UpdateTaskRequest};
use crate::responses::TaskApiResponse;

pub async fn get_task(
    State(pg_pool): State<PgPool>,
) -> Result<TaskApiResponse, TaskApiError> {
    let rows = sqlx::query_as!(Task, "SELECT * FROM tasks ORDER BY task_id")
        .fetch_all(&pg_pool)
        .await
        .map_err(|_| TaskApiError::InternalServerError)?;

    Ok(TaskApiResponse::Data(rows))
}

pub async fn create_task(
    State(pg_pool): State<PgPool>,
    Json(task): Json<CreateTaskRequest>,
) -> Result<TaskApiResponse, TaskApiError> {
    let row = sqlx::query_as!(
        CreateTask,
        "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
        task.name,
        task.priority,
    )
        .fetch_one(&pg_pool)
        .await
        .map_err(|_| TaskApiError::InternalServerError)?;

    Ok(TaskApiResponse::Created(row))
}

pub async fn update_task(
    State(pg_pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(update_task_request): Json<UpdateTaskRequest>,
) -> Result<TaskApiResponse, TaskApiError> {
    let mut query = String::from("UPDATE tasks SET task_id = $1");

    let mut variable_number = 2;

    if update_task_request.name.is_some() {
        query.push_str(format!(", name = ${}", variable_number).as_str());
        variable_number += 1;
    }

    if update_task_request.priority.is_some() {
        query.push_str(format!(", priority = ${}", variable_number).as_str());
    }

    query.push_str(" WHERE task_id = $1");

    let mut main_query = sqlx::query(&query).bind(task_id);

    if update_task_request.name.is_some() {
        main_query = main_query.bind(update_task_request.name);
    }

    if update_task_request.priority.is_some() {
        main_query = main_query.bind(update_task_request.priority);
    }

    main_query
        .execute(&pg_pool)
        .await
        .map_err(|_| TaskApiError::InternalServerError)?;

    Ok(TaskApiResponse::Ok)
}

pub async fn delete_task(
    State(pg_pool): State<PgPool>,
    Path(task_id): Path<i32>,
) -> Result<TaskApiResponse, TaskApiError> {
    sqlx::query!(
        "DELETE FROM tasks WHERE task_id = $1",
        task_id
    )
        .execute(&pg_pool)
        .await
        .map_err(|_| TaskApiError::InternalServerError)?;

    Ok(TaskApiResponse::Ok)
}
