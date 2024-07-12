use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schemas::tasks)]
pub struct TaskRecord {
    pub(crate) task_id: i32,
    pub(crate) name: String,
    pub(crate) priority: Option<i32>,
}

#[derive(Deserialize, Insertable, Selectable)]
#[diesel(table_name = crate::schemas::tasks)]
pub struct CreateTaskRequest {
    pub(crate) name: String,
    pub(crate) priority: Option<i32>,
}

#[derive(Serialize, Selectable)]
#[diesel(table_name = crate::schemas::tasks)]
pub struct CreateTask {
    pub(crate) task_id: i32,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = crate::schemas::tasks)]
pub struct UpdateTaskRequest {
    pub(crate) name: Option<String>,
    pub(crate) priority: Option<i32>,
}