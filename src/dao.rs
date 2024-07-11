use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TaskRow {
    pub(crate) task_id: i32,
    pub(crate) name: String,
    pub(crate) priority: Option<i32>,
}


#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub(crate) name: String,
    pub(crate) priority: Option<i32>,
}

#[derive(Serialize)]
pub struct CreateTaskRow {
    pub(crate) task_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub(crate) name: Option<String>,
    pub(crate) priority: Option<i32>,
}