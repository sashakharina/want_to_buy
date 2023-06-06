use actix_web::web::ServiceConfig;

mod users;
mod contacts;
mod purchases;
mod lists;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        actix_web::web::scope("/api/v1")
            .service(users::scope())
            .service(lists::scope())
            .service(purchases::scope())
            .service(contacts::scope()),
    );
}
