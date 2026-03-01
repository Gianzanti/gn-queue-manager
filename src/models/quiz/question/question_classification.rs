use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Classification {
    pub id: Uuid,
    pub question_id: Vec<Uuid>,
    pub option_text: String,
    pub weights: serde_json::Value,
    // pub created_at: chrono::NaiveDateTime,
    // pub updated_at: chrono::NaiveDateTime,
    // pub is_deleted: bool,
}
