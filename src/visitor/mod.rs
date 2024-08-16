use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct Visitor {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub lgpd: bool,
    pub image_rights: bool,
}

#[derive(Deserialize)]
pub struct VisitorSubmission {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub lgpd: bool,
    pub image_rights: bool,
}

#[derive(Deserialize)]
pub struct VisitorUpdateRecord {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub lgpd: Option<bool>,
    pub image_rights: Option<bool>,
}
