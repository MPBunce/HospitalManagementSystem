
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Physician {
    pub employee_id: i32,
    pub name: String,
    pub position: String,
    pub ssn: String,
}
