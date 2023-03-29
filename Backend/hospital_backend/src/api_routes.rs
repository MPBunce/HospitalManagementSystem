use actix_web::{web, HttpResponse, Responder};
use crate::db::AppState;

//models
use crate::models::*;

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub async fn get_docs(db: web::Data<AppState>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Physician").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Physician {
            employee_id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            ssn: row.get(3)?,
        })
    }).unwrap();

    let physicians: Vec<Physician> = rows.map(|r| r.unwrap()).collect();

    HttpResponse::Ok().json(physicians)

}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hw").route(web::get().to(hello_world)))
        .service(web::resource("/get_docs").route(web::get().to(get_docs))
    );
}