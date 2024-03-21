use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(pool: web::Data<PgPool>, form: web::Form<FormData>) -> HttpResponse {
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {e}");
            HttpResponse::BadRequest().body("email exists")
        }
    }
}
