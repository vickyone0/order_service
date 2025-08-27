use actix_web::{web, HttpResponse, Responder, Route};
use serde_json::json;
use uuid::Uuid;

use crate::{kafka_producer::KafkaProducer, models::Order, repository::create_order};

// Configure routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/orders")
            .route(web::post().to(create_order_handler))
    );
}

// Handler for creating an order
async fn create_order_handler(
    order: web::Json<Order>,
    kafka_producer: web::Data<KafkaProducer>,
) -> impl Responder {
    let mut new_order = order.into_inner();
    new_order.id = Some(Uuid::new_v4()); // Assign a UUID

    // Persist to DB (replace with your actual DB logic)
    let db_result = create_order(&new_order).await;

    match db_result {
        Ok(_) => {
             // Serialize the order to JSON
            let order_json = match serde_json::to_string(&new_order) {
                Ok(json_string) => json_string,
                Err(e) => {
                    eprintln!("Failed to serialize order: {}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };
            // Produce to Kafka
            match kafka_producer.produce("order_created", &order_json).await {
                Ok(_) => HttpResponse::Created().json(&new_order),
                Err(e) => {
                    eprintln!("Failed to produce Kafka message: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to save order to DB: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}