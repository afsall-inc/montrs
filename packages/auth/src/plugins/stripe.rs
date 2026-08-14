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

//! Stripe plugin — subscription management and webhook handling.
//! /subscription/list, /subscription/upgrade (stub), /stripe/webhook.
//! User metadata stores stripeCustomerId.

use crate::{AuthError, context::AuthState, plugin::AuthPlugin};
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};

/// Stripe plugin — subscription management.
pub struct StripePlugin {
    state: Option<AuthState>,
}

impl StripePlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for StripePlugin {
    fn default() -> Self {
        Self::new()
    }
}

fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    if let Some(v) = headers.get("authorization").and_then(|v| v.to_str().ok())
    {
        if let Some(t) = v.strip_prefix("Bearer ") {
            return Some(t.to_string());
        }
    }
    if let Some(v) = headers.get("cookie").and_then(|v| v.to_str().ok()) {
        for part in v.split(';') {
            let part = part.trim();
            if let Some(t) = part.strip_prefix("session=") {
                return Some(t.to_string());
            }
            if let Some(t) = part.strip_prefix("__montrs_session=") {
                return Some(t.to_string());
            }
        }
    }
    None
}

impl AuthPlugin for StripePlugin {
    fn name(&self) -> &'static str {
        "stripe"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("StripePlugin: state not set");
        Router::new()
            .route("/subscription/list", get(list_subscriptions))
            .route("/subscription/upgrade", post(upgrade_subscription))
            .route("/stripe/webhook", post(stripe_webhook))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionInfo {
    pub plan: String,
    pub status: String,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub current_period_end: Option<i64>,
    pub cancel_at_period_end: bool,
}

async fn list_subscriptions(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token =
        extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;
    let user = state
        .db
        .find_user_by_id(&session.user_id)
        .await?
        .ok_or_else(AuthError::user_not_found)?;

    let stripe_customer_id = user.metadata.get("stripeCustomerId").cloned();
    let sub_json: Option<Value> = state
        .db
        .plugin_get("stripe_subscription", &session.user_id)
        .await
        .ok()
        .flatten();

    let subscriptions = if let Some(s) = sub_json {
        vec![s]
    } else {
        vec![json!({
            "plan": "free",
            "status": "active",
            "stripeCustomerId": stripe_customer_id,
        })]
    };

    Ok(Json(json!({ "subscriptions": subscriptions })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeRequest {
    pub plan: Option<String>,
    pub price_id: Option<String>,
}

async fn upgrade_subscription(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<UpgradeRequest>,
) -> Result<Json<Value>, AuthError> {
    let token =
        extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state
        .session
        .validate(&token)
        .await?
        .ok_or_else(AuthError::invalid_session)?;

    let plan = req.plan.unwrap_or_else(|| "pro".into());
    // Stub: just store the subscription intent.
    state
        .db
        .plugin_set(
            "stripe_subscription",
            &session.user_id,
            json!({
                "plan": plan,
                "status": "pending",
                "updatedAt": chrono::Utc::now().to_rfc3339(),
            }),
        )
        .await
        .ok();

    Ok(Json(json!({
        "success": true,
        "message": "Subscription upgrade initiated. This is a stub — integrate Stripe Checkout for production.",
        "plan": plan,
        "url": format!("{}/upgrade?plan={plan}", state.config.base_url.trim_end_matches('/')),
    })))
}

#[derive(Debug, Deserialize)]
pub struct StripeWebhookEvent {
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub id: Option<String>,
    pub data: Option<serde_json::Value>,
    /// For raw body parsing.
    #[serde(default)]
    pub raw: String,
}

async fn stripe_webhook(
    State(state): State<AuthState>,
    body: String,
) -> Result<Json<Value>, AuthError> {
    // Parse the webhook event.
    let event: StripeWebhookEvent =
        serde_json::from_str(&body).unwrap_or(StripeWebhookEvent {
            event_type: None,
            id: None,
            data: None,
            raw: body.clone(),
        });

    let event_type =
        event.event_type.clone().unwrap_or_else(|| "unknown".into());

    // Store the webhook event for audit.
    let _ = state
        .db
        .plugin_set(
            "stripe_webhook",
            &uuid::Uuid::new_v4().to_string(),
            json!({
                "type": event_type,
                "receivedAt": chrono::Utc::now().to_rfc3339(),
                "raw": body,
            }),
        )
        .await;

    // Handle specific event types.
    match event_type.as_str() {
        "customer.subscription.created"
        | "customer.subscription.updated"
        | "customer.subscription.deleted"
        | "invoice.paid"
        | "invoice.payment_failed" => {
            if let Some(data) = &event.data {
                if let Some(object) = data.get("object") {
                    if let Some(customer) =
                        object.get("customer").and_then(|v| v.as_str())
                    {
                        // Store subscription info in user metadata.
                        let entries = state
                            .db
                            .plugin_list("stripe_customer")
                            .await
                            .unwrap_or_default();
                        for (key, val) in entries {
                            if val.as_str() == Some(customer) {
                                let _ = state
                                    .db
                                    .plugin_set("stripe_subscription", &key, json!({
                                        "plan": object.get("plan").and_then(|p| p.get("nickname")).and_then(|v| v.as_str()).unwrap_or("unknown"),
                                        "status": object.get("status").and_then(|v| v.as_str()).unwrap_or("unknown"),
                                        "stripeCustomerId": customer,
                                        "stripeSubscriptionId": object.get("id").and_then(|v| v.as_str()),
                                        "currentPeriodEnd": object.get("current_period_end"),
                                        "cancelAtPeriodEnd": object.get("cancel_at_period_end").and_then(|v| v.as_bool()).unwrap_or(false),
                                    }))
                                    .await;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    Ok(Json(json!({ "received": true, "type": event_type })))
}
