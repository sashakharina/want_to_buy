use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Scope,
};

pub fn scope() -> Scope {
    actix_web::web::scope("lists")
        //.service(get_own)
}