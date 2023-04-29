////////////////////////        Doctor Stuff        //////////////////////////////////
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Physician {
    pub employee_id: i32,
    pub name: String,
    pub position: String,
    pub ssn: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Departments {
    pub employee_id: i32,
    pub department: i32,
    pub primary_affiliation: bool,
    pub department_id: i32,
    pub name: String,
    pub head: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Procedures {
    pub employee_id: i32,
    pub treatment: i32,
    pub certification_date: String,
    pub certification_exp: String,
    pub code: i32,
    pub name: String, 
    pub cost: f32,
}

//////////////////////          Nurses          /////////////////////////////////
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Nurse {
    pub employee_id: i32,
    pub name: String,
    pub position: String,
    pub registered: bool,
    pub ssn: i32,
}

//////////////////////////      MEDICATION     //////////////////////////////
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Medication {
    pub code: i32,
    pub name: String,
    pub brand: String,
    pub description: String,
    pub in_stock: bool,
}

///////////////////////         Appointment     ///////////////////////////////////// 
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Appointments {
    pub appointment_id: Option<i32>,
    pub patient: Option<i32>,
    pub prep_nurse: Option<i32>,
    pub physician: Option<i32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub exam_room: Option<String>,
    pub patient_ssn: Option<i32>,
    pub patient_name: Option<String>,
    pub patient_address: Option<String>,
    pub patient_phone: Option<String>,
    pub patient_insurance_id: Option<i32>,
    pub patient_primary_care_physician: Option<i32>,
    pub prescribes_physician: Option<i32>,
    pub prescribes_patient_id: Option<i32>,
    pub prescribes_medication: Option<i32>,
    pub prescribes_date: Option<String>,
    pub prescribes_appointment: Option<i32>,
    pub prescribes_dose: Option<String>,
}


////////////////////////        Surgery         \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Surgery {
    pub patient: Option<i32>,
    pub procedure: Option<i32>,
    pub stay: Option<i32>,
    pub physician: Option<i32>,
    pub assisting_nurse: Option<i32>,
    pub patient_ssn: Option<i32>,
    pub patient_name: Option<String>,
    pub patient_address: Option<String>,
    pub patient_phone: Option<String>,
    pub patient_insurance_id: Option<i32>,
    pub patient_primary_care_physician: Option<i32>,
    pub procedure_code: Option<i32>,
    pub procedure_name: Option<i32>,
    pub procedure_cost: Option<f32>,
    pub stay_id: Option<i32>,
    pub stay_patient_id: Option<i32>,
    pub stay_room: Option<i32>,
    pub stay_start: Option<String>,
    pub stay_end: Option<String>,
    pub room_number: Option<i32>,
    pub room_type: Option<String>,
    pub room_blockfloor: Option<i32>,
    pub room_blockcode: Option<i32>,
    pub room_unacailable: Option<bool>,
}