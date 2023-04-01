use actix_web::{App, HttpServer};
use actix_web::web::Data;

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
        App::new()
            .app_data(app_state.clone())
            .configure(api_routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
