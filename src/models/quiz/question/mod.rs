use uuid::Uuid;

pub mod question_classification;
pub mod question_mchoice;

#[derive(Debug, PartialEq, Clone)]
pub enum Answers {
    MultipleChoice(Vec<question_mchoice::MultipleChoice>),
    Classification(Vec<question_classification::Classification>),
}

#[derive(Clone)]
pub struct Question {
    pub id: Uuid,
    pub quiz_id: Vec<Uuid>,
    pub question_text: String,
    pub answers: Answers,
    // pub created_at: chrono::NaiveDateTime,
    // pub updated_at: chrono::NaiveDateTime,
    // pub is_deleted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_creation() {
        // Get the current time in UTC
        // let now_utc = Utc::now();

        let quiz_id = Uuid::now_v7();
        let question_id = Uuid::now_v7();

        let answer_01 = question_mchoice::MultipleChoice {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: "Experiência + visual + tecnologia".to_string(),
            weights: serde_json::json!({"resound": 1}),
        };

        let answer_02 = question_mchoice::MultipleChoice {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: "Bem-estar + acolhimento _ conexão".to_string(),
            weights: serde_json::json!({"beltone": 1}),
        };

        let answer_03 = question_mchoice::MultipleChoice {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: "Autenticidade + praticidade + economia".to_string(),
            weights: serde_json::json!({"danavox": 1}),
        };

        let answer_04 = question_mchoice::MultipleChoice {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: "Independência + otimismo + custo-benefício".to_string(),
            weights: serde_json::json!({"interton": 1}),
        };

        let question = Question {
            id: question_id,
            quiz_id: vec![quiz_id],
            question_text: "Ao atender um paciente, qual combinação de fatores mais representa o que você quer oferecer?".to_string(),
            answers: Answers::MultipleChoice(vec![answer_01.clone(), answer_02.clone(), answer_03.clone(), answer_04.clone()]),
            // created_at: now_utc.naive_utc(),
            // updated_at: now_utc.naive_utc(),
            // is_deleted: false,
        };

        assert_eq!(question.id, question_id);
        assert_eq!(question.quiz_id, vec![quiz_id]);
        assert_eq!(
            question.question_text,
            "Ao atender um paciente, qual combinação de fatores mais representa o que você quer oferecer?"
        );
        assert_eq!(
            question.answers,
            Answers::MultipleChoice(vec![answer_01, answer_02, answer_03, answer_04])
        );
        // assert!(!question.is_deleted);
        // assert_eq!(question.created_at, now_utc.naive_utc());
        // assert_eq!(question.updated_at, now_utc.naive_utc());
    }
}
