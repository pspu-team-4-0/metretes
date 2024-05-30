mod config;
mod http;
mod services;


use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::Context;

use axum::extract::State;
use http::AppContext;
use rdkafka::{config::RDKafkaLogLevel, consumer::StreamConsumer, producer::FutureProducer, ClientConfig, consumer::Consumer};
// use services::kafka_consumer_task;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use clap::Parser;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "simply_chat=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&cfg.database_url)
        .await
        .context("Failed to establish postgres connection.")?;
    
    let listener = tokio::net::TcpListener::bind(&cfg.listen_on)
        .await
        .expect("Failed to bind");

    sqlx::migrate!().run(&db).await?;
    
    info!("Connecting to Minio...");
    
    let ctx = AppContext {
        cfg: Arc::new(cfg.clone()),
        fp_kafka: Arc::new(create_producer(&cfg.kafka_url)),
        sc_kafka: Arc::new(create_kafka_consumer(&cfg.kafka_url)),
        db,
    };
    
    let sc_kafka = ctx.sc_kafka.clone();
    let db = ctx.db.clone();

    let services = tokio::spawn(async move {
        // sc_kafka.subscribe();
        // kafka_consumer_task(sc_kafka, db).await;
    });

    let app = http::router(ctx);
    tracing::debug!("listening on {}", listener.local_addr().expect("Infallible"));
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await
        .expect("Http server error");
    Ok(())
}

fn create_producer(bootstrap_server: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("message.timeout.ms", "5000")
        .set("allow.auto.create.topics", "true")
        .create()
        .expect("Producer creation error")
}

fn create_kafka_consumer(bootstrap_server: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("group.id", "access-metretes")
        .set("bootstrap.servers", bootstrap_server)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed")
}