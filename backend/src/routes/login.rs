use crate::entity::users::{self, Model};

use crate::utils::users::{RequestUser, ResponseUser};
use crate::utils::utils::get_secret;

use argon2::{Argon2, PasswordVerifier};
use axum::{http::StatusCode, Extension, Json};

use axum::response::{IntoResponse, Response};
use chrono::{Days, Utc};
use cookie::Cookie;
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserClaims {
    sub: String,
    id: i32,
    exp: usize,
}

impl UserClaims {
    pub fn new(sub: String, id: i32, exp: usize) -> Self {
        Self { sub, id, exp }
    }
}

fn validate_login(request_pw: String, u: Model) -> Response {
    let copy = u.clone();
    let parsed_hash = argon2::PasswordHash::new(&u.password).unwrap();

    let verified = Argon2::default().verify_password(request_pw.as_bytes(), &parsed_hash);

    match verified {
        Ok(_) => (),
        Err(_) => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let exp = Utc::now()
        .checked_add_days(Days::new(1))
        .unwrap()
        .timestamp() as usize;

    let claims = UserClaims::new(u.name, u.id, exp);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_ref()),
    );

    let token = match token {
        Ok(token) => token,
        Err(_) => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let response_user = ResponseUser::new(copy.id, copy.name);

    let cookie = Cookie::build("authorization", token)
        .http_only(true)
        .path("/")
        .same_site(cookie::SameSite::None)
        .finish();

    (
        StatusCode::OK,
        [("Set-Cookie", cookie.to_string())],
        Json(response_user),
    )
        .into_response()
}

#[axum::debug_handler]
pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(login): Json<RequestUser>,
) -> Response {
    let user = users::Entity::find()
        .filter(users::Column::Name.eq(login.username))
        .one(&db)
        .await;

    let u = match user {
        Ok(u) => u,
        Err(_) => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    match u {
        Some(u) => validate_login(login.password, u),
        None => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    }
}
