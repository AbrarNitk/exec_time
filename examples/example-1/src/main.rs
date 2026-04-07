use exec_time::exec_time;
use std::time::Duration;

#[exec_time(name = "example.login", level = "info", unit = "ms")]
async fn login() -> &'static str {
    tokio::time::sleep(Duration::from_millis(25)).await;
    "ok"
}

#[exec_time(name = "example.query", level = "debug", warn_over = "40ms")]
async fn query_db() -> usize {
    tokio::time::sleep(Duration::from_millis(50)).await;
    42
}

#[exec_time(backend = "stdout", name = "example.stdout")]
async fn stdout_override() -> &'static str {
    tokio::time::sleep(Duration::from_millis(10)).await;
    "stdout"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .with_max_level(tracing::Level::TRACE)
        .init();

    println!("login={}", login().await);
    println!("rows={}", query_db().await);
    println!("override={}", stdout_override().await);
}
