use crate::startup::State;
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::types::chrono::Local;
use sqlx::types::Uuid;
use sqlx::Connection;
use sqlx::PgConnection;

#[derive(Deserialize, Debug)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(req: HttpRequest, form: web::Form<FormData>) -> HttpResponse {
    let connection_string = &req.app_data::<State>().unwrap().connection_string;
    let mut connection = PgConnection::connect(connection_string)
        .await
        .expect("failed to connect to DB");
    let name = &form.name;
    let email = &form.email;
    let subscribed_at = Local::now();
    let id = uuid::Uuid::new_v4();

    let insert = sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)",
        id,
        email,
        name,
        subscribed_at
    )
    .execute(&mut connection)
    .await;

    match insert {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("error: {e:?}");
            HttpResponse::BadRequest().body("account exists")
        }
    }
}
