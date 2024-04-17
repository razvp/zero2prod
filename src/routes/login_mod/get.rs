use actix_web::{get, http::header::ContentType, web, HttpResponse};
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;

use crate::startup::HmacSecret;

#[derive(serde::Deserialize)]
struct QueryParams {
    error: String,
    tag: String,
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;
        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));
        let mut mac =
            Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes()).unwrap();
        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}

#[get("/login")]
pub async fn login_form(
    query: Option<web::Query<QueryParams>>,
    secret: web::Data<HmacSecret>,
) -> HttpResponse {
    let error_html = match query {
        None => "".into(),
        Some(query) => match query.0.verify(&secret) {
            Ok(error) => format!("<p><i>{}</i></p>", htmlescape::encode_minimal(&error)),
            Err(e) => {
                tracing::warn!(
                    error.message = %e,
                        error.cause_chain = ?e,
                        "Failed to verify query parameters using the HMAC tag"
                );
                "".into()
            }
        },
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        // .body(include_str!("login.html"))
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <!-- This is equivalent to a HTTP header -->
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Home</title>
  </head>
  <body>
    {error_html}
    <form action="/login" method="post">
      <label>Username
        <input type="text" name="username" placeholder="Enter Username">
      </label>
      <label>Password
        <input type="password" name="password" placeholder="Enter Password">
      </label>
      <button type="submit">Login</button>
    </form>
  </body>
</html>"#
        ))
}
