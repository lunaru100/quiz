use std::{str::FromStr, sync::Arc};

use axum::{extract::Request, Router, ServiceExt};
use routers::{auth::AuthRouter, game::GameRouter};
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use tokio::net::TcpListener;
use tower_http::{
    normalize_path::NormalizePath,
    services::{ServeDir, ServeFile},
};

mod routers;

#[derive(Clone)]
pub struct AppState {
    database: Arc<SqlitePool>,
}

#[tokio::main]
async fn main() {
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
            .nest("/game", AuthRouter.into())
            .nest("/auth", GameRouter.into())
        )
        .nest_service("/example", ServeDir::new("./example"))
        .nest_service("/assets", ServeDir::new("../frontend/dist/assets"))
        .nest_service("/img", ServeDir::new("../frontend/img"))
        .fallback_service(ServeFile::new("../frontend/dist/index.html"))
        .with_state(AppState {
            database: Arc::new(pool)
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
