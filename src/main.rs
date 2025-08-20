//! src/main.rs

use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to load configuration");
    let address = format!("0.0.0.0:{}", configuration.application_port);

    let listener = TcpListener::bind(&address)?;

    run(listener)?.await
}
