use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
    error::KafkaError
};
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: String, topic: String) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        KafkaProducer {
            producer,
            topic,
        }
    }

    pub async fn produce(&self, key: &str, payload: &str) -> Result<(), KafkaError> {
        let topic = self.topic.clone();
        let record = FutureRecord::to(&topic)
            .payload(payload)
            .key(key);

        self.producer.send(record, Duration::from_secs(0)).await
            .map_err(|(e, _)| e)?; // Extract KafkaError from the result
        Ok(())
    }
}