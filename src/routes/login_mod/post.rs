use actix_web::{post, HttpResponse};
use reqwest::header::LOCATION;

use actix_web::web;
use secrecy::SecretString;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: SecretString,
}

#[post("/login")]
pub async fn login(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
