//! Agent Auth plugin — register and manage agent tokens for programmatic access.
//! /agent/register, /agent/token, /agent/capability — store agents in plugin_store.

use crate::context::AuthState;
use crate::plugin::AuthPlugin;
use crate::AuthError;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// A registered agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub token_hash: String,
    pub capabilities: Vec<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

/// Agent Auth plugin — manage agents for programmatic access.
pub struct AgentAuthPlugin {
    state: Option<AuthState>,
}

impl AgentAuthPlugin {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Default for AgentAuthPlugin {
    fn default() -> Self {
        Self::new()
    }
}

fn extract_token(headers: &axum::http::HeaderMap) -> Option<String> {
    if let Some(v) = headers.get("authorization").and_then(|v| v.to_str().ok()) {
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

impl AuthPlugin for AgentAuthPlugin {
    fn name(&self) -> &'static str {
        "agent_auth"
    }

    fn on_build(&mut self, state: &AuthState) -> Result<(), AuthError> {
        self.state = Some(state.clone());
        Ok(())
    }

    fn router(&self) -> Router {
        let state = self.state.clone().expect("AgentAuthPlugin: state not set");
        Router::new()
            .route("/agent/register", post(register_agent))
            .route("/agent/token", post(get_token))
            .route("/agent/capability", post(check_capability))
            .route("/agent/list", get(list_agents))
            .route("/agent/revoke", post(revoke_agent))
            .with_state(state)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterAgentRequest {
    pub name: String,
    pub capabilities: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, String>>,
}

async fn register_agent(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<RegisterAgentRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;

    if req.name.is_empty() {
        return Err(AuthError::missing_field("name"));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let raw_token = crate::utils::generate_token();
    let token_hash = sha256_hex(&raw_token);

    let agent = Agent {
        id: id.clone(),
        name: req.name,
        user_id: session.user_id.clone(),
        token_hash,
        capabilities: req.capabilities.unwrap_or_else(|| vec!["*".into()]),
        enabled: true,
        created_at: Utc::now(),
        last_used_at: None,
        metadata: req.metadata.unwrap_or_default(),
    };

    state
        .db
        .plugin_set("agent", &id, serde_json::to_value(&agent).unwrap())
        .await
        .map_err(|e| AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string()))?;

    Ok(Json(json!({
        "agentId": id,
        "token": raw_token,
        "message": "Store this token securely. It will not be shown again.",
    })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenRequest {
    pub agent_id: String,
}

async fn get_token(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<GetTokenRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;

    let entry = state.db.plugin_get("agent", &req.agent_id).await?.ok_or_else(|| {
        AuthError::new(crate::error::AuthErrorCode::InvalidToken, "Agent not found")
    })?;
    let agent: Agent = serde_json::from_value(entry)
        .map_err(|_| AuthError::new(crate::error::AuthErrorCode::InternalError, "Invalid agent record"))?;

    if agent.user_id != session.user_id {
        return Err(AuthError::forbidden());
    }

    // Generate a new token.
    let raw_token = crate::utils::generate_token();
    let new_hash = sha256_hex(&raw_token);

    let mut updated = agent;
    updated.token_hash = new_hash;
    updated.last_used_at = None;

    state
        .db
        .plugin_set("agent", &req.agent_id, serde_json::to_value(&updated).unwrap())
        .await
        .ok();

    Ok(Json(json!({
        "token": raw_token,
        "agentId": req.agent_id,
    })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityRequest {
    pub capability: String,
    pub agent_token: Option<String>,
}

async fn check_capability(
    State(state): State<AuthState>,
    Json(req): Json<CapabilityRequest>,
) -> Result<Json<Value>, AuthError> {
    // Authenticate via agent token or session.
    let agent = if let Some(at) = &req.agent_token {
        let hash = sha256_hex(at);
        let entries = state.db.plugin_list("agent").await.map_err(|e| {
            AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
        })?;
        let mut found = None;
        for (_, val) in entries {
            if let Ok(a) = serde_json::from_value::<Agent>(val) {
                if a.token_hash == hash && a.enabled {
                    found = Some(a);
                    break;
                }
            }
        }
        found.ok_or_else(|| AuthError::invalid_token())?
    } else {
        return Err(AuthError::missing_field("agentToken"));
    };

    let has_capability = agent.capabilities.iter().any(|c| c == "*" || c == &req.capability);

    // Update last_used_at.
    let mut updated = agent;
    updated.last_used_at = Some(Utc::now());
    state
        .db
        .plugin_set("agent", &updated.id, serde_json::to_value(&updated).unwrap())
        .await
        .ok();

    Ok(Json(json!({
        "hasCapability": has_capability,
        "capability": req.capability,
        "agentId": updated.id,
    })))
}

async fn list_agents(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;

    let entries = state.db.plugin_list("agent").await.map_err(|e| {
        AuthError::new(crate::error::AuthErrorCode::InternalError, e.to_string())
    })?;

    let agents: Vec<Value> = entries
        .into_iter()
        .filter_map(|(_, v)| {
            let a: Agent = serde_json::from_value(v).ok()?;
            if a.user_id == session.user_id {
                Some(json!({
                    "id": a.id,
                    "name": a.name,
                    "capabilities": a.capabilities,
                    "enabled": a.enabled,
                    "createdAt": a.created_at.to_rfc3339(),
                    "lastUsedAt": a.last_used_at.map(|d| d.to_rfc3339()),
                    "metadata": a.metadata,
                }))
            } else {
                None
            }
        })
        .collect();

    Ok(Json(json!({ "agents": agents })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeAgentRequest {
    pub agent_id: String,
}

async fn revoke_agent(
    State(state): State<AuthState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<RevokeAgentRequest>,
) -> Result<Json<Value>, AuthError> {
    let token = extract_token(&headers).ok_or_else(AuthError::invalid_session)?;
    let session = state.session.validate(&token).await?.ok_or_else(AuthError::invalid_session)?;

    let entry = state.db.plugin_get("agent", &req.agent_id).await?.ok_or_else(|| {
        AuthError::new(crate::error::AuthErrorCode::InvalidToken, "Agent not found")
    })?;
    let agent: Agent = serde_json::from_value(entry)
        .map_err(|_| AuthError::new(crate::error::AuthErrorCode::InternalError, "Invalid agent record"))?;

    if agent.user_id != session.user_id {
        return Err(AuthError::forbidden());
    }

    state.db.plugin_delete("agent", &req.agent_id).await.ok();

    Ok(Json(json!({ "success": true, "revoked": req.agent_id })))
}

fn sha256_hex(input: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}