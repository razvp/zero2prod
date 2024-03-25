use actix_web::{get, HttpResponse, Responder};


#[get("/health_check")]
async fn health_check_endpoint() -> impl Responder {
    tracing::info!("health_check hit");
    HttpResponse::Ok()
}
