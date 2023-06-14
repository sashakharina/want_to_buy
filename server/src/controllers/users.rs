use std::cmp::Ordering;

use actix_web::{
    get, post, delete,
    web::{Data, Json, Path},
    Scope, Error, http::header, HttpRequest, HttpResponse, Responder,
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

    if !pwhash::bcrypt::verify(&request.password, &user.password) {
        return Err(actix_web::error::ErrorForbidden("incorrect password".to_owned()));
    }

    let session_id = Session::insert_session(user.id, &mut conn).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?.session_id;

    Ok(Json(LoginResponse { user_info: (user), session_id: (session_id), lists: vec![] }))
}

#[delete("logout")]
pub async fn logout (
    req: HttpRequest,
    context: Data<Context>,
) -> impl Responder {
    let auth_header = req
    .headers()
    .get(header::AUTHORIZATION);

    if auth_header.is_none() {
        return HttpResponse::Unauthorized();
    }
    let session_id = auth_header.unwrap().to_str().unwrap();

    let mut conn = context.db_pool.acquire().await.unwrap();
    match Session::drop_session(session_id, &mut conn).await{
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError() 
    }
}

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
        .service(logout)
}

pub async fn is_authorised (
    request: HttpRequest,
    context: &Context,
) -> Result<Session, Error> {
    let auth_header = request
    .headers()
    .get(header::AUTHORIZATION);

    if auth_header.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("session token missing"));
    }

    let session_id = auth_header.unwrap().to_str().unwrap();

    let mut conn = context.db_pool.acquire().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let session = Session::get_session(session_id, &mut conn).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or(actix_web::error::ErrorUnauthorized("session token is not valid"))?;
    if chrono::offset::Utc::now()
        .signed_duration_since(session.created_at)
        .cmp(&chrono::Duration::minutes(context.config.expiration_time)) == Ordering::Greater {

        Session::drop_session(session_id, &mut conn).await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        return Err(actix_web::error::ErrorUnauthorized("session token expired"));
    }
    Ok(session)
}