//! API server bootstrap — a minimal axum server using montrs-auth.
use montrs_auth::AuthConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let auth = montrs_auth::MontrsAuth::builder()
        .config(AuthConfig::new("change-me-32-char-minimum-secret!!").base_url("http://localhost:3000"))
        .database(Box::new(montrs_auth::database::MemoryDatabaseAdapter::new()))
        .build()
        .await?;

    let app = axum::Router::new()
        .merge(auth.axum_router())
        .route("/health", axum::routing::get(|| async { "ok" }));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("API server listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}