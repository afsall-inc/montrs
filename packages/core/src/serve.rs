//! montrs-core/src/serve.rs: SSR server entry point.
//!
//! Provides `montrs_serve` — a single function call that replaces the ~40 lines
//! of boilerplate in every app's `main.rs`. Creates its own single-threaded
//! tokio runtime with LocalSet for Leptos SSR compatibility.

#[cfg(feature = "ssr")]
use crate::{AppConfig, Router};

/// Start an Axum SSR server backed by a MontRS Router.
///
/// Creates a single-threaded tokio runtime with a `LocalSet` to support
/// Leptos `spawn_local` during SSR rendering. Reads `MONTRS_SITE_ADDR`,
/// `MONTRS_SITE_ROOT`, and `MONTRS_SITE_PKG_DIR` from the environment.
///
/// # Example
/// ```rust,ignore
/// #[cfg(feature = "ssr")]
/// fn main() {
///     tracing_subscriber::fmt().with_env_filter("info").init();
///     let spec = app::build_spec();
///     montrs_core::serve::montrs_serve(spec.router, || view! { <app::App /> })
///         .unwrap();
/// }
/// ```
#[cfg(feature = "ssr")]
pub fn montrs_serve<C, F, IV>(
    router: Router<C>,
    app_fn: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    C: AppConfig + 'static,
    F: Fn() -> IV + Clone + Send + Sync + 'static,
    IV: leptos::prelude::IntoView + 'static,
{
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(async move { serve_inner(router, app_fn).await })
}

#[cfg(feature = "ssr")]
async fn serve_inner<C, F, IV>(
    router: Router<C>,
    app_fn: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    C: AppConfig + 'static,
    F: Fn() -> IV + Clone + Send + Sync + 'static,
    IV: leptos::prelude::IntoView + 'static,
{
    use axum::Router as AxumRouter;
    use leptos::prelude::*;
    use leptos_axum::LeptosRoutes;
    use tokio::task::LocalSet;
    use tower_http::services::ServeDir;

    if std::env::var("LEPTOS_OUTPUT_NAME").is_err() {
        unsafe { std::env::set_var("LEPTOS_OUTPUT_NAME", "website") };
    }

    let conf = get_configuration(None).unwrap();
    let addr = std::env::var("MONTRS_SITE_ADDR")
        .unwrap_or_else(|_| conf.leptos_options.site_addr.to_string());
    let site_root = std::env::var("MONTRS_SITE_ROOT")
        .unwrap_or_else(|_| conf.leptos_options.site_root.to_string());

    let axum_routes = router.to_axum_route_listings();

    let app = AxumRouter::new()
        .leptos_routes_with_context(
            &conf.leptos_options,
            axum_routes,
            {
                let r = router.clone();
                move || {
                    provide_context(r.clone());
                }
            },
            app_fn,
        )
        .fallback_service(ServeDir::new(&site_root))
        .with_state(conf.leptos_options);

    let (host, port_str) = addr.rsplit_once(':').unwrap_or((&addr, "3000"));
    let mut port: u16 = port_str.parse().unwrap_or(3000);
    for _ in 0..100 {
        let bind_addr = format!("{host}:{port}");
        if let Ok(listener) = tokio::net::TcpListener::bind(&bind_addr).await {
            tracing::info!("listening on http://{host}:{port}");
            let local = LocalSet::new();
            let _guard = local.enter();
            axum::serve(listener, app.clone().into_make_service()).await?;
            return Ok(());
        }
        port += 1;
    }
    Err("Could not bind to any port in range".into())
}
