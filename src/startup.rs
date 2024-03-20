
use std::net::TcpListener;
use actix_web::{dev::Server, App, HttpServer};

use crate::routes::{health_check,subscribe};

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
