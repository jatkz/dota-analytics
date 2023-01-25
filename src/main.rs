use std::net::TcpListener;
use dota_analytics::{configuration::{get_configuration}, startup::run, telemetry::{init_subscriber, get_subscriber}};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("dota_analytics".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to get configuration");
    let db_pool = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind TcpListener");
    run(listener, db_pool)?.await?;

    Ok(())
}
