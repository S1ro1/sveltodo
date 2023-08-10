use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::extract::{Extension, Json};
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::entity::users;

#[derive(Deserialize)]
pub struct RequestRegister {
    username: String,
    password: String,
    repeated_password: String,
}

#[derive(Serialize)]
pub struct ResponseRegister {
    id: i32,
    username: String,
}

fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let phash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    dbg!(&phash);
    dbg!(phash.len());
    phash
}

pub async fn register(
    Extension(db): Extension<DatabaseConnection>,
    Json(user): Json<RequestRegister>,
) -> Result<Json<ResponseRegister>, StatusCode> {
    if user.password != user.repeated_password {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = users::ActiveModel {
        name: Set(user.username.clone()),
        password: Set(hash_password(user.password)),
        ..Default::default()
    };

    let model = user
        .save(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseRegister {
        username: model.name.unwrap(),
        id: model.id.unwrap(),
    }))
}
