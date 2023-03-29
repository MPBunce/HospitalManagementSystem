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
    pub procedure_code: i32,
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
    pub appointment_id: i32,
    pub patient_id: i32,
    pub prep_nurse_id: i32,
    pub physician_id: i32,
    pub start_time: String,
    pub end_time: String,
    pub examination_room: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Medication {
    pub code: i32,
    pub name: String,
    pub brand: String,
    pub description: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Prescribes {
    pub physician_id: i32,
    pub patient_id: i32,
    pub medication_code: i32,
    pub date: String,
    pub appointment_id: i32,
    pub dose: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Block {
    pub floor: i32,
    pub code: i32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Room {
    pub room_number: i32,
    pub room_type: String,
    pub block_floor: i32,
    pub block_code: i32,
    pub unavailable: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OnCall {
    pub nurse_id: i32,
    pub block_floor: i32,
    pub block_code: i32,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Stay {
    pub stay_id: i32,
    pub patient_id: i32,
    pub room_number: i32,
    pub stay_start: String,
    pub stay_end: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Undergoes {
    pub patient_id: i32,
    pub procedure_code: i32,
    pub stay_id: i32,
    pub undergoes_date: String,
    pub attending_physician_id: i32,
    pub attending_nurse_id: i32
}