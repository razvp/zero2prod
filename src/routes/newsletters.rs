use actix_web::{post, web, HttpResponse, ResponseError};
use anyhow::Context;
use reqwest::StatusCode;
use sqlx::PgPool;
use tracing::instrument;

use crate::{domain::SubscriberEmail, email_client::EmailClient, routes::error_chain_fmt};

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
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, PublishError> {
    let _subscribers = get_confirmed_subscribers(&pool).await?;
    for subscriber in _subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(
                        // clone here so we use it in .with_context()
                        &subscriber.email, 
                        &body.title,
                        &body.content.html,
                        &body.content.text,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter isssue to: {}", subscriber.email)
                    })?;

            },
            Err(error) => {
                tracing::warn!(
                    // record error chain as structured field
                    error.cause_chain = ?error,
                    "Skipping a confirmed subscriber. \
                    Their stored contact details are invalid",
                );
            },
        }}

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
    email: SubscriberEmail,
}

#[instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let rows = sqlx::query!(
        "SELECT email FROM subscriptions WHERE status = 'confirmed'"
    )
    .fetch_all(pool)
    .await?;

    let confirmed_subscribers =
        rows.into_iter()
            .map(|r| match SubscriberEmail::parse(r.email) {
                Ok(email) => Ok(ConfirmedSubscriber { email }),
                Err(error) => Err(anyhow::anyhow!(error)),
            }).collect();

    Ok(confirmed_subscribers)
}
