use axum::{extract::State, response::Response, Router};

use crate::AppState;

pub struct GameRouter;

impl GameRouter {
    pub async fn start(State(AppState { games, database }): State<AppState>) -> Response {
        todo!()
    }
}

impl Into<Router<AppState>> for GameRouter {
    fn into(self) -> Router<AppState> {
        Router::new()
    }
}
