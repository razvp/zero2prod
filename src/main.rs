use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("failed to bind");
    let connection_string = configuration.database.connection_string();
    let db_pool = PgPool::connect(&connection_string)
        .await
        .expect("failed to connect do DB");
    run(listener, db_pool)?.await
}
