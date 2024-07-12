use diesel::table;

table! {
    tasks(task_id) {
        task_id -> Int4,
        name -> Varchar,
        priority -> Nullable<Int4>,
    }
}