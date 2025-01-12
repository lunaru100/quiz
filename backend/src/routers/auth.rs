use axum::{extract::State, response::{IntoResponse, Response}, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

pub struct AuthRouter;


#[derive(Serialize, Deserialize)]
pub struct LoginParams {
    #[serde(alias = "username", alias = "email")]
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterParams {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl AuthRouter {
    pub async fn login(State(AppState { database, .. }): State<AppState>, Json(params): Json<LoginParams>) -> Response {
        todo!()
    }
}

impl Into<Router<AppState>> for AuthRouter {
    fn into(self) -> Router<AppState> {
        Router::new()
            .route("/login", axum::routing::post(Self::login))

    }
}
