use actix_web::{App, HttpServer};
mod api;
mod kafka_producer;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    println!("CustomSmartPointers created.");
    // Initialize Kafka producer (you might want to do this once globally)
    let kafka_producer = kafka_producer::KafkaProducer::new("localhost:9092".to_string(), "orders".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(kafka_producer.clone())) // Share the producer
            .configure(api::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}