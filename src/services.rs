use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use sqlx::PgPool;
use crate::dao::{CreateTaskRequest, CreateTaskRow, TaskRow, UpdateTaskRequest};

pub async fn get_task(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(TaskRow, "SELECT * FROM tasks ORDER BY task_id")
        .fetch_all(&pg_pool)
        .await
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": error.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": rows}).to_string(),
    ))
}

pub async fn create_task(
    State(pg_pool): State<PgPool>,
    Json(task): Json<CreateTaskRequest>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        CreateTaskRow,
        "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
        task.name,
        task.priority,
    )
        .fetch_one(&pg_pool)
        .await
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": error.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": row}).to_string(),
    ))
}

pub async fn update_task(
    State(pg_pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(update_task_request): Json<UpdateTaskRequest>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
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
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": error.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true}).to_string()
    ))
}

pub async fn delete_task(
    State(pg_pool): State<PgPool>,
    Path(task_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!(
        "DELETE FROM tasks WHERE task_id = $1",
        task_id
    )
        .execute(&pg_pool)
        .await
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": error.to_string()}).to_string()
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true}).to_string()
    ))
}
