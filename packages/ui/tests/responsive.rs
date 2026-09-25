// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Responsive catalog verification for `montrs-ui`.
//!
//! Asserts that core components are responsive-safe out of the box — across
//! all standard devices, down to 320px minimum mobile width, with no
//! horizontal overflow.

use leptos::prelude::*;
use montrs_test::prelude::*;
use montrs_ui::components::*;

macro_rules! assert_component_responsive {
    ($name:literal, $expr:expr) => {{
        let owner = leptos::prelude::Owner::new();
        let html = owner.with(|| ($expr).to_html());
        let check = ResponsiveCheck::new(&html);
        let violations = check.sweep(320.0..=1280.0, 32.0);
        assert!(
            violations.is_empty(),
            "component {} failed responsive check:\n{}",
            $name,
            violations
                .iter()
                .map(ResponsiveReport::render)
                .collect::<Vec<_>>()
                .join("\n")
        );
    }};
}

#[test]
fn button_is_responsive() {
    assert_component_responsive!(
        "Button",
        view! { <button::Button>"Click"</button::Button> }
    );
}

#[test]
fn badge_is_responsive() {
    assert_component_responsive!(
        "Badge",
        view! { <badge::Badge>"Badge"</badge::Badge> }
    );
}

#[test]
fn alert_is_responsive() {
    assert_component_responsive!(
        "Alert",
        view! {
            <alert::Alert>
                <alert::AlertTitle>"Heads up"</alert::AlertTitle>
                <alert::AlertDescription>"Something happened"</alert::AlertDescription>
            </alert::Alert>
        }
    );
}

#[test]
fn card_is_responsive() {
    assert_component_responsive!(
        "Card",
        view! {
            <card::Card class="w-full">
                <card::CardHeader>
                    <card::CardTitle>"Title"</card::CardTitle>
                    <card::CardDescription>"Desc"</card::CardDescription>
                </card::CardHeader>
                <card::CardContent>
                    <p>"Content"</p>
                </card::CardContent>
            </card::Card>
        }
    );
}

#[test]
fn input_is_responsive() {
    assert_component_responsive!(
        "Input",
        view! { <input::Input placeholder="Search..." /> }
    );
}

#[test]
fn separator_is_responsive() {
    assert_component_responsive!(
        "Separator",
        view! { <separator::Separator /> }
    );
}

#[test]
fn skeleton_is_responsive() {
    assert_component_responsive!(
        "Skeleton",
        view! { <skeleton::Skeleton class="h-4 w-full" /> }
    );
}

#[test]
fn progress_is_responsive() {
    assert_component_responsive!(
        "Progress",
        view! { <progress::Progress value=50.0 /> }
    );
}

#[test]
fn standard_devices_suite() {
    let owner = leptos::prelude::Owner::new();
    let html = owner.with(|| {
        view! {
            <div class="flex flex-col gap-4 w-full p-4">
                <card::Card class="w-full">
                    <card::CardHeader>
                        <card::CardTitle>"Responsive Card"</card::CardTitle>
                    </card::CardHeader>
                    <card::CardContent>
                        <button::Button>"Action"</button::Button>
                    </card::CardContent>
                </card::Card>
            </div>
        }
        .to_html()
    });
    ResponsiveCheck::new(&html).assert_responsive();
}
