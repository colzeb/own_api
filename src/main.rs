use actix_web::{web::{self, Path}, App, HttpResponse, HttpServer, get, guard, middleware};
mod services;
mod model;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}


#[get("pet/{petId}")]
async fn pet_by_id(petId: Path<u32>) -> HttpResponse {
    HttpResponse::Ok().body("body")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let app = App::new();
    
    // let user_info= app.service(
    //     web::scope("/user")
    //     .service(services::user::info::get_user_name)
    // );
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::NormalizePath::trim())
        .service(
            web::scope("/user")
            .service(index)
            .guard(guard::Header("content-type", "application/json"))
            .service(services::user::info::get_user_name)
        )
        .service(
            web::scope("/user")
            .service(index)
            .guard(guard::Header("content-type", "application/txt"))
            .service(services::user::info::get_user_name)
        )
        .service(index)
        .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}