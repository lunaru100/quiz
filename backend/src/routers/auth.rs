use std::collections::HashMap;

use argon2::{Argon2, PasswordHash, PasswordVerifier, PasswordHasher};
use axum::{extract::{Request, State}, http::{HeaderMap, HeaderValue, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Json, Router};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::{models::user::User, AppState};

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
        let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?1").bind(&params.login).fetch_optional(&*database).await {
            Ok(Some(user)) => user,
            Ok(None) => match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?1").bind(&params.login).fetch_optional(&*database).await {
                Ok(Some(user)) => user,
                Ok(None) => return (StatusCode::UNAUTHORIZED, "Wrong username or password").into_response(),
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        match Argon2::default().verify_password(
            params.password.as_bytes(),
            &PasswordHash::new(&user.password).unwrap(),
        ) {
            Ok(_) => {}
            Err(_) => {
                return (StatusCode::UNAUTHORIZED, "Wrong username or password").into_response()
            }
        }

        let res = match encode_jwt(&user.id) {
            Ok(res) => res,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

       (
            StatusCode::OK,
            HeaderMap::try_from(&HashMap::from([(
                http::header::SET_COOKIE,
                HeaderValue::from_str(
                    Cookie::build(("Authorization", &res))
                        .path("/")
                        .http_only(true)
                        .same_site(axum_extra::extract::cookie::SameSite::Lax)
                        .build()
                        .to_string()
                        .as_str(),
                )
                .unwrap(),
                //HeaderValue::from_str(format!("Authorization={}; Path=/; HttpOnly; SameSite=Lax", res).as_str())
                //    .unwrap(),
            )]))
            .unwrap(),
            Json(LoginResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            }),
        )
            .into_response()

    }

    pub async fn register(
        State(AppState { database, .. }): State<AppState>,
        Json(params): Json<RegisterParams>,
    ) -> Response {
        match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username IN (?1, ?2) OR email IN (?1, ?2)")
            .bind(&params.username)
            .bind(&params.email)
            .fetch_optional(&*database)
            .await
        {
            Ok(Some(user)) => {
            if user.username == params.username || user.username == params.email {
                return (StatusCode::BAD_REQUEST, "Username").into_response();
            }
            return (StatusCode::BAD_REQUEST, "Email").into_response();

            }
            Ok(None) => {},
            Err(error) => {
                println!("{error:?}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }

        let salt = argon2::password_hash::SaltString::generate(&mut rand::rngs::OsRng);
        let password = match Argon2::default().hash_password(params.password.as_bytes(), &salt) {
            Ok(password) => password.to_string(),
            Err(error) => {
                println!("{error:?}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        let id = Uuid::new_v4();
        match sqlx::query("INSERT INTO users (id, username, email, password) VALUES (?1, ?2, ?3, ?4)")
            .bind(&id)
            .bind(&params.username)
            .bind(&params.email)
            .bind(&password)
            .execute(&*database)
            .await
        {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(error) => {
                println!("{error:?}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    pub async fn logout() -> Response {
        (
            StatusCode::OK,
            HeaderMap::try_from(&HashMap::from([(
                http::header::SET_COOKIE,
                HeaderValue::from_static(r#"Authorization="";"#),
            )]))
            .unwrap(),
        )
            .into_response()
    }
}

impl Into<Router<AppState>> for AuthRouter {
    fn into(self) -> Router<AppState> {
        Router::new()
            .route("/login", axum::routing::post(Self::login))

    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
    pub id: Uuid,
}

pub static SECRET: OnceCell<String> = OnceCell::const_new();

pub fn encode_jwt(id: &Uuid) -> Result<String, ()> {
    let now = chrono::Utc::now();
    let delta = chrono::Duration::days(1);
    let exp = (now + delta).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let claims = Claims {
        exp,
        iat,
        id: id.clone(),
    };
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.get().unwrap().as_ref()),
    )
    .map_err(|_| ())
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, ()> {
    let decoded = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.get().unwrap().as_ref()),
        &Validation::default(),
    )
    .map_err(|_e| { /*println!("{:?}", _e)*/ })?;

    Ok(decoded)
}

pub async fn authorize(
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("{}", request.uri());

    let get_auth = |jar: CookieJar| -> Option<Claims> {
        Some(decode_jwt(jar.get("Authorization")?.value()).ok()?.claims)
    };
    request.extensions_mut().insert(get_auth(jar));

    Ok(next.run(request).await)
}

pub async fn force_authorize(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let ext: Option<Claims> = request.extensions_mut().remove::<Option<Claims>>().unwrap();
    match ext {
        Some(ext) => {
            request.extensions_mut().insert(ext);
            Ok(next.run(request).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
