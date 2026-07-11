use std::net::SocketAddr;
use std::sync::Arc;

use ajo_backend::{
    config::Config,
    middleware::rate_limit::{RateLimitState, SlidingWindowLimiter},
    models::user::CreateUserInput,
    routes::build_router,
    services::{auth_service::AuthService, stellar_service::StellarService},
    AppState,
};
use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    Router,
};
use chrono::Utc;
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

fn test_config() -> Config {
    Config {
        database_url: String::new(),
        jwt_secret: "test-secret-at-least-32-bytes-long-for-hs256".to_string(),
        jwt_access_expiry_secs: 900,
        jwt_refresh_expiry_secs: 604_800,
        host: "0.0.0.0".to_string(),
        port: 8080,
        frontend_url: "http://localhost:3000".to_string(),
        horizon_url: "https://horizon-testnet.stellar.org".to_string(),
        soroban_rpc_url: "https://soroban-testnet.stellar.org".to_string(),
        contract_id: "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4".to_string(),
        app_version: "test".to_string(),
        // Deliberately invalid — verification/reset emails fail to send in tests, which
        // AuthService logs and swallows rather than propagating, so registration and
        // password-reset flows still succeed without a live Resend account.
        resend_api_key: "re_test_invalid".to_string(),
        resend_from_email: "Ajo Platform <test@ajo-platform.com>".to_string(),
        email_verification_token_expiry_hours: 24,
        password_reset_token_expiry_minutes: 30,
    }
}

fn test_app(pool: PgPool) -> Router {
    let config = Arc::new(test_config());
    let stellar = Arc::new(StellarService::new(
        config.horizon_url.clone(),
        config.soroban_rpc_url.clone(),
        config.contract_id.clone(),
    ));

    let state = AppState {
        db: pool,
        config,
        stellar,
        rate_limiter: Arc::new(RateLimitState::new(1000.0)),
        auth_rate_limiter: Arc::new(SlidingWindowLimiter::new()),
    };

    build_router(state)
}

fn json_request(method: &str, uri: &str, body: Value) -> Request<Body> {
    let addr: SocketAddr = "127.0.0.1:1234".parse().unwrap();
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .extension(ConnectInfo(addr))
        .body(Body::from(body.to_string()))
        .unwrap()
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap_or(Value::Null)
}

fn unique_email(label: &str) -> String {
    format!("{label}-{}@example.com", Uuid::new_v4())
}

fn signup_input(email: &str) -> CreateUserInput {
    CreateUserInput {
        email: email.to_string(),
        password: "CorrectHorse7!".to_string(),
        display_name: "Test User".to_string(),
        wallet_address: None,
    }
}

#[sqlx::test]
async fn duplicate_email_registration_is_rejected(pool: PgPool) {
    let app = test_app(pool);
    let email = unique_email("dup");

    let payload = json!({
        "email": email,
        "password": "CorrectHorse7!",
        "display_name": "Amara Okonkwo",
    });

    let first = app
        .clone()
        .oneshot(json_request("POST", "/api/v1/auth/register", payload.clone()))
        .await
        .unwrap();
    assert_eq!(first.status(), StatusCode::CREATED);

    let second = app
        .oneshot(json_request("POST", "/api/v1/auth/register", payload))
        .await
        .unwrap();
    assert_eq!(second.status(), StatusCode::CONFLICT);

    let body = body_json(second).await;
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("already registered"));
}

#[sqlx::test]
async fn weak_password_registration_is_rejected(pool: PgPool) {
    let app = test_app(pool);

    // Passes length + complexity but is in the top-10k breached-password list.
    let payload = json!({
        "email": unique_email("weak"),
        "password": "Password1",
        "display_name": "Amara Okonkwo",
    });

    let response = app
        .oneshot(json_request("POST", "/api/v1/auth/register", payload))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = body_json(response).await;
    let message = body["error"]["message"].as_str().unwrap().to_lowercase();
    assert!(message.contains("common") || message.contains("breach"));
}

#[sqlx::test]
async fn missing_complexity_registration_is_rejected(pool: PgPool) {
    let app = test_app(pool);

    let payload = json!({
        "email": unique_email("nodigits"),
        "password": "NoDigitsHere",
        "display_name": "Amara Okonkwo",
    });

    let response = app
        .oneshot(json_request("POST", "/api/v1/auth/register", payload))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test]
async fn expired_verification_token_is_rejected(pool: PgPool) {
    let config = test_config();
    let email = unique_email("verify-expired");

    let user = AuthService::register(&pool, &config, signup_input(&email))
        .await
        .expect("registration should succeed");

    // Insert an already-expired token directly, bypassing the 24h default.
    let token_plain = "expired-token-plain-value";
    let token_hash = ajo_backend::utils::tokens::hash_token(token_plain);

    sqlx::query(
        "INSERT INTO email_verification_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(user.id)
    .bind(&token_hash)
    .bind(Utc::now() - chrono::Duration::hours(1))
    .execute(&pool)
    .await
    .unwrap();

    let err = AuthService::verify_email(&pool, token_plain)
        .await
        .expect_err("expired token should be rejected");
    assert!(matches!(err, ajo_backend::errors::AppError::BadRequest(_)));

    let verified: bool = sqlx::query_scalar("SELECT email_verified FROM users WHERE id = $1")
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!verified, "expired token must not verify the account");
}

#[sqlx::test]
async fn valid_verification_token_verifies_email_once(pool: PgPool) {
    let config = test_config();
    let email = unique_email("verify-ok");

    let user = AuthService::register(&pool, &config, signup_input(&email))
        .await
        .expect("registration should succeed");

    let token_plain = "valid-token-plain-value";
    let token_hash = ajo_backend::utils::tokens::hash_token(token_plain);

    sqlx::query(
        "INSERT INTO email_verification_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(user.id)
    .bind(&token_hash)
    .bind(Utc::now() + chrono::Duration::hours(1))
    .execute(&pool)
    .await
    .unwrap();

    AuthService::verify_email(&pool, token_plain)
        .await
        .expect("valid token should verify the account");

    let verified: bool = sqlx::query_scalar("SELECT email_verified FROM users WHERE id = $1")
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(verified);

    // Reusing the same (now-consumed) token must fail.
    let err = AuthService::verify_email(&pool, token_plain).await;
    assert!(err.is_err(), "an already-used verification token must not verify twice");
}

#[sqlx::test]
async fn refresh_token_reuse_revokes_the_session_family(pool: PgPool) {
    let config = test_config();
    let email = unique_email("reuse");

    AuthService::register(&pool, &config, signup_input(&email))
        .await
        .expect("registration should succeed");

    let pair = AuthService::login(
        &pool,
        &email,
        "CorrectHorse7!",
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await
    .expect("login should succeed");

    // First rotation: legitimate use of the original refresh token.
    let rotated = AuthService::refresh(
        &pool,
        &pair.refresh_token,
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await
    .expect("first refresh should succeed");

    // Replaying the original (now rotated-out) token is a reuse — must fail...
    let reuse_err = AuthService::refresh(
        &pool,
        &pair.refresh_token,
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await;
    assert!(reuse_err.is_err(), "reusing a rotated-out refresh token must fail");

    // ...and must also invalidate the token that would have chained from it, i.e. the
    // whole family is dead, including the legitimately-rotated one.
    let chained_err = AuthService::refresh(
        &pool,
        &rotated.refresh_token,
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await;
    assert!(
        chained_err.is_err(),
        "detecting reuse must revoke the entire token family, not just the replayed token"
    );
}

#[sqlx::test]
async fn login_rate_limit_triggers_after_max_attempts(pool: PgPool) {
    let app = test_app(pool);
    let email = unique_email("ratelimit");

    // Register the account first so login reaches the password check, not a 404-shaped path.
    let register_payload = json!({
        "email": email,
        "password": "CorrectHorse7!",
        "display_name": "Amara Okonkwo",
    });
    let created = app
        .clone()
        .oneshot(json_request("POST", "/api/v1/auth/register", register_payload))
        .await
        .unwrap();
    assert_eq!(created.status(), StatusCode::CREATED);

    let bad_login = json!({ "email": email, "password": "WrongPassword9!" });

    for attempt in 0..5 {
        let response = app
            .clone()
            .oneshot(json_request("POST", "/api/v1/auth/login", bad_login.clone()))
            .await
            .unwrap();
        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "attempt {attempt} should be a normal auth failure, not yet rate limited"
        );
    }

    // The 6th attempt within the window must be rate limited rather than another 401.
    let limited = app
        .oneshot(json_request("POST", "/api/v1/auth/login", bad_login))
        .await
        .unwrap();
    assert_eq!(limited.status(), StatusCode::TOO_MANY_REQUESTS);
}

#[sqlx::test]
async fn password_reset_invalidates_existing_sessions(pool: PgPool) {
    let config = test_config();
    let email = unique_email("reset");

    AuthService::register(&pool, &config, signup_input(&email))
        .await
        .expect("registration should succeed");

    let pair = AuthService::login(
        &pool,
        &email,
        "CorrectHorse7!",
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await
    .expect("login should succeed");

    let token_plain = "reset-token-plain-value";
    let token_hash = ajo_backend::utils::tokens::hash_token(token_plain);
    let user_id: Uuid = sqlx::query_scalar("SELECT id FROM users WHERE email = $1")
        .bind(&email)
        .fetch_one(&pool)
        .await
        .unwrap();

    sqlx::query(
        "INSERT INTO password_reset_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&token_hash)
    .bind(Utc::now() + chrono::Duration::minutes(30))
    .execute(&pool)
    .await
    .unwrap();

    AuthService::reset_password(&pool, token_plain, "NewCorrectHorse9!")
        .await
        .expect("reset should succeed");

    // The refresh token issued before the reset must no longer work.
    let err = AuthService::refresh(
        &pool,
        &pair.refresh_token,
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await;
    assert!(err.is_err(), "password reset must invalidate pre-existing refresh tokens");

    // Old password no longer works; new one does.
    let old_login = AuthService::login(
        &pool,
        &email,
        "CorrectHorse7!",
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await;
    assert!(old_login.is_err());

    AuthService::login(
        &pool,
        &email,
        "NewCorrectHorse9!",
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await
    .expect("new password should work after reset");
}

#[sqlx::test]
async fn login_never_reveals_whether_the_email_exists(pool: PgPool) {
    let config = test_config();
    let email = unique_email("enum");

    AuthService::register(&pool, &config, signup_input(&email))
        .await
        .expect("registration should succeed");

    let wrong_password_err = AuthService::login(
        &pool,
        &email,
        "WrongPassword9!",
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await
    .expect_err("wrong password should fail");

    let unknown_email_err = AuthService::login(
        &pool,
        &unique_email("never-registered"),
        "WrongPassword9!",
        &config.jwt_secret,
        config.jwt_access_expiry_secs,
        config.jwt_refresh_expiry_secs,
    )
    .await
    .expect_err("unknown email should fail");

    assert_eq!(wrong_password_err.to_string(), unknown_email_err.to_string());
}
