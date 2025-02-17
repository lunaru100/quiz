use std::{collections::HashMap, str::FromStr, sync::Arc};

use axum::{extract::Request, middleware, Router, ServiceExt};
use game::Game;
use rand::Rng;
use routers::{auth::AuthRouter, game::GameRouter};
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::{
    normalize_path::NormalizePath,
    services::{ServeDir, ServeFile},
};
use uuid::Uuid;

mod routers;
mod game;
mod models;

#[derive(Clone)]
pub struct AppState {
    database: Arc<SqlitePool>,
    games: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Game>>>>>,
}

#[tokio::main]
async fn main() {
    routers::auth::SECRET
        .get_or_init(|| async {
            let secret = dotenvy::var("JWT_SECRET").unwrap_or_else(|_| {
                rand::thread_rng()
                    .sample_iter(rand::distributions::Alphanumeric)
                    .take(64)
                    .map(char::from)
                    .collect()
            });
            println!("SECRET: {}", secret);
            secret
        })
        .await;

    let pool = SqlitePoolOptions::new()
        //.before_acquire(|conn, _| {
        //    Box::pin(async move { conn.before_acquire().await.map(|_| true) })
        //})
        .connect_with(
            SqliteConnectOptions::from_str(
                dotenvy::var("DATABASE")
                    .unwrap_or("sqlite://dev.sqlite".to_string())
                    .as_str(),
            )
            .unwrap()
            .pragma(
                "key",
                dotenvy::var("DATABASE_KEY").unwrap_or("secret".to_string()),
            )
            .pragma("foreign_keys", "ON"),
        )
        .await
        .unwrap();
    let app = Router::new()
        .nest("/api", Router::new()
            .nest("/game", GameRouter.into())
            .nest("/auth", AuthRouter.into())
        )
        .nest_service("/example", ServeDir::new("./example"))
        .nest_service("/assets", ServeDir::new("../frontend/dist/assets"))
        .nest_service("/img", ServeDir::new("../frontend/img"))
        .layer(middleware::from_fn(routers::auth::authorize))
        .fallback_service(ServeFile::new("../frontend/dist/index.html"))
        .with_state(AppState {
            database: Arc::new(pool),
            games: Arc::new(RwLock::new(HashMap::new())),
        });
    let app = NormalizePath::trim_trailing_slash(app);
    let port = dotenvy::var("PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}
