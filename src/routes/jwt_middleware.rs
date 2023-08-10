use axum::response::Response;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

const SECRET: &str = "siro";

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
    let auth_header = request
        .headers()
        .get("authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_header = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt_token = auth_header.trim_start_matches("Bearer ");

    let token_header =
        jsonwebtoken::decode_header(&jwt_token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    dbg!(&token_header);

    let user_claims = jsonwebtoken::decode::<UserClaims>(
        jwt_token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::new(token_header.alg),
    );

    dbg!(&user_claims);

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
