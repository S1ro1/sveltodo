use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::entity::tasks::{ActiveModel, Entity};

#[derive(serde::Deserialize)]
pub struct StatusUpdate {
    finished: bool,
}

pub async fn update_task_status(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Json(status): Json<StatusUpdate>,
) -> Result<(), StatusCode> {
    let task = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut current_task: ActiveModel = task.unwrap().into();

    current_task.finished = Set(status.finished);

    current_task
        .update(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
