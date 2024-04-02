use actix_web::{get, web, HttpResponse};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Parameters {
    subscription_token: String,
}



#[tracing::instrument(
    name = "Confirm a pending subscriber",
    skip(_parameters)
)]
#[get("/subscriptions/confirm")]
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    dbg!(&_parameters);
    HttpResponse::Ok().finish()
}
