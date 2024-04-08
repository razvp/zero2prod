use actix_web::{get, HttpResponse};


#[get("/")]
pub async fn home() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("home.html"))
}
