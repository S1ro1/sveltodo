use crate::entity::users::{self, Model};
use argon2::{Argon2, PasswordVerifier};
use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    id: i32,
    name: String,
}

fn validate_login(request_pw: String, u: Model) -> Result<Json<ResponseUser>, StatusCode> {
    let parsed_hash = argon2::PasswordHash::new(&u.password).unwrap();

    let result = Argon2::default().verify_password(request_pw.as_bytes(), &parsed_hash);

    match result {
        Ok(_) => Ok(Json(ResponseUser {
            id: u.id,
            name: u.name,
        })),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(login): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let user = users::Entity::find()
        .filter(users::Column::Name.eq(login.username))
        .one(&db)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    match user {
        Some(u) => {
            return validate_login(login.password, u);
        }
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
}
