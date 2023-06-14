use actix_web::{
    get, post, delete,
    web::{Data, Json, Path},
    Scope, Error, HttpRequest,
};

use crate::{models::{GetListResponse, CreateListRequest, List}, context::{Context}, controllers::users::is_authorised};

#[post("")]
pub async fn create_list(
    headers: HttpRequest,
    request: Json<CreateListRequest>,
    context: Data<Context>,
) -> Result<Json<GetListResponse>, Error> {
    let session = is_authorised(headers, &context).await?;

    if session.user_id != request.user_id {
        return Err(actix_web::error::ErrorUnauthorized("session token is not valid"));
    }

    let mut conn = context.db_pool.acquire().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let list = List::insert_from_request(request.into_inner(), &mut conn).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or(actix_web::error::ErrorConflict("list with such name already exists".to_owned()))?;

    Ok(Json(GetListResponse::from_list(list, vec![])))
}

pub fn scope() -> Scope {
    actix_web::web::scope("lists")
        .service(create_list)
}