
use std::net::TcpListener;
use actix_web::{dev::Server, App, HttpServer};

use crate::{configuration::get_configuration, routes::{health_check,subscribe}};

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) connection_string: String,
}


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!("test");
    let configuration = get_configuration().expect("failed to read config");
    let connection_string = configuration.database.connection_string();

    let server = HttpServer::new(move || {
        let state = State {connection_string : connection_string.clone()};
        App::new().app_data(state)
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
