use crate::entity::tasks::{ActiveModel, Entity};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::utils::tasks::{RequestTask, ResponseTask};

pub async fn update_task(
    Extension(db): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(task): Json<RequestTask>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let current_task = Entity::find_by_id(task_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut current_model: ActiveModel = current_task.unwrap().into();

    current_model.title = Set(task.title.clone());
    current_model.description = Set(task.description.clone());
    current_model.difficulty = Set(task.difficulty.clone());

    let updated_task = current_model
        .update(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseTask::new(
        updated_task.id,
        updated_task.title,
        updated_task.description,
        updated_task.difficulty,
        updated_task.finished,
    )))
}
