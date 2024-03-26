
use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

use crate::email_client::EmailClient;
use crate::routes::health_check_endpoint;
use crate::routes::subscribe;

pub fn run(listener: TcpListener, db_pool: PgPool, email_client: EmailClient) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(health_check_endpoint)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
