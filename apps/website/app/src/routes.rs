use crate::pages::*;
use async_trait::async_trait;
use leptos::prelude::*;
use montrs_core::*;

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

pub struct MotionView;
impl RouteView for MotionView {
    fn render(&self) -> impl IntoView {
        view! { <crate::pages::Motion /> }
    }
}

pub struct AnimatedIconsView;
impl RouteView for AnimatedIconsView {
    fn render(&self) -> impl IntoView {
        view! { <crate::pages::AnimatedIcons /> }
    }
}

// ---------------------------------------------------------------------------
// MontRS Routes (using view_route! macro)
// ---------------------------------------------------------------------------

view_route! { HomeRoute, "/", HomeView }
view_route! { IconsRoute, "/icons", IconsView }
view_route! { ComponentsRoute, "/components", ComponentsView }
view_route! { BlocksRoute, "/blocks", BlocksView }
view_route! { MotionRoute, "/motion", MotionView }
view_route! { AnimatedIconsRoute, "/animated-icons", AnimatedIconsView }

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
        router.register(MotionRoute);
        router.register(AnimatedIconsRoute);
    }
}
