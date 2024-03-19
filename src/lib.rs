use actix_web::dev::Server;
use actix_web::{get, post, web, HttpResponse};
use actix_web::{App, HttpServer, Responder};
use std::net::TcpListener;
use serde::Deserialize;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize, Debug)]
struct FormData {
    name: String,
    email: String,
}


#[post("/subscribe")]
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    dbg!(&form);
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!("test");
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
