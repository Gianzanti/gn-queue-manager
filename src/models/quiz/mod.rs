use std::collections::HashSet;
use uuid::Uuid;

mod question;

pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub back_img: String,
    pub has_matching: bool,
    pub matches: HashSet<String>,
    pub match_threshold: i32,
    pub questions: Vec<question::Question>,
    // pub created_at: chrono::NaiveDateTime,
    // pub updated_at: chrono::NaiveDateTime,
    // pub is_active: bool,
    // pub is_deleted: bool,
}

#[cfg(test)]
mod tests {
    use crate::models::quiz::question::{
        Answers, Question, question_classification::Classification,
        question_mchoice::MultipleChoice,
    };

    use super::*;

    fn mount_question_multiple_choice(quiz_id: Uuid) -> Question {
        let question_id = Uuid::now_v7();

        let answer_01 = MultipleChoice {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: "Experiência + visual + tecnologia".to_string(),
            weights: serde_json::json!({"resound": 1}),
        };

        let answer_02 = MultipleChoice {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: "Bem-estar + acolhimento _ conexão".to_string(),
            weights: serde_json::json!({"beltone": 1}),
        };

        let answer_03 = MultipleChoice {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: "Autenticidade + praticidade + economia".to_string(),
            weights: serde_json::json!({"danavox": 1}),
        };

        let answer_04 = MultipleChoice {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: "Independência + otimismo + custo-benefício".to_string(),
            weights: serde_json::json!({"interton": 1}),
        };

        Question {
            id: question_id,
            quiz_id: vec![quiz_id],
            question_text: "Ao atender um paciente, qual combinação de fatores mais representa o que você quer oferecer?".to_string(),
            answers: Answers::MultipleChoice(vec![answer_01.clone(), answer_02.clone(), answer_03.clone(), answer_04.clone()]),
            // created_at: now_utc.naive_utc(),
            // updated_at: now_utc.naive_utc(),
            // is_deleted: false,
        }
    }

    fn mount_question_classification(quiz_id: Uuid) -> Question {
        let question_id = Uuid::now_v7();

        let item_01 = Classification {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: String::from("Humano"),
            weights: serde_json::json!({"interton": 1, "beltone": 1, "danavox": 1}),
        };

        let item_02 = Classification {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: String::from("Inovador"),
            weights: serde_json::json!({"interton": 1, "beltone": 1, "resound": 1}),
        };

        let item_03 = Classification {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: String::from("Tecnológico"),
            weights: serde_json::json!({"resound": 1}),
        };

        let item_04 = Classification {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: String::from("Inclusivo"),
            weights: serde_json::json!({"interton": 1, "danavox": 1}),
        };

        let item_05 = Classification {
            id: Uuid::now_v7(),
            question_id: vec![question_id],
            option_text: String::from("Confiável"),
            weights: serde_json::json!({"beltone": 1, "resound": 1}),
        };

        Question {
            id: question_id,
            quiz_id: vec![quiz_id],
            question_text: "Ao atender um paciente, qual combinação de fatores mais representa o que você quer oferecer?".to_string(),
            answers: Answers::Classification(vec![item_01.clone(), item_02.clone(), item_03.clone(), item_04.clone(), item_05.clone()]),
            // created_at: now_utc.naive_utc(),
            // updated_at: now_utc.naive_utc(),
            // is_deleted: false,
        }
    }

    #[test]
    fn test_quiz_creation() {
        // Get the current time in UTC
        // let now_utc = Utc::now();

        let quiz_id = Uuid::now_v7();
        let matches = HashSet::from([
            "Resound".to_string(),
            "Beltone".to_string(),
            "Danavox".to_string(),
            "Interton".to_string(),
        ]);

        let question_01 = mount_question_multiple_choice(quiz_id);
        let question_02 = mount_question_classification(quiz_id);

        let quiz = Quiz {
            id: quiz_id,
            title: "Quiz de Marca".to_string(),
            description: "Descubra qual marca de aparelho auditivo mais combina com você!"
                .to_string(),
            back_img: "https://example.com/quiz-background.jpg".to_string(),
            has_matching: true,
            matches,
            match_threshold: 1,
            questions: vec![question_01.clone(), question_02.clone()],
            // created_at: now_utc.naive_utc(),
            // updated_at: now_utc.naive_utc(),
            // is_active: true,
            // is_deleted: false,
        };
    }
}
