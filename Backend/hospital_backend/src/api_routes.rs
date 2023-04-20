use actix_web::{web, HttpResponse, Responder, HttpResponseBuilder, http::StatusCode};
use crate::db::AppState;

//Models
use crate::models::*;

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub async fn get_all_physicians(db: web::Data<AppState>) -> impl Responder {

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

pub async fn get_nurses(db: web::Data<AppState>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Nurse").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Nurse {
            employee_id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            registered: row.get(3)?,
            ssn: row.get(4)?,
        })
    }).unwrap();

    let nurses: Vec<Nurse> = rows.map(|r| r.unwrap()).collect();

    HttpResponse::Ok().json(nurses)

}


pub async fn get_physician(db: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let statement_string = "SELECT * FROM Physician WHERE EmployeeID = ".to_owned() + &path.to_string();
    let mut stmt = conn.prepare( &statement_string ).unwrap();

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

pub async fn get_departments(db: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let statement_string = format!("Select DISTINCT Affiliated_With.Physician, Affiliated_With.Department, Affiliated_With.PrimaryAffiliation, 
                                        Department.DepartmentID, Department.Name, Department.Head
                                    FROM Physician
                                        LEFT JOIN Affiliated_With ON Physician.EmployeeID = Affiliated_With.Physician 
                                        LEFT JOIN Department ON Affiliated_With.Department = Department.DepartmentID
                                    WHERE
                                        Physician.EmployeeID = {}", path);

    let mut stmt = conn.prepare( &statement_string ).unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Departments {
            employee_id: row.get(0)?,
            department: row.get(1)?,
            primary_affiliation: row.get(2)?,
            department_id: row.get(3)?,
            name: row.get(4)?,
            head: row.get(5)?,
        })
    }).unwrap();

    let departments: Vec<Departments> = rows.map(|r| r.unwrap()).collect();

    HttpResponse::Ok().json(departments)


}

pub async fn get_procedures(db: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let statement_string = format!("SELECT DISTINCT 
                                        Trained_In.Physician, Trained_In.Treatment, Trained_In.CertificationDate, Trained_In.CertificationExpires, 
                                        Procedure.Code, Procedure.Name, Procedure.Cost
                                    FROM Physician
                                        LEFT JOIN Trained_In ON Physician.EmployeeID = Trained_In.Physician
                                        LEFT JOIN Procedure ON Trained_In.Treatment = Procedure.Code
                                    WHERE
                                        Physician.EmployeeID = {}", path);


    let mut stmt = conn.prepare( &statement_string ).unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Procedures {
            employee_id: row.get(0)?,
            treatment: row.get(1)?,
            certification_date: row.get(2)?,
            certification_exp: row.get(3)?,
            code: row.get(4)?,
            name: row.get(5)?,
            cost: row.get(6)?
        })
    });

    match rows {
        Ok(iter) => {
            let mut procedures = Vec::new();
            for result in iter {
                match result {
                    Ok(row) => procedures.push(row),
                    Err(e) => {
                        eprintln!("Error retrieving row: {:?}", e);
                        return HttpResponse::NotFound().body("No Prod");
                    }
                }
            }
            HttpResponse::Ok().json(procedures)
        }
        Err(e) => {
            eprintln!("Error querying database: {:?}", e);
            HttpResponse::InternalServerError().body("No Prod")
        }
    }

}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/").route(web::get().to(hello_world)))
        .service(web::resource("/api/get_all_physicians").route(web::get().to(get_all_physicians)))
        .service(web::resource("/api/get_physician/{id}").route(web::get().to(get_physician)))
        .service(web::resource("/api/get_departments/{id}").route(web::get().to(get_departments)))
        .service(web::resource("/api/get_procedures/{id}").route(web::get().to(get_procedures)))
        .service(web::resource("/api/get_nurses").route(web::get().to(get_nurses)))        
    ;

}