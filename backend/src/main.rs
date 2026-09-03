use std::{net::SocketAddr, sync::Arc, time::Duration};

use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use ajo_backend::{
    config::Config,
    db,
    middleware::rate_limit::{
        cleanup_old_buckets, cleanup_sliding_window, RateLimitState, SlidingWindowLimiter,
    },
    routes::{build_router, health::init_start_time},
    services::stellar_service::StellarService,
    AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ajo_backend=debug,tower_http=info,sqlx=warn".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(true)
                .with_thread_ids(false),
        )
        .init();

    let config = Arc::new(Config::from_env()?);
    tracing::info!(version = %config.app_version, "Starting Ajo Backend");

    let db = db::create_pool(&config.database_url).await?;
    db::run_migrations(&db).await?;

    let stellar = Arc::new(StellarService::new(
        config.horizon_url.clone(),
        config.soroban_rpc_url.clone(),
        config.contract_id.clone(),
    ));

    let rate_limiter = Arc::new(RateLimitState::new(100.0));
    cleanup_old_buckets(rate_limiter.clone(), Duration::from_secs(600));

    let auth_rate_limiter = Arc::new(SlidingWindowLimiter::new());
    cleanup_sliding_window(auth_rate_limiter.clone(), Duration::from_secs(900));

    init_start_time();

    let state = AppState {
        db,
        config: config.clone(),
        stellar,
        rate_limiter,
        auth_rate_limiter,
    };

    // Missed-contribution detection — runs every hour
    {
        let pool = state.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3600));
            loop {
                interval.tick().await;
                match ajo_backend::services::contribution_service::ContributionService::detect_missed(&pool).await {
                    Ok(n) if n > 0 => tracing::info!(count = n, "Missed contributions detected"),
                    Err(e) => tracing::error!(error = %e, "Missed contribution scan failed"),
                    _ => {}
                }
            }
        });
    }

    // Upcoming-payout reminders — runs every 6 hours
    {
        let pool = state.db.clone();
        let config = config.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(6 * 3600));
            loop {
                interval.tick().await;
                match ajo_backend::services::notification_service::NotificationService::send_due_reminders(&pool, &config).await {
                    Ok(n) if n > 0 => tracing::info!(count = n, "Payout reminders sent"),
                    Err(e) => tracing::error!(error = %e, "Payout reminder scan failed"),
                    _ => {}
                }
            }
        });
    }

    let addr: SocketAddr = config.bind_address().parse()?;
    let listener = TcpListener::bind(addr).await?;
    tracing::info!(address = %addr, "Listening");

    let router = build_router(state);
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
