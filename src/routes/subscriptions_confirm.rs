use actix_web::{get, HttpResponse};



#[get("/subscriptions/confirm")]
pub async fn confirm() -> HttpResponse {
    HttpResponse::Ok().finish()
}
