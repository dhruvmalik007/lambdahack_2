use tracing_subscriber::{filter, util::SubscriberInitExt};

pub fn setup() {
    // Read the env
    dotenv::dotenv().expect("failed to read .env file");

    // Set up the tracing
    let filter = format!(
        "mining={}",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string())
    );
    let filter = filter::EnvFilter::new(filter);
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish()
        .try_init()
        .expect("failed to initialize tracing subscriber");
}
