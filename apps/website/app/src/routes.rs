// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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

pub struct FoundationsView;
impl RouteView for FoundationsView {
    fn render(&self) -> impl IntoView {
        view! { <crate::pages::Foundations /> }
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
view_route! { FoundationsRoute, "/foundations", FoundationsView }

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
        router.register(FoundationsRoute);
    }
}
