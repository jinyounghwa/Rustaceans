use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use actix_web::{post, web, HttpResponse, Responder, App, HttpServer, middleware};
use bcrypt::{hash, verify};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    iat: usize,
    exp: usize,
}

const SECRET: &[u8] = b"your-secret-key-should-be-long-and-random";

fn create_token(user_id: &str, email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::days(1);
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}

#[allow(dead_code)]
fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default())?;
    Ok(token_data.claims)
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct UserResponse {
    id: Uuid,
    email: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    password_hash: String,
}

#[post("/auth/login")]
async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash FROM users WHERE email = $1"
    )
    .bind(&body.email)
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => {
            if verify(&body.password, &user.password_hash).unwrap_or(false) {
                match create_token(&user.id.to_string(), &user.email) {
                    Ok(token) => {
                        HttpResponse::Ok().json(LoginResponse {
                            token,
                            user: UserResponse {
                                id: user.id,
                                email: user.email,
                                name: user.name,
                            },
                        })
                    }
                    Err(_) => HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": "토큰 생성 실패"})),
                }
            } else {
                HttpResponse::Unauthorized()
                    .json(serde_json::json!({"error": "비밀번호가 틀렸습니다"}))
            }
        }
        Ok(None) => HttpResponse::Unauthorized()
            .json(serde_json::json!({"error": "사용자를 찾을 수 없습니다"})),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "데이터베이스 오류"}))
        },
    }
}

#[derive(Deserialize)]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
}

#[post("/auth/register")]
async fn register(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    let password_hash = match hash(&body.password, 12) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": "비밀번호 해싱 실패"})),
    };

    let result = sqlx::query_as::<_, UserResponse>(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id, name, email"
    )
    .bind(&body.name)
    .bind(&body.email)
    .bind(password_hash)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => {
            let token = create_token(&user.id.to_string(), &user.email)
                .unwrap_or_default();

            HttpResponse::Created().json(LoginResponse {
                token,
                user, // UserResponse implements Serialize
            })
        }
        Err(e) => {
            eprintln!("Registration error: {}", e);
            HttpResponse::Conflict()
                .json(serde_json::json!({"error": "이미 존재하는 이메일입니다"}))
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/dbname".to_string());
        
    println!("Connecting to database: {}", database_url);

    let pool = match PgPool::connect(&database_url).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Database connection failed: {}. Server will exit.", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(login)
            .service(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}