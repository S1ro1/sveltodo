use axum::response::Response;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::utils::get_secret;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserClaims {
    sub: String,
    id: i32,
    exp: usize,
}

pub async fn jwt_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let jar = CookieJar::from_headers(request.headers());

    let auth_header = match jar.get("authorization") {
        Some(jwt_token) => jwt_token.to_string(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !auth_header.starts_with("authorization=") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt_token = auth_header.trim_start_matches("authorization=");

    let token_header =
        jsonwebtoken::decode_header(&jwt_token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_claims = jsonwebtoken::decode::<UserClaims>(
        &jwt_token,
        &DecodingKey::from_secret(get_secret().as_ref()),
        &Validation::new(token_header.alg),
    );

    let user_claims = match user_claims {
        Ok(user_claims) => user_claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let id = user_claims.claims.id;

    request
        .headers_mut()
        .insert("userid", id.to_string().parse().unwrap());

    Ok(next.run(request).await)
}
