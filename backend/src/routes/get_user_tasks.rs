use crate::entity::tasks;
use crate::entity::users;
use crate::utils::tasks::ResponseTask;
use axum::{
    http::{HeaderMap, StatusCode},
    Extension, Json,
};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

pub async fn get_user_tasks(
    Extension(db): Extension<DatabaseConnection>,
    headers: HeaderMap,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let user = headers.get("userid").unwrap().to_str().unwrap();

    let user_id = user.parse::<i32>().unwrap();

    let user = users::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap();

    let tasks = user
        .find_related(tasks::Entity)
        .filter(tasks::Column::Removed.eq(false))
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        tasks
            .into_iter()
            .map(|task| {
                ResponseTask::new(
                    task.id,
                    task.title,
                    task.description,
                    task.difficulty,
                    task.finished,
                )
            })
            .collect::<Vec<ResponseTask>>(),
    ))
}
