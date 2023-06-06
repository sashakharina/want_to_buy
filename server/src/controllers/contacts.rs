use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Scope,
};

pub fn scope() -> Scope {
    actix_web::web::scope("contacts")
        //.service(get_own)
}