use rand::seq::SliceRandom;
use serde::Serialize;
use uuid::Uuid;

use crate::models;

pub struct Game {
    id: Uuid,
    pub players: Vec<PlayerData>,
    current_question: usize,
    questions: Vec<Question>,
    categories: Vec<Uuid>,
}

impl Game {
    pub fn new(question_count: usize, categories: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            players: Vec::new(),
            current_question: 0,
            questions: Vec::with_capacity(question_count),
            categories,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_questions(&self) -> &[Question] {
        &self.questions
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }

    pub fn random_category(&self) -> Uuid {
        *self.categories.choose(&mut rand::thread_rng()).unwrap()
    }
}

pub struct PlayerData {
    id: Uuid,
    name: String,
    score: i32,
}

impl PlayerData {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            score: 0,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }
}

#[derive(Clone, Serialize)]
pub struct Question {
    pub text: String,
    pub answers: Vec<String>,
    pub category: Uuid,
    pub id: Uuid,
    pub answer: usize,
}

impl From<models::question::Question> for Question {
    fn from(question: models::question::Question) -> Self {
        let rand = question.randomize();
        Self {
            text: question.question,
            answers: rand.0,
            category: question.category,
            id: question.id,
            answer: rand.1,
        }
    }
}
