use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::{errors::AppError, AppState};

#[derive(Debug)]
struct TokenBucket {
    tokens: f64,
    capacity: f64,
    refill_rate: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            tokens: capacity,
            capacity,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    fn try_consume(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity);
        self.last_refill = now;

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct RateLimitState {
    buckets: Arc<Mutex<HashMap<String, TokenBucket>>>,
    capacity: f64,
    refill_rate: f64,
}

impl RateLimitState {
    pub fn new(requests_per_minute: f64) -> Self {
        let refill_rate = requests_per_minute / 60.0;
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            capacity: requests_per_minute,
            refill_rate,
        }
    }

    fn try_consume(&self, ip: &str) -> bool {
        let mut buckets = self.buckets.lock().expect("mutex not poisoned");

        let bucket = buckets
            .entry(ip.to_string())
            .or_insert_with(|| TokenBucket::new(self.capacity, self.refill_rate));

        bucket.try_consume()
    }
}

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let ip = addr.ip().to_string();

    if !state.rate_limiter.try_consume(&ip) {
        tracing::warn!(ip = %ip, "Rate limit exceeded");
        return Err(AppError::TooManyRequests);
    }

    Ok(next.run(req).await)
}

pub fn cleanup_old_buckets(rate_limiter: Arc<RateLimitState>, max_idle: Duration) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300));
        loop {
            interval.tick().await;
            let now = Instant::now();
            let mut buckets = rate_limiter.buckets.lock().expect("mutex not poisoned");
            buckets.retain(|_, bucket| {
                now.duration_since(bucket.last_refill) < max_idle
            });
        }
    });
}
