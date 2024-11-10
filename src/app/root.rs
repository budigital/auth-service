use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn get() -> impl Responder {
    let response = json!({
        "success": true,
        "data": {
            "message": "Hello World!"
        },
    });

    HttpResponse::Ok().json(response)
}
