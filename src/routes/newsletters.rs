use actix_web::{post, web, HttpResponse, ResponseError};
use reqwest::StatusCode;
use sqlx::PgPool;
use tracing::instrument;

use crate::routes::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

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
pub async fn publish_newsletter(
    _data: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PublishError> {
    let _subscribers = get_confirmed_subscribers(&pool).await?;
    Ok(HttpResponse::Ok().finish())
}

impl ResponseError for PublishError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

struct ConfirmedSubscriber {
    email: String,
}

#[instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<ConfirmedSubscriber>, anyhow::Error> {
    let rows = sqlx::query_as!(
        ConfirmedSubscriber,
        "SELECT email FROM subscriptions WHERE status = 'confirmed'"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
