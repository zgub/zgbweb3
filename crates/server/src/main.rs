//! zgbweb server (placeholder).
//!
//! Today it only serves the static site built by `trunk build --release` from `dist/`.
//! It exists so the deployment shape is settled early: one binary behind a reverse
//! proxy. Accounts, sessions and server-side saves come later (see docs/DECISIONS.md D-008).
//!
//! Usage: `cargo run -p zgbweb-server -- [dist-dir] [addr]`
//! defaults: `dist`, `127.0.0.1:8090`

use axum::{Router, http::HeaderValue, routing::get};
use tower_http::{
    compression::CompressionLayer, services::ServeDir, set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .init();

    let mut args = std::env::args().skip(1);
    let dist = args.next().unwrap_or_else(|| "dist".into());
    let addr = args.next().unwrap_or_else(|| "127.0.0.1:8090".into());

    // Topic routing is in the URL hash (#/topic), so every path is either a real file
    // or the index page.
    let static_files = ServeDir::new(&dist).append_index_html_on_directories(true);

    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .fallback_service(static_files)
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("serving {dist} on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
}
