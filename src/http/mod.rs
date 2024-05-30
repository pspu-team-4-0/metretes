mod outgoing;
mod auth;
mod error;
mod models;

use std::{sync::Arc, time::Duration};

use axum::{extract::State, http::header::AUTHORIZATION};
use error::HttpError;
use rdkafka::{consumer::StreamConsumer, producer::FutureProducer};
use sqlx::PgPool;
use tower_http::{catch_panic::CatchPanicLayer, compression::CompressionLayer, sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer, trace::{DefaultMakeSpan, TraceLayer}};

use crate::config::Config;

pub type Result<T, E = HttpError> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct AppContext {
    pub cfg: Arc<Config>,
    pub fp_kafka: Arc<FutureProducer>,
    pub sc_kafka: Arc<StreamConsumer>,
    pub db: PgPool
}

pub fn router(ctx: AppContext) -> axum::Router {
    axum::Router::new()
        .nest("/auth", auth::router())
        .layer((
            SetSensitiveHeadersLayer::new([AUTHORIZATION]),
            CompressionLayer::new(),
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
            TimeoutLayer::new(Duration::from_secs(30)),
            CatchPanicLayer::new(),
        ))
        .with_state(ctx)

}