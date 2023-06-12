use actix_web::{get, Responder};
use actix_web::web::Json;
use serde_json::json;

#[get("/")]
pub async fn home() -> impl Responder {
    Json(json!{
        {
            "app": env!("CARGO_PKG_NAME"),
            "version": env!("CARGO_PKG_VERSION")
        }
    })
}