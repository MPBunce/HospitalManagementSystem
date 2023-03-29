use actix_web::{App, HttpServer};
use actix_web::web::Data;
// use std::path::Path;
// use r2d2_sqlite::SqliteConnectionManager;
// use r2d2::Pool;

mod api_routes;
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


// struct Physician {
//     employee_id: i32,
//     name: String,
//     position: String,
//     ssn: i32,
// }

// fn main() -> Result<(), rusqlite::Error> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
//     println!("\n");
//     println!("{}", database_url);
//     println!("\n");
//     let conn = Connection::open(&database_url).unwrap();
//     //let conn = Connection::open("/Users/m/Desktop/Programming/Projects/HospitalManagmentSystem/Database/Hospital.db").unwrap();

//     let mut stmt = conn.prepare("SELECT * FROM Physician")?;
//     let physician_iter = stmt.query_map([], |row| {
//         Ok(Physician {
//             employee_id: row.get(0)?,
//             name: row.get(1)?,
//             position: row.get(2)?,
//             ssn: row.get(3)?,
//         })
//     })?;

//     // Iterate over the results and print them

//     for physician in physician_iter {
//         match physician {
//             Ok(physician) => {
//                 println!("ID: {}\nName: {}\nPosition: {}\nSSN: {}\n", physician.employee_id, physician.name, physician.position, physician.ssn);
//             }
//             Err(error) => {
//                 println!("Error retrieving physician: {}", error);
//             }
//         }
//     }

//     Ok(())

// }
