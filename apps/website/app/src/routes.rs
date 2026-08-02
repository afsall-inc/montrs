use leptos::prelude::*;
use montrs_core::*;
use async_trait::async_trait;

use crate::pages::*;

// ---------------------------------------------------------------------------
// RouteView wrappers for each page component
// ---------------------------------------------------------------------------

pub struct HomeView;
impl RouteView for HomeView {
    fn render(&self) -> impl IntoView {
        view! { <Home /> }
    }
}

pub struct IconsView;
impl RouteView for IconsView {
    fn render(&self) -> impl IntoView {
        view! { <crate::pages::Icons /> }
    }
}

pub struct ComponentsView;
impl RouteView for ComponentsView {
    fn render(&self) -> impl IntoView {
        view! { <crate::pages::Components /> }
    }
}

pub struct BlocksView;
impl RouteView for BlocksView {
    fn render(&self) -> impl IntoView {
        view! { <crate::pages::Blocks /> }
    }
}

// ---------------------------------------------------------------------------
// MontRS Routes (using view_route! macro)
// ---------------------------------------------------------------------------

view_route! { HomeRoute, "/", HomeView }
view_route! { IconsRoute, "/icons", IconsView }
view_route! { ComponentsRoute, "/components", ComponentsView }
view_route! { BlocksRoute, "/blocks", BlocksView }

// ---------------------------------------------------------------------------
// Website Plate
// ---------------------------------------------------------------------------

pub struct WebsitePlate;

#[async_trait]
impl<C: AppConfig + 'static> Plate<C> for WebsitePlate {
    fn name(&self) -> &'static str {
        "website"
    }

    fn description(&self) -> &'static str {
        "MontRS website — montrs.com"
    }

    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }

    async fn init(
        &self,
        _ctx: &mut PlateContext<C>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    fn register_routes(&self, router: &mut Router<C>) {
        router.register(HomeRoute);
        router.register(IconsRoute);
        router.register(ComponentsRoute);
        router.register(BlocksRoute);
    }
}