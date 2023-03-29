use actix_web::{App, HttpServer};
use actix_web::web::Data;

mod api_routes;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_url = "/Users/m/Desktop/Programming/Projects/HospitalManagmentSystem/Database/Hospital.db";
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
