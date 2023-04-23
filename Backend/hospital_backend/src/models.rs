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