
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FormData {
    name: String,
    email: String,
}


#[post("/subscribe")]
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    dbg!(&form);
    HttpResponse::Ok().finish()
}
