use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn get() -> impl Responder {
    let res = json!({
        "success": true,
        "data": {
            "message": "OK"
        },
    });

    HttpResponse::Ok().json(res)
}
