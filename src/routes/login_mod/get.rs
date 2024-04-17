use actix_web::{get, http::header::ContentType, HttpRequest, HttpResponse};

#[get("/login")]
pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let error_html = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie.value())
        }
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
