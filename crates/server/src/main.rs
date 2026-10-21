//! zgbweb3 server: thin on purpose (DECISIONS D-032).
//!
//! Today it serves the static site built by `trunk build --release` from `dist/`,
//! with the headers and refusals that keep crawlers out (D-021). Everything the
//! visitor sees is compiled into the wasm, including the texts; nothing here is
//! needed to read the site.
//!
//! `/api/…` is reserved for the first feature that stores data (an ideas inbox,
//! comments, saved state, accounts). Postgres arrives with that feature, not before.
//!
//! Usage: `cargo run -p zgbweb3-server -- [dist-dir] [addr]`
//! defaults: `dist`, `127.0.0.1:8090`

use axum::{
    Router,
    body::Body,
    extract::Request,
    http::{HeaderName, HeaderValue, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use tower_http::{
    compression::CompressionLayer, services::ServeDir, set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

/// Disallow everything for everyone. Only polite crawlers read this (D-021).
const ROBOTS_TXT: &str = "\
# zgub.net is shared by word and links, not by crawlers.
User-agent: *
Disallow: /
";

/// User agents refused outright with 403. Substrings, compared case-insensitively.
/// Keep in step with the ai.robots.txt project's list; only crawlers that say
/// who they are can be caught this way.
const BLOCKED_AGENTS: &[&str] = &[
    "gptbot",
    "chatgpt-user",
    "oai-searchbot",
    "claudebot",
    "claude-user",
    "claude-searchbot",
    "anthropic-ai",
    "ccbot",
    "perplexitybot",
    "perplexity-user",
    "bytespider",
    "amazonbot",
    "applebot",
    "meta-externalagent",
    "facebookbot",
    "diffbot",
    "cohere-ai",
    "timpibot",
    "imagesiftbot",
    "youbot",
    "duckassistbot",
    "petalbot",
];

fn is_blocked(user_agent: &str) -> bool {
    let ua = user_agent.to_ascii_lowercase();
    BLOCKED_AGENTS.iter().any(|b| ua.contains(b))
}

async fn refuse_crawlers(req: Request, next: Next) -> Response {
    let ua = req
        .headers()
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if is_blocked(ua) {
        tracing::info!(user_agent = ua, path = %req.uri().path(), "refused crawler");
        return (StatusCode::FORBIDDEN, "No crawling, please.\n").into_response();
    }
    next.run(req).await
}

/// Nothing behind `/api` yet. Answers clearly instead of falling through to the site.
async fn api_not_yet() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"error":"no api yet"}"#))
        .expect("static response")
}

fn header_layer(name: HeaderName, value: &'static str) -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(name, HeaderValue::from_static(value))
}

fn app(dist: &str) -> Router {
    // Page routing is in the URL hash (#/page), so every path is a real file or the index.
    let static_files = ServeDir::new(dist).append_index_html_on_directories(true);

    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/robots.txt", get(|| async { ROBOTS_TXT }))
        .route("/api", get(api_not_yet))
        .route("/api/{*rest}", get(api_not_yet))
        .fallback_service(static_files)
        .layer(middleware::from_fn(refuse_crawlers))
        .layer(CompressionLayer::new())
        .layer(header_layer(
            HeaderName::from_static("x-robots-tag"),
            "noindex, nofollow, noarchive, noimageindex",
        ))
        // Text-and-data-mining opt-out (W3C TDMRep).
        .layer(header_layer(
            HeaderName::from_static("tdm-reservation"),
            "1",
        ))
        .layer(header_layer(header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
        .layer(header_layer(
            header::REFERRER_POLICY,
            "strict-origin-when-cross-origin",
        ))
        .layer(TraceLayer::new_for_http())
}

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

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("serving {dist} on http://{addr}");
    axum::serve(listener, app(&dist))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crawler_agents_are_caught_and_browsers_are_not() {
        assert!(is_blocked(
            "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; GPTBot/1.2; +https://openai.com/gptbot)"
        ));
        assert!(is_blocked("CCBot/2.0 (https://commoncrawl.org/faq/)"));
        assert!(is_blocked("Mozilla/5.0 (compatible; ClaudeBot/1.0)"));
        assert!(!is_blocked(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:140.0) Gecko/20100101 Firefox/140.0"
        ));
        assert!(!is_blocked(""));
    }
}
