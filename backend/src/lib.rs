use std::sync::Arc;

pub mod config;
pub mod controllers;
pub mod db;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

use config::Config;
use middleware::rate_limit::{RateLimitState, SlidingWindowLimiter};
use services::stellar_service::StellarService;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<Config>,
    pub stellar: Arc<StellarService>,
    pub rate_limiter: Arc<RateLimitState>,
    pub auth_rate_limiter: Arc<SlidingWindowLimiter>,
}
