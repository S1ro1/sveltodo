use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

use crate::entity::tasks::Entity as Task;

#[axum::debug_handler]
pub async fn delete_task(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let task = Task::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let task = task.unwrap();

    let _ = task
        .delete(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
