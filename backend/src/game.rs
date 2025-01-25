use uuid::Uuid;

pub struct Game {
    id: Uuid,
    players: Vec<PlayerData>,
    current_question: i32,
    questions: Vec<Uuid>,
}

pub struct PlayerData {
    id: Uuid,
    name: String,
    score: i32,
}
