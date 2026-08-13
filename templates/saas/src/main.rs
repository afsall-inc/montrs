//! SaaS API server — auth, i18n, and services wired together.
use montrs_auth::AuthConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let auth = montrs_auth::MontrsAuth::builder()
        .config(AuthConfig::new("change-me-32-char-minimum-secret!!").base_url("http://localhost:3000"))
        .database(Box::new(montrs_auth::database::MemoryDatabaseAdapter::new()))
        .plugin(Box::new(montrs_auth::plugins::TwoFactorPlugin::new()))
        .plugin(Box::new(montrs_auth::plugins::OrganizationPlugin::new()))
        .plugin(Box::new(montrs_auth::plugins::AdminPlugin::new()))
        .plugin(Box::new(montrs_auth::plugins::ApiKeyPlugin::new()))
        .build()
        .await?;

    let app = axum::Router::new()
        .merge(auth.axum_router())
        .route("/health", axum::routing::get(|| async { "ok" }));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("SaaS server listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}