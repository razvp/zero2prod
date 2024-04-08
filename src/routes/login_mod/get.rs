use actix_web::{get, http::header::ContentType, web, HttpResponse};

#[derive(serde::Deserialize)]
struct QueryParams {
    error: Option<String>
}



#[get("/login")]
pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let error_html = match query.0.error {
        None => "".into(),
        Some(error_message) => format!("<p><i>{error_message}</i></p>"),
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
