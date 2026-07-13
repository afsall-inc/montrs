pub mod command;
pub mod config;
pub mod error;
pub mod ext;
pub mod mcp;
pub mod utils;

use clap::{Parser, Subcommand};
use montrs_core::AgentError;

#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum CargoCli {
    Montrs(MontrsCli),
}

#[derive(Parser, Debug)]
#[command(name = "montrs")]
#[command(about = "MontRS Meta-framework CLI", long_about = None)]
pub struct MontrsCli {
    #[command(subcommand)]
    pub command: Commands,

    /// Build artifacts in release mode, with optimizations.
    #[arg(short, long)]
    pub release: bool,

    /// Turn on partial hot-reloading.
    #[arg(long)]
    pub hot_reload: bool,

    /// The features to use when compiling all targets.
    #[arg(long)]
    pub features: Vec<String>,

    /// Verbosity (none: info, errors & warnings, -v: verbose, -vv: very verbose).
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Output logs from dependencies (multiple --log accepted).
    #[arg(long)]
    pub log: Vec<String>,

    /// Use tailwind.toml to generate tailwind.config.js (Pure Rust config).
    #[arg(long)]
    pub tailwind_toml: bool,

    /// Use Tailwind v4 CSS-only configuration (No JS/TOML needed).
    #[arg(long, conflicts_with = "tailwind_toml")]
    pub tailwind_v4: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build the project for production.
    Build,
    /// Serve the project for development with hot-reload.
    Serve,
    /// Watch for changes and rebuild automatically.
    Watch,
    /// Run cargo tests for app, client and server.
    Test {
        /// If specified, filters tests by name.
        filter: Option<String>,

        /// Report format (human, json, junit).
        #[arg(long, default_value = "human")]
        report: String,

        /// Output file for the report (if not human).
        #[arg(long)]
        output: Option<String>,

        /// Run tests in parallel jobs.
        #[arg(short = 'j', long)]
        jobs: Option<usize>,
    },
    /// Run performance benchmarks.
    Bench {
        /// The target file to benchmark (optional).
        #[arg(index = 1)]
        target: Option<String>,

        /// Number of measurement iterations.
        #[arg(long, default_value = "100")]
        iterations: u32,

        /// Number of warm-up iterations.
        #[arg(long, default_value = "10")]
        warmup: u32,

        /// Benchmark timeout in seconds.
        #[arg(long)]
        timeout: Option<u64>,

        /// Filter benchmarks by name.
        #[arg(short, long)]
        filter: Option<String>,

        /// Export report to JSON file.
        #[arg(long)]
        json_output: Option<String>,

        /// Simple/Native mode: Benchmarks a file/binary directly without project overhead.
        #[arg(long)]
        simple: bool,

        /// Generate weight file from benchmark results (Substrate-style).
        #[arg(long)]
        generate_weights: Option<String>,
    },
    /// Format the project's Rust and view! code.
    Fmt {
        /// Check if files are formatted without modifying them.
        #[arg(long)]
        check: bool,
        /// Path to format (default: current directory).
        #[arg(default_value = ".")]
        path: String,
        /// Verbose output.
        #[arg(short, long)]
        verbose: bool,
    },
    /// Start the server and end-2-end tests.
    #[command(name = "e2e")]
    E2e {
        /// Run browsers in headless mode.
        #[arg(long)]
        headless: bool,

        /// Keep the server running after tests complete.
        #[arg(long)]
        keep_alive: bool,

        /// Specify browser to use (chromium, firefox, webkit).
        #[arg(long)]
        browser: Option<String>,
    },
    /// Create a new project from a template.
    New {
        /// Name of the project.
        name: String,
        /// Template to use.
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    /// Run custom tasks defined in montrs.toml.
    Run {
        /// Name of the task to run.
        task: String,
    },
    /// List available tasks.
    Tasks,
    /// Generate shell completions.
    Completions {
        /// Shell to generate completions for.
        shell: clap_complete::Shell,
    },
    /// Upgrade the MontRS CLI to the latest version.
    Upgrade,
    /// Generate an agent-readable specification and snapshot of the project.
    Spec {
        /// Include documentation in the snapshot.
        #[arg(long)]
        include_docs: bool,
        /// Output format (json, yaml, txt).
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Generate a single-file "sketch" of a MontRS component.
    Sketch {
        /// Name of the sketch file.
        name: String,
        /// Component type (plate, route, or app).
        #[arg(short, long, default_value = "plate")]
        kind: String,
    },
    /// Expand a sketch file into a full MontRS workspace structure.
    Expand {
        /// Path to the sketch file.
        path: String,
    },
    /// Generate boilerplate for plates and routes.
    Generate {
        #[command(subcommand)]
        subcommand: GenerateSubcommand,
    },
    /// Agent-facing tools for validation, diagnostics, and atomic changes.
    Agent {
        #[command(subcommand)]
        subcommand: AgentSubcommand,
    },
    /// Model Context Protocol (MCP) server mode.
    Mcp {
        #[command(subcommand)]
        subcommand: McpSubcommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum AgentSubcommand {
    /// Validate structural correctness and project invariants.
    Check {
        /// Path to check.
        #[arg(default_value = ".")]
        path: String,
    },
    /// Assess project health and agent-readability.
    Doctor {
        /// Optional package to focus on.
        #[arg(short, long)]
        package: Option<String>,
    },
    /// Show a diagnostic diff for an error file, including the offending code and the suggested fix.
    Diff {
        /// Path to the error file or diagnostic report.
        path: String,
    },
    /// List all active and resolved errors tracked by the agent.
    ListErrors {
        /// Filter by status (active or resolved).
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Manage agent rules and IDE integration.
    Rules {
        #[command(subcommand)]
        subcommand: RulesSubcommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum RulesSubcommand {
    /// Scaffold IDE rules (.trae/rules and .cursorrules) for the current project.
    Setup,
}

#[derive(Subcommand, Debug)]
pub enum McpSubcommand {
    /// Start the MCP server over stdio.
    Serve,
}

#[derive(Subcommand, Debug)]
pub enum GenerateSubcommand {
    /// Generate a new plate.
    Plate {
        /// Name of the plate.
        name: String,
    },
    /// Generate a new route.
    Route {
        /// Path pattern for the route (e.g., "/users/:id").
        path: String,
        /// The plate to add this route to.
        #[arg(short, long)]
        plate: String,
    },
}

pub async fn run(cli: MontrsCli) -> anyhow::Result<()> {
    // Setup logger based on verbosity
    let log_level = match cli.verbose {
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    // Initialize tracing if not already initialized
    let _ = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .try_init();

    let mut config = config::MontrsConfig::load()?;
    config.project.verbose = cli.verbose;
    config.project.log = cli.log.clone();
    config.project.release = cli.release;
    config.project.hot_reload = cli.hot_reload;
    config.project.features = cli.features.clone();

    if cli.tailwind_toml {
        config.project.tailwind_style = Some(config::TailwindStyle::Toml);
    } else if cli.tailwind_v4 {
        config.project.tailwind_style = Some(config::TailwindStyle::V4);
    }

    match cli.command {
        Commands::Build => command::build::run().await,
        Commands::Serve => command::serve::run().await,
        Commands::Watch => command::watch::run().await,
        Commands::Test {
            filter,
            report,
            output,
            jobs,
        } => command::test::run(filter, report, output, jobs).await,
        Commands::Bench {
            target,
            iterations,
            warmup,
            timeout,
            filter,
            json_output,
            simple,
            generate_weights,
        } => {
            command::bench::run(
                target,
                iterations,
                warmup,
                timeout,
                filter,
                json_output,
                simple,
                generate_weights,
            )
            .await
        }
        Commands::Fmt {
            check,
            path,
            verbose,
        } => command::fmt::run(config.fmt, check, path, verbose).await,
        Commands::E2e {
            headless,
            keep_alive,
            browser,
        } => command::e2e::run(headless, keep_alive, browser).await,
        Commands::New { name, template } => {
            command::new::run(name, template).await
        }
        Commands::Run { task } => command::run::run(task).await,
        Commands::Tasks => command::run::list().await,
        Commands::Completions { shell } => {
            use clap::CommandFactory;
            let mut cmd = MontrsCli::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(
                shell,
                &mut cmd,
                name,
                &mut std::io::stdout(),
            );
            Ok(())
        }
        Commands::Spec {
            include_docs,
            format,
        } => command::spec::run(include_docs, format).await,
        Commands::Sketch { name, kind } => {
            command::sketch::run(name, kind).await
        }
        Commands::Expand { path } => command::expand::run(path).await,
        Commands::Upgrade => command::upgrade::run().await,
        Commands::Generate { subcommand } => match subcommand {
            GenerateSubcommand::Plate { name } => {
                command::generate::plate(name).await
            }
            GenerateSubcommand::Route { path, plate } => {
                command::generate::route(path, plate).await
            }
        },
        Commands::Agent { subcommand } => {
            match command::agent::run(subcommand).await {
                Ok(output) => {
                    println!("{}", output);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error running agent command: {}", e);
                    Err(e)
                }
            }
        }
        Commands::Mcp { subcommand } => command::mcp::run(subcommand).await,
    }
}

/// Main entry point for the CLI, handling both standalone and cargo subcommand modes.
pub fn main_entry() {
    let args: Vec<String> = std::env::args().collect();

    // Agent: Autogenerate .agent folder on every CLI interaction (even before parsing)
    if let Ok(cwd) = std::env::current_dir() {
        let agent_manager = montrs_agent::AgentManager::new(&cwd);
        let app_name = std::fs::read_to_string("montrs.toml")
            .ok()
            .and_then(|c| toml::from_str::<toml::Value>(&c).ok())
            .and_then(|v| {
                v.get("project")
                    .and_then(|p| p.get("name"))
                    .and_then(|n| n.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "app".to_string());

        // Initialize .agent if it doesn't exist
        if !agent_manager.agent_dir().exists()
            && let Err(e) = agent_manager.ensure_dir()
        {
            eprintln!("Warning: Failed to create .agent directory: {}", e);
        }

        // Agent: Update tools and snapshot if we are in an existing project
        if args.len() > 1 && args[1] != "new" {
            // Check if we're in a MontRS project before doing agent work
            if cwd.join("montrs.toml").exists()
                || cwd.join("Cargo.toml").exists()
            {
                if let Err(e) = agent_manager.write_tools_spec() {
                    eprintln!("Warning: Failed to update tools spec: {}", e);
                }

                match agent_manager.generate_snapshot(&app_name) {
                    Ok(snapshot) => {
                        if let Err(e) =
                            agent_manager.write_snapshot(&snapshot, "json")
                        {
                            eprintln!(
                                "Agent: Failed to write JSON snapshot: {}",
                                e
                            );
                        }
                    }
                    Err(_) => {
                        // Silent failure for snapshot generation in non-project dirs
                    }
                }
            }
        }
    }

    // Determine if we are being run as a cargo subcommand or standalone
    let cli = if args.get(1).map(|s| s.as_str()) == Some("montrs") {
        CargoCli::parse().montrs_cli()
    } else {
        MontrsCli::parse()
    };

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    if let Err(e) = rt.block_on(run(cli)) {
        use console::style;

        // Check if the error implements AgentError
        let mut reported = false;
        if let Ok(cwd) = std::env::current_dir() {
            let agent_manager = montrs_agent::AgentManager::new(&cwd);

            // Try to find if it's a CliError or other AgentError
            if let Some(agent_err) = e.downcast_ref::<error::CliError>() {
                eprintln!(
                    "{}[{}]: {}",
                    style("error").red().bold(),
                    style(agent_err.error_code()).yellow().bold(),
                    style(agent_err).bold()
                );
                eprintln!(
                    "  {} {}",
                    style("help:").cyan().bold(),
                    agent_err.explanation()
                );

                let fixes = agent_err.suggested_fixes();
                if !fixes.is_empty() {
                    eprintln!(
                        "  {} suggested fixes:",
                        style("hint:").green().bold()
                    );
                    for fix in fixes {
                        eprintln!("    - {}", fix);
                    }
                }

                let docs = agent_err.documentation_refs();
                if !docs.is_empty() {
                    eprintln!(
                        "  {} related framework rules:",
                        style("rules:").magenta().bold()
                    );
                    for doc in docs {
                        eprintln!("    - {}", style(doc).underlined());
                    }
                }

                let _ = agent_manager.report_agent_error(agent_err);
                reported = true;
            }
        }

        if !reported {
            eprintln!("{} Error: {:?}", style("✘").red().bold(), e);

            // Agent: Report error to .agent/errorfile.json
            if let Ok(cwd) = std::env::current_dir() {
                let agent_manager = montrs_agent::AgentManager::new(&cwd);
                let _ = agent_manager.report_error(format!("{:?}", e));
            }
        }

        std::process::exit(1);
    } else {
        // Agent: On success, check if we resolved any active errors
        if let Ok(cwd) = std::env::current_dir() {
            let agent_manager = montrs_agent::AgentManager::new(&cwd);
            let diff = agent_manager.generate_diff();
            if let Err(err) = agent_manager.auto_resolve_active_errors(
                "Build/Command succeeded".to_string(),
                diff,
            ) {
                eprintln!("Agent: Failed to resolve active errors: {}", err);
            }
        }
    }
}

impl CargoCli {
    pub fn montrs_cli(self) -> MontrsCli {
        match self {
            CargoCli::Montrs(cli) => cli,
        }
    }
}
