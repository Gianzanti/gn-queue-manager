use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[repr(i32)]
pub enum Customer {
    Resound = 0,
    Beltone = 1,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Visitor {
    pub id: i32,
    pub customer: Customer,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub lgpd: bool,
    pub image_rights: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub observations: String,
    pub confirm_visit: bool,
    pub state: String,
    pub job: String,
}

#[derive(Deserialize, Debug)]
pub struct VisitorSubmission {
    pub customer: Customer,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub lgpd: bool,
    pub image_rights: bool,
    pub state: String,
    pub job: String,
}

#[derive(Deserialize, Debug)]
pub struct VisitorUpdateRecord {
    pub name: Option<String>,
    pub customer: Option<Customer>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub lgpd: Option<bool>,
    pub image_rights: Option<bool>,
    pub observations: Option<String>,
    pub confirm_visit: Option<bool>,
    pub state: Option<String>,
    pub job: Option<String>,
}
