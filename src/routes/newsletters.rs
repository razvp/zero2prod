use actix_web::{post, web, HttpResponse};


#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

// this needs to send emails to all subscribers that have a `confirmed` status
// 0. get newsletter issue details from incoming API call
// 1. get all subscribers with `confirmed` status
// 2. send email for each one, or maybe bulk?
#[post("/newsletters")]
pub async fn publish_newsletter(_data: web::Json<BodyData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
