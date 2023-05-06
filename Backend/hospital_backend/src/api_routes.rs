use actix_web::{ web, HttpResponse, Responder };
use crate::db::AppState;
use rusqlite::ToSql;

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

pub async fn departments_list(db: web::Data<AppState>) -> impl Responder {
    let conn = db.db_pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Department").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Department{
            department_id: row.get(0)?,
            name: row.get(1)?,
            head: row.get(2)?,
 
        })
    }).unwrap();

    let departments: Vec<Department> = rows.map(|r| r.unwrap()).collect();

    HttpResponse::Ok().json(departments)
}

pub async fn procedures_list(db: web::Data<AppState>) -> impl Responder {
    let conn = db.db_pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Procedure").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Procedure{
            code: row.get(0)?,
            name: row.get(1)?,
            cost: row.get(2)?,
 
        })
    }).unwrap();

    let procedures: Vec<Procedure> = rows.map(|r| r.unwrap()).collect();

    HttpResponse::Ok().json(procedures)
}

pub async fn room_list(db: web::Data<AppState>) -> impl Responder {
    let conn = db.db_pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Room").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Room{
            room_number: row.get(0)?,
            room_type: row.get(1)?,
            room_blockfloor: row.get(2)?,
            room_blockcode: row.get(3)?,
            room_unavailable: row.get(4)?,
        })
    }).unwrap();

    let rooms: Vec<Room> = rows.map(|r| r.unwrap()).collect();

    HttpResponse::Ok().json(rooms)

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

pub async fn get_medication(db: web::Data<AppState>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let statement_string = format!("SELECT * FROM Medication");

    let mut stmt = conn.prepare( &statement_string ).unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Medication {
            code: row.get(0)?,
            name: row.get(1)?,
            brand: row.get(2)?,
            description: row.get(3)?,
            in_stock: row.get(4)?,

        })
    });

    match rows {
        Ok(iter) => {
            let mut medications = Vec::new();
            for result in iter {
                match result {
                    Ok(row) => medications.push(row),
                    Err(e) => {
                        eprintln!("Error retrieving row: {:?}", e);
                        return HttpResponse::NotFound().body("No Medications");
                    }
                }
            }
            HttpResponse::Ok().json(medications)
        }
        Err(e) => {
            eprintln!("Error querying database: {:?}", e);
            HttpResponse::InternalServerError().body("Error")
        }
    }

}

pub async fn get_appointments(db: web::Data<AppState>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let statement_string = format!("SELECT *
                                    FROM Appointment
                                        LEFT JOIN Patient ON Patient.SSN = Appointment.Patient
                                        LEFT JOIN Prescribes ON Prescribes.Appointment = Appointment.AppointmentID"
    );

    let mut stmt = conn.prepare( &statement_string ).unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Appointments {
            appointment_id: row.get(0).ok(),
            patient: row.get(1).ok(),
            prep_nurse: row.get(2).ok(),
            physician: row.get(3).ok(),
            start_time: row.get(4).ok(),
            end_time: row.get(5).ok(),
            exam_room: row.get(6).ok(),
            patient_ssn: row.get(7).ok(),
            patient_name: row.get(8).ok(),
            patient_address: row.get(9).ok(),
            patient_phone: row.get(10).ok(),
            patient_insurance_id: row.get(11).ok(),
            patient_primary_care_physician: row.get(12).ok(),
            prescribes_physician: row.get(13).ok(),
            prescribes_patient_id: row.get(14).ok(),
            prescribes_medication: row.get(15).ok(),
            prescribes_date: row.get(16).ok(),
            prescribes_appointment: row.get(17).ok(),
            prescribes_dose: row.get(18).ok(),
        })
    });

    match rows {
        Ok(iter) => {
            let mut appointments = Vec::new();
            for result in iter {
                match result {
                    Ok(row) => appointments.push(row),
                    Err(e) => {
                        eprintln!("Error retrieving row: {:?}", e);
                        return HttpResponse::NotFound().body("No Appointments?");
                    }
                }
            }
            HttpResponse::Ok().json(appointments)
        }
        Err(e) => {
            eprintln!("Error querying database: {:?}", e);
            HttpResponse::InternalServerError().body("Error")
        }
    }

}

pub async fn get_surgeries(db: web::Data<AppState>) -> impl Responder {


    let conn = db.db_pool.get().unwrap();
    let statement_string = format!("SELECT *
                                    FROM Undergoes
                                        LEFT JOIN Patient ON Patient.SSN = Undergoes.Patient
                                        LEFT JOIN Procedure ON Procedure.Code = Undergoes.Procedure
                                        LEFT JOIN Stay on Stay.StayID = Undergoes.Stay
                                        LEFT JOIN Room on Room.Number = Stay.Room"
    );

    let mut stmt = conn.prepare( &statement_string ).unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Surgery {
            patient: row.get(0).ok(),
            procedure: row.get(1).ok(),
            stay: row.get(2).ok(),
            physician: row.get(3).ok(),
            assisting_nurse: row.get(4).ok(),

            patient_ssn: row.get(5).ok(),
            patient_name: row.get(6).ok(),
            patient_address: row.get(7).ok(),
            patient_phone: row.get(8).ok(),
            patient_insurance_id: row.get(9).ok(),
            patient_primary_care_physician: row.get(10).ok(),

            procedure_code: row.get(11).ok(),
            procedure_name: row.get(12).ok(),
            procedure_cost: row.get(13).ok(),

            stay_id: row.get(14).ok(),
            stay_patient_id: row.get(15).ok(),
            stay_room: row.get(16).ok(),
            stay_start: row.get(17).ok(),
            stay_end: row.get(18).ok(),
            
            room_number: row.get(19).ok(),
            room_type: row.get(20).ok(),
            room_blockfloor: row.get(21).ok(),
            room_blockcode: row.get(22).ok(),
            room_unavailable: row.get(23).ok(),
        })
    });

    match rows {
        Ok(iter) => {
            let mut surgeries = Vec::new();
            for result in iter {
                match result {
                    Ok(row) => surgeries.push(row),
                    Err(e) => {
                        eprintln!("Error retrieving row: {:?}", e);
                        return HttpResponse::NotFound().body("No Surgeries?");
                    }
                }
            }
            HttpResponse::Ok().json(surgeries)
        }
        Err(e) => {
            eprintln!("Error querying database: {:?}", e);
            HttpResponse::InternalServerError().body("Error")
        }
    }

}

pub async fn create_medication(db: web::Data<AppState>, medication: web::Json<Medication>) -> impl Responder {
    let conn = db.db_pool.get().unwrap();

    let params: &[&dyn ToSql] = &[
        &medication.code, 
        &medication.name,
        &medication.brand, 
        &medication.description, 
        &medication.in_stock
    ];


    let result = conn.execute(
        "INSERT INTO Medication (Code, Name, Brand, Description, In_Stock) VALUES ($1, $2, $3, $4, $5)", params,
    );

    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Created().finish()
            } else {
                HttpResponse::InternalServerError().body("Failed to create medication")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create medication: {}", e)),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/").route(web::get().to(hello_world)))
        .service(web::resource("/api/get_all_physicians").route(web::get().to(get_all_physicians)))
        .service(web::resource("/api/get_physician/{id}").route(web::get().to(get_physician)))
        .service(web::resource("/api/get_departments/{id}").route(web::get().to(get_departments)))
        .service(web::resource("/api/get_procedures/{id}").route(web::get().to(get_procedures)))
        .service(web::resource("/api/get_nurses").route(web::get().to(get_nurses)))    
        .service(web::resource("/api/get_medication").route(web::get().to(get_medication)))      
        .service(web::resource("/api/get_appointments").route(web::get().to(get_appointments))) 
        .service(web::resource("/api/get_surgeries").route(web::get().to(get_surgeries))) 
        .service(web::resource("/api/departments_list").route(web::get().to(departments_list))) 
        .service(web::resource("/api/room_list").route(web::get().to(room_list))) 
        .service(web::resource("/api/procedures_list").route(web::get().to(procedures_list)))
        .service(web::resource("/api/create_medication").route(web::get().to(create_medication))) 
    ;

}