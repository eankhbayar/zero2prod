//! src/main.rs

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to load configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let address = format!("0.0.0.0:{}", configuration.application_port);

    let listener = TcpListener::bind(&address)?;
    run(listener, connection_pool)?.await
}
