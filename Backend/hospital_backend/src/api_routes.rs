use actix_web::{web,  HttpResponse,  Responder};

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hw").route(web::get().to(hello_world)));
}