use axum::{
    http::{HeaderMap, StatusCode},
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::entity::tasks;
use crate::utils::tasks::{RequestTask, ResponseTask};

pub async fn create_task(
    Extension(db): Extension<DatabaseConnection>,
    headers: HeaderMap,
    Json(task): Json<RequestTask>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let user = headers.get("userid").unwrap().to_str().unwrap();

    let userid = user.parse::<i32>().unwrap();

    let task = tasks::ActiveModel {
        title: Set(task.title),
        description: Set(task.description),
        difficulty: Set(task.difficulty),
        user_id: Set(userid),
        ..Default::default()
    };

    let created_task = task
        .save(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseTask::new(
        created_task.id.unwrap(),
        created_task.title.unwrap(),
        created_task.description.unwrap(),
        created_task.difficulty.unwrap(),
        created_task.finished.unwrap(),
    )))
}
