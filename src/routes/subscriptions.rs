use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;

#[derive(Deserialize, Debug)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(pool: web::Data<PgPool>, form: web::Form<FormData>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    tracing::info!(
        "request_id {request_id} - Adding '{}' '{}' as a new subscriber",
        form.email,
        form.name
    );
    tracing::info!("request_id {request_id} - Saving new subscriber details in the database");
    let name = &form.name;
    let email = &form.email;
    let subscribed_at = Utc::now();
    let id = uuid::Uuid::new_v4();

    let insert = sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)",
        id,
        email,
        name,
        subscribed_at
    )
    .execute(pool.as_ref())
    .await;

    match insert {
        Ok(_) => {
            tracing::info!("request_id {request_id} - New subscriber details saved.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id {request_id} - Failed to execute query: {e}");
            HttpResponse::BadRequest().body("email exists")
        }
    }
}
