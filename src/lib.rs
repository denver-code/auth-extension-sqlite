use axum::{extract::Extension, response::Json, routing::post, Router};
use corex_api::ExtensionTrait;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use std::sync::Arc;

/// Represents a user in the database.
#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: i64,
    username: String,
    password_hash: String,
}

/// Request body for the register endpoint.
#[derive(Debug, Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

/// Request body for the login endpoint.
#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

/// Response for successful login or register.
#[derive(Debug, Serialize)]
struct AuthResponse {
    message: String,
    user_id: i64,
}

/// Error response for failed login or register.
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

/// SQLite-based authentication extension.
pub struct AuthExtensionSQLite {
    pool: Arc<SqlitePool>,
}

impl AuthExtensionSQLite {
    /// Initialize the SQLite database and run migrations.
    async fn init_db() -> Result<SqlitePool, String> {
        // Create the database if it doesn't exist
        let db_url = "sqlite:auth.db";
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            Sqlite::create_database(db_url)
                .await
                .map_err(|e| format!("Failed to create database: {}", e))?;
        }

        // Create a connection pool
        let pool = SqlitePool::connect(db_url)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        // Run migrations
        let m = sqlx::migrate!("./migrations"); // Path to the migrations folder
        m.run(&pool)
            .await
            .map_err(|e| format!("Failed to run migrations: {}", e))?;

        Ok(pool)
    }

    /// Register a new user.
    async fn register(
        Extension(pool): Extension<Arc<SqlitePool>>,
        axum::Json(payload): axum::Json<RegisterRequest>,
    ) -> Result<Json<AuthResponse>, Json<ErrorResponse>> {
        // Hash the password (use a proper hashing library like `argon2` in production)
        let password_hash = format!("{:x}", md5::compute(&payload.password));

        // Insert the user into the database
        let result = sqlx::query(
            r#"
            INSERT INTO users (username, password_hash)
            VALUES (?, ?)
            "#,
        )
        .bind(&payload.username)
        .bind(&password_hash)
        .execute(&*pool)
        .await;

        match result {
            Ok(result) => {
                let user_id = result.last_insert_rowid();
                Ok(Json(AuthResponse {
                    message: "User registered successfully".to_string(),
                    user_id,
                }))
            }
            Err(e) => Err(Json(ErrorResponse {
                error: format!("Failed to register user: {}", e),
            })),
        }
    }

    /// Login an existing user.
    async fn login(
        Extension(pool): Extension<Arc<SqlitePool>>,
        axum::Json(payload): axum::Json<LoginRequest>,
    ) -> Result<Json<AuthResponse>, Json<ErrorResponse>> {
        // Hash the password (use a proper hashing library like `argon2` in production)
        let password_hash = format!("{:x}", md5::compute(&payload.password));

        // Fetch the user from the database
        let result = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, password_hash
            FROM users
            WHERE username = ? AND password_hash = ?
            "#,
        )
        .bind(&payload.username)
        .bind(&password_hash)
        .fetch_optional(&*pool)
        .await;

        match result {
            Ok(Some(user)) => Ok(Json(AuthResponse {
                message: "Login successful".to_string(),
                user_id: user.id,
            })),
            Ok(None) => Err(Json(ErrorResponse {
                error: "Invalid username or password".to_string(),
            })),
            Err(e) => Err(Json(ErrorResponse {
                error: format!("Failed to login: {}", e),
            })),
        }
    }
}

impl ExtensionTrait for AuthExtensionSQLite {
    fn name(&self) -> &'static str {
        "AuthExtensionSQLite"
    }

    fn extend(&self, router: Router) -> Router {
        // Add the login and register endpoints
        router
            .route("/register", post(Self::register))
            .route("/login", post(Self::login))
            .layer(Extension(self.pool.clone()))
    }
}

/// Create and return an instance of `AuthExtensionSQLite`.
pub async fn create_auth_extension() -> Result<Arc<AuthExtensionSQLite>, String> {
    let pool = AuthExtensionSQLite::init_db().await?;
    Ok(Arc::new(AuthExtensionSQLite {
        pool: Arc::new(pool),
    }))
}
