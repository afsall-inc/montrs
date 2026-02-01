pub mod protocol;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use protocol::*;
use serde_json::{json, Value};
use crate::command::agent;
use crate::AgentSubcommand;

pub async fn run_server() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut stdout = io::stdout();
    let mut line = String::new();

    while reader.read_line(&mut line).await? > 0 {
        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                let response = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: Value::Null,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                        data: None,
                    }),
                };
                let resp_str = serde_json::to_string(&response)?;
                stdout.write_all(resp_str.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
                line.clear();
                continue;
            }
        };

        let response = handle_request(request).await?;
        let resp_str = serde_json::to_string(&response)?;
        stdout.write_all(resp_str.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
        line.clear();
    }

    Ok(())
}

async fn handle_request(req: JsonRpcRequest) -> anyhow::Result<JsonRpcResponse> {
    let result = match req.method.as_str() {
        "initialize" => {
            let result = InitializeResult {
                protocol_version: "2024-11-05".to_string(),
                capabilities: ServerCapabilities {
                    tools: Some(ToolCapabilities {
                        list_changed: Some(false),
                    }),
                },
                server_info: ServerInfo {
                    name: "montrs-mcp".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                },
            };
            Some(serde_json::to_value(result)?)
        }
        "notifications/initialized" => {
            None
        }
        "tools/list" => {
            let tools = vec![
                Tool {
                    name: "agent_check".to_string(),
                    description: "Validate structural correctness and project invariants.".to_string(),
                    input_validator: json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string", "description": "Path to check" }
                        }
                    }),
                },
                Tool {
                    name: "agent_doctor".to_string(),
                    description: "Assess project health and agent-readability.".to_string(),
                    input_validator: json!({
                        "type": "object",
                        "properties": {
                            "package": { "type": "string", "description": "Optional package to focus on" }
                        }
                    }),
                },
                Tool {
                    name: "agent_diff".to_string(),
                    description: "Analyze error file and generate fix suggestions.".to_string(),
                    input_validator: json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string", "description": "Path to error file" }
                        },
                        "required": ["path"]
                    }),
                },
                Tool {
                    name: "list_router_structure".to_string(),
                    description: "Retrieve the full routing tree of the application.".to_string(),
                    input_validator: json!({
                        "type": "object",
                        "properties": {}
                    }),
                },
                Tool {
                    name: "get_project_snapshot".to_string(),
                    description: "Get the full agent.json specification for the project.".to_string(),
                    input_validator: json!({
                        "type": "object",
                        "properties": {}
                    }),
                },
                Tool {
                    name: "agent_list_errors".to_string(),
                    description: "List all active and resolved errors tracked by the agent.".to_string(),
                    input_validator: json!({
                        "type": "object",
                        "properties": {
                            "status": { "type": "string", "enum": ["Pending", "Fixed"], "description": "Filter by status" }
                        }
                    }),
                },
                Tool {
                    name: "get_agent_entry_point".to_string(),
                    description: "Get the unified entry point for agent operations, mapping tasks to guides.".to_string(),
                    input_validator: json!({ "type": "object", "properties": {} }),
                },
            ];
            Some(serde_json::to_value(ListToolsResult { tools })?)
        }
        "tools/call" => {
            let params: CallToolParams = serde_json::from_value(req.params)?;
            let tool_result = handle_tool_call(params).await?;
            Some(serde_json::to_value(tool_result)?)
        }
        _ => {
            return Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: format!("Method not found: {}", req.method),
                    data: None,
                }),
            });
        }
    };

    Ok(JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: req.id,
        result,
        error: None,
    })
}

async fn handle_tool_call(params: CallToolParams) -> anyhow::Result<CallToolResult> {
    match params.name.as_str() {
        "agent_check" => {
            let path = params.arguments.get("path").and_then(|v| v.as_str()).unwrap_or(".");
            let output = agent::run(AgentSubcommand::Check { path: path.to_string() }).await?;
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: output }],
                is_error: false,
            })
        }
        "agent_doctor" => {
            let package = params.arguments.get("package").and_then(|v| v.as_str()).map(|s| s.to_string());
            let output = agent::run(AgentSubcommand::Doctor { package }).await?;
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: output }],
                is_error: false,
            })
        }
        "agent_diff" => {
            let path = params.arguments.get("path").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("Missing path argument"))?;
            let output = agent::run(AgentSubcommand::Diff { path: path.to_string() }).await?;
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: output }],
                is_error: false,
            })
        }
        "get_project_snapshot" => {
            let include_docs = params.arguments.get("include_docs").and_then(|v| v.as_bool()).unwrap_or(false);
            // We'll call the spec command logic
            let output = crate::command::spec::run_to_string(include_docs, "json".to_string()).await?;
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: output }],
                is_error: false,
            })
        }
        "list_router_structure" => {
            // This is a simplified version, ideally we'd have a specific router introspection command
            let output = crate::command::spec::run_to_string(false, "json".to_string()).await?;
            // Extract router info from snapshot
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: output }],
                is_error: false,
            })
        }
        "get_agent_entry_point" => {
            let cwd = std::env::current_dir()?;
            let manager = montrs_agent::AgentManager::new(cwd);
            // Try to get from project first, then fallback to embedded
            let snapshot = manager.generate_snapshot("temp").unwrap_or_else(|_| manager.generate_framework_snapshot());
            let entry_point = snapshot.agent_entry_point.unwrap_or_else(|| "No entry point found.".to_string());
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: entry_point }],
                is_error: false,
            })
        }
        "agent_list_errors" => {
            let status = params.arguments.get("status").and_then(|v| v.as_str()).map(|s| s.to_string());
            let output = agent::run(AgentSubcommand::ListErrors { status }).await?;
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: output }],
                is_error: false,
            })
        }
        _ => Ok(CallToolResult {
            content: vec![ToolContent::Text { text: format!("Unknown tool: {}", params.name) }],
            is_error: true,
        }),
    }
}
