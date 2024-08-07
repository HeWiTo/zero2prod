use std::net::TcpListener;
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{
    BunyanFormattingLayer,
    JsonStorageLayer
};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    Registry
};
use tracing_log::LogTracer;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use env_logger::Env;
use zero2prod::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
