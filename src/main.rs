use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let db_pool = PgPool::connect_lazy(&connection_string.expose_secret())
        .expect("failed to create Postgres connection pool");
    let address = format!("{}:{}",configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address).expect("failed to bind");
    
    tracing::info!("STARTING APP on: `{}`", configuration.application.host);
    run(listener, db_pool)?.await
}

