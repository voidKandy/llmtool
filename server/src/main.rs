use std::sync::LazyLock;

use trace::TRACING;

pub mod api;
pub mod handlers;
pub mod services;
pub mod trace;

type MainErr = Box<dyn std::error::Error + Send + Sync + 'static>;
type MainResult<T> = std::result::Result<T, MainErr>;

#[macro_export]
macro_rules! other_err {
    ($($arg:tt)*) => ({
        Into::<crate::MainErr>::into(std::io::Error::other(format!($($arg)*)))
    });
}

#[tokio::main]
async fn main() {
    LazyLock::force(&TRACING);
    tracing::info!("TRACING INITIALIZED");
    let app = handlers::App::init().await;
    app.run().await.unwrap();
}
