
use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::PgPool;

use crate::routes::{health_check,subscribe};



pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new().app_data(connection.clone())
            .wrap(Logger::default())
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
