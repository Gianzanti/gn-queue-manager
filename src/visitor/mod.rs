use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum Customer {
    Resound,
    Beltone,
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
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct VisitorSubmission {
    pub customer: Customer,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub lgpd: bool,
    pub image_rights: bool,
}

#[derive(Deserialize, Debug)]
pub struct VisitorUpdateRecord {
    pub name: Option<String>,
    pub customer: Option<Customer>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub lgpd: Option<bool>,
    pub image_rights: Option<bool>,
}
