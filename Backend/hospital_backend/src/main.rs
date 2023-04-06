use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::web::Data;

use actix_cors::Cors;

//Importing my crates
mod api_routes;
mod models;
mod db;
// adam so cool and handsome yep yep yep yep yep yep yep ye 
// this mac is warm to the touch wtf, kinda nice lowk
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_url = "/home/bunk/Programming/HospitalManagmentSystem/Database/hospital.db";
    let pool = db::create_pool(db_url.to_string());
    let app_state = Data::new(db::AppState { db_pool: pool });

    HttpServer::new(move || {

        // let cors = Cors::default()
        //     .allowed_origin("*")
        //     .allowed_origin_fn(|origin, _req_head| {
        //         origin.as_bytes().ends_with(b".rust-lang.org")
        //     })
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .configure(api_routes::init)
            .wrap(Cors::default().allowed_origin("https://localhost:3000").send_wildcard())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
