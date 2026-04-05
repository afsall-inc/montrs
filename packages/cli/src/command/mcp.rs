use crate::McpSubcommand;
use crate::mcp;

pub async fn run(subcommand: McpSubcommand) -> anyhow::Result<()> {
    match subcommand {
        McpSubcommand::Serve => mcp::run_server().await,
    }
}
