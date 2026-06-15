pub mod auth;
pub mod contributions;
pub mod groups;
pub mod health;
pub mod members;

use axum::{
    http::{header, HeaderValue, Method},
    middleware, routing::get, Router,
};
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{
    middleware::{auth::auth_middleware, rate_limit::rate_limit_middleware, request_id::request_id_middleware},
    AppState,
};

pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            state
                .config
                .frontend_url
                .parse::<HeaderValue>()
                .expect("valid CORS origin"),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    let public = Router::new()
        .route("/health", get(health::health_check))
        .nest("/auth", auth::public_router())
        .route("/groups", get(groups::list_groups_handler))
        .route("/groups/:id", get(groups::get_group_handler))
        .route("/groups/:id/members", get(groups::list_group_members_handler))
        .route("/groups/:id/payout-schedule", get(groups::get_payout_schedule_handler))
        .route("/groups/:group_id/contributions", get(contributions::list_contributions_handler));

    let protected = Router::new()
        .route("/groups", axum::routing::post(groups::create_group_handler))
        .route("/groups/:id/join", axum::routing::post(groups::join_group_handler))
        .route("/groups/:id/activate", axum::routing::patch(groups::activate_group_handler))
        .route("/contributions", axum::routing::post(contributions::record_contribution_handler))
        .route("/groups/:group_id/me", get(members::get_my_membership_handler))
        .route("/groups/:group_id/members/:member_id", axum::routing::delete(members::remove_member_handler))
        .route("/auth/me", get(auth::me_handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    let api_v1 = Router::new().merge(public).merge(protected);

    Router::new()
        .nest("/api/v1", api_v1)
        .layer(middleware::from_fn(request_id_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(cors)
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .with_state(state)
}
