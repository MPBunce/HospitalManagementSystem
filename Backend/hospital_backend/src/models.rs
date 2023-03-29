#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Physician {
    pub employee_id: i32,
    pub name: String,
    pub position: String,
    pub ssn: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Department {
    pub department_id: i32,
    pub name: String,
    pub employee_id: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AffiliatedWith {
    pub employee_id: i32,
    pub department_id: String,
    pub primary_affiliation: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Procedure {
    pub code: i32,
    pub name: String,
    pub cost: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrainedIn {
    pub employee_id: i32,
    pub treatment_code: i32,
    pub certification_date: String,
    pub certification_expires: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Patient {
    pub snn: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub insurance_id: i32,
    pub pcp: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Nurse {
    pub employee_id: i32,
    pub name: String,
    pub position: String,
    pub registered: bool,
    pub ssn: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Appointment {

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Medication {

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Prescribes {

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Block {

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Room {

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OnCall {

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Stay {

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Undergoes {

}