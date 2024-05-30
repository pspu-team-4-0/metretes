use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CustomMessage {
    name: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KafkaMessage {
    action: Action,
    message_id: i32,
    data: Option<CustomMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Action {
    Create,
    Update,
    Delete,
}

use std::sync::Arc;
use rdkafka::{consumer::{Consumer, StreamConsumer}, Message};

#[allow(dead_code)]
pub async fn kafka_consumer_task(con: Arc<StreamConsumer>, db: sqlx::PgPool) {
    con.subscribe(&["messages"]).expect("Failed to subscribe to topics");

    tracing::info!("Starting the consumer loop...");

    loop {
        match con.recv().await {
            Err(e) => tracing::warn!("Kafka error: {}", e),
            Ok(m) => {
                let Some(payload) = m.payload() else {
                    tracing::error!("Could not find a payload :( for message in {}", m.topic());
                    continue;
                };

                // here we use `from_slice()` as we initally send it as &[u8]
                let message: KafkaMessage = match serde_json::from_slice(payload) {
                    Ok(res) => res,
                    Err(e) => {
                        // if there is a deserialization error, print an error
                        // and go to the next loop iteration
                        tracing::error!("Deserialization error: {e}");
                        continue;
                    }
                };

                // print out our payload
                tracing::info!("Got payload: {message:?}");

                let _ = con
                    .store_offset_from_message(&m)
                    .inspect_err(|e| tracing::warn!("Error while storing offset: {}", e));
            }
        };
    }
}