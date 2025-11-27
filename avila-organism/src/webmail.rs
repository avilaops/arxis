//! Interface webmail

use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Ávila Webmail - Coming Soon!" }))
        .route("/inbox", get(inbox))
        .route("/compose", get(compose))
}

async fn inbox() -> &'static str {
    "Inbox - TODO"
}

async fn compose() -> &'static str {
    "Compose - TODO"
}
