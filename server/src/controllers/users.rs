use actix_web::{
    get, post, delete,
    web::{Data, Json, Path, Header},
    Scope, Error, http::header,
};
use pwhash::bcrypt;

use crate::models::{User, CreateUserRequest, Session, UserPublic, LoginRequest, LoginResponse};
use crate::context::Context;

#[post("")]
pub async fn create_user (
    request: Json<CreateUserRequest>,
    context: Data<Context>,
) -> Result<Json<LoginResponse>, Error> {
    let mut request = request.into_inner();
    request.password = bcrypt::hash(request.password).unwrap();
    let mut conn = context.db_pool.acquire().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let user = User::insert_from_request(request, &mut conn).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or(actix_web::error::ErrorConflict("user with such email already exists".to_owned()))?;
    let session_id = Session::insert_session(user.id, &mut conn).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?.session_id;
    Ok(Json(LoginResponse { user_info: (user), session_id: (session_id), lists: vec![] }))
}

#[post("login")]
pub async fn login (
    request: Json<LoginRequest>,
    context: Data<Context>,
) -> Result<Json<LoginResponse>, Error> {
    let mut conn = context.db_pool.acquire().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let user = User::get_by_email(&request.email, &mut conn).await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    .ok_or(actix_web::error::ErrorNotFound("user with such email doesn't exist".to_owned()))?;

    if pwhash::bcrypt::verify(&request.password, &user.password) {
        return Err(actix_web::error::ErrorForbidden("incorrect password".to_owned()));
    }

    let session_id = Session::insert_session(user.id, &mut conn).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?.session_id;

    Ok(Json(LoginResponse { user_info: (user), session_id: (session_id), lists: vec![] }))
}

// #[delete("logout")]
// pub async fn logout (
//     headers: Header<header::>,
//     context: Data<Context>,
// ) -> Result<HttpResponse, Error> {
//    // Ok(HttpResponse::)
// }

#[get("public/{email}")]
pub async fn get_user_public_info (
    email: Path<String>,
    context: Data<Context>,
) -> Result<Json<UserPublic>, Error> {
    let mut conn = context.db_pool.acquire().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let user = User::get_by_email(&email, &mut conn).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or(actix_web::error::ErrorNotFound("user with such email doesn't exist".to_owned()))?;
    Ok(Json(user.into()))
}

pub fn scope() -> Scope {
    actix_web::web::scope("users")
        .service(create_user)
        .service(get_user_public_info)
        .service(login)
        //.service(factory)
}