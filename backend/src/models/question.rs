use rand::{seq::SliceRandom, Rng};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Question {
    pub question: String,
    a: String,
    b: String,
    c: String,
    d: String,
    answer: u8,
    pub id: Uuid,
    pub category: Uuid,
}

impl Question {
    pub fn randomize(&self) -> (Vec<String>, usize) {
        let mut answers = vec![self.a.clone(), self.b.clone(), self.c.clone(), self.d.clone()];
        let correct = answers.remove(self.answer as usize);

        answers.shuffle(&mut rand::thread_rng());
        let index = rand::thread_rng().gen_range(0..4);

        answers.insert(index, correct);
        (answers, index)
    }
}
