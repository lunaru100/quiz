use axum::Router;

use crate::AppState;

pub struct GameRouter;

impl Into<Router<AppState>> for GameRouter {
    fn into(self) -> Router<AppState> {
        Router::new()
    }
}
