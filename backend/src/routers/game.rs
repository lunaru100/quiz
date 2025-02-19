use std::{collections::{HashMap, HashSet}, sync::Arc};

use axum::{extract::State, middleware, response::{IntoResponse, Response}, routing::{get, post}, Extension, Json, Router};
use http::StatusCode;
use serde::Deserialize;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{game::{Game, PlayerData, Question}, models::{self, user::User}, AppState};

use super::auth::{self, Claims};

pub struct GameRouter;

#[derive(Deserialize)]
pub struct GameConfig {
    pub categories: Vec<Uuid>,
    pub num_questions: usize,
}

#[derive(Deserialize)]
pub struct QuestionRequest {
    game_id: Uuid,
}

#[derive(Deserialize)]
pub struct AnswerRequest {
    game_id: Uuid,
    answer: usize,
}

impl GameRouter {
    pub async fn start(
        State(AppState { games, database }): State<AppState>,
        Extension(claims): Extension<Claims>,
        Json(config): Json<GameConfig>,
    ) -> Response {
        let game = Game::new(config.num_questions, config.categories);
        let game = Arc::new(RwLock::new(game));
        {
            games.write().await.insert(game.clone().read().await.get_id(), game.clone());
        }


        let user = if claims.guest {
            User::new_guest(claims.id)
        } else {
            match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?1")
                .bind(claims.id)
                .fetch_one(&*database)
                .await {
                    Ok(user) => user,
                    Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
                }
        };
        game.clone().write().await.players.push(PlayerData::new(user.id, user.username));

        (StatusCode::CREATED, Json(game.clone().read().await.get_id())).into_response()
    }

    pub async fn question(
        State(AppState { games, database }): State<AppState>,
        Extension(claims): Extension<Claims>,
        Json(QuestionRequest { game_id }): Json<QuestionRequest>,
    ) -> Response {
        let game = match games.read().await.get(&game_id) {
            Some(game) => game.clone(),
            None => return StatusCode::NOT_FOUND.into_response(),
        };

        let game_ref = game.read().await;

        if !(&game_ref.players).into_iter().find(|x| x.get_id() == claims.id).is_none() {
            return StatusCode::UNAUTHORIZED.into_response();
        }
        /*
         sqlx::query_as::
            <_, models::question::Question>
            ("SELECT * FROM questions WHERE category = ?1 AND ID NOT IN ({}) ORDER BY RANDOM() LIMIT 1")
            .bind(game_ref.random_category())
         * */

        let id_hash_set = game_ref.get_questions().into_iter().map(|x| x.id).collect::<HashSet<_>>();
        let sql = format!(
            "SELECT * FROM questions WHERE category = ?1 AND ID NOT IN ({}) ORDER BY RANDOM() LIMIT 1",
            (2..=id_hash_set.len()+2)
                .map(|x| format!("?{}", x))
                .collect::<Vec<String>>()
                .join(",")
        );

        let mut query = sqlx::query_as::<_, models::question::Question>(&sql)
            .bind(game_ref.random_category());

        for id in id_hash_set {
            query = query.bind(id);
        }

        let question = match query
            .fetch_optional(&*database)
            .await
        {
            Ok(Some(question)) => question,
            _ => return StatusCode::NOT_FOUND.into_response(),
        };

        let question: Question = question.into();
        drop(game_ref);
        let mut game_ref = game.write().await;
        game_ref.add_question(question.clone());
        (StatusCode::OK, Json(question)).into_response()
    }

    pub async fn answer(
        State(AppState { games, database }): State<AppState>,
        Extension(claims): Extension<Claims>,
        Json(AnswerRequest { game_id, answer }): Json<AnswerRequest>,
    ) -> Response {
        let game = match games.read().await.get(&game_id) {
            Some(game) => game.clone(),
            None => return StatusCode::NOT_FOUND.into_response(),
        };

        let game_ref = game.read().await;

        if !(&game_ref.players).into_iter().find(|x| x.get_id() == claims.id).is_none() {
            return StatusCode::UNAUTHORIZED.into_response();
        }

        (StatusCode::OK, Json(game_ref.get_questions().last().unwrap().answer == answer)).into_response()
    }

    pub async fn categories(
        State(AppState { database, .. }): State<AppState>,
    ) -> Response {
        let categories = sqlx::query_as::<_, models::category::Category>("SELECT * FROM categories")
            .fetch_all(&*database)
            .await
            .unwrap();

        (StatusCode::OK, Json(categories
            .into_iter()
            .map(|x| (x.name, x.id))
            .collect::<HashMap<_, _>>())).into_response()
    }
}

impl Into<Router<AppState>> for GameRouter {
    fn into(self) -> Router<AppState> {
        Router::new()
            .route("/categories", get(Self::categories))
            .route("/start", post(Self::start))
            .route("/question", post(Self::question))
            .route("/answer", post(Self::answer))
            .layer(middleware::from_fn(auth::force_authorize))
    }
}
