use actix_web::{
    get, guard, middleware,
    web::{self, Path},
    App, HttpResponse, HttpServer,
};
mod model;
mod services;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[get("pet/{petId}")]
async fn pet_by_id(petId: Path<u32>) -> HttpResponse {
    let pet = model::pet::Pet {
        name: "Dog".to_owned(),
        id: 1,
        status: model::status::Status::available,
        category: model::category::Category { id: 1, name: "Dog".to_owned() },
        tags: vec![model::tag::Tag {id: 1, name: "Animal".to_owned()}],
        photoUrls: vec!["http://google.com".to_owned()],
    };
    HttpResponse::Ok().json(pet)
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
                    .service(services::user::info::get_user_name),
            )
            .service(
                web::scope("/user")
                    .service(index)
                    .guard(guard::Header("content-type", "application/txt"))
                    .service(services::user::info::get_user_name),
            )
            .service(index)
            .service(index)
            .service(pet_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
