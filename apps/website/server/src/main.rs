use axum::Router;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let conf = get_configuration(None).unwrap();
    let addr = std::env::var("LEPTOS_SITE_ADDR")
        .unwrap_or_else(|_| conf.leptos_options.site_addr.to_string());
    let site_root = std::env::var("LEPTOS_SITE_ROOT")
        .unwrap_or_else(|_| conf.leptos_options.site_root.to_string_lossy().to_string());
    let pkg_dir = std::env::var("LEPTOS_SITE_PKG_DIR")
        .unwrap_or_else(|_| "pkg".to_string());

    let routes = generate_route_list(website::App);

    let app = Router::new()
        .leptos_routes(&conf.leptos_options, routes, website::App)
        .fallback_service(ServeDir::new(site_root))
        .with_state(conf.leptos_options);

    // Auto-increment port if addr is in use
    let (host, port_str) = match addr.rsplit_once(':') {
        Some((h, p)) => (h, p),
        None => (addr.as_str(), "3000"),
    };
    let mut port: u16 = port_str.parse().unwrap_or(3000);
    let mut listener = None;
    for _ in 0..100 {
        let bind_addr = format!("{host}:{port}");
        match tokio::net::TcpListener::bind(bind_addr).await {
            Ok(l) => {
                listener = Some(l);
                break;
            }
            Err(_) => port += 1,
        }
    }

    let listener = listener.expect("Could not bind to any port");
    tracing::info!("listening on http://{host}:{port}");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
