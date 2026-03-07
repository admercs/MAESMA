//! MAESMA CLI — command-line interface for agentic AI for autonomous Earth system observation, model discovery, and simulation.

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(
    name = "maesma",
    about = "Agentic AI for Autonomous Earth System Observation, Model Discovery, and Simulation",
    version,
    long_about = "MAESMA: An agentic framework for autonomously assembling, benchmarking, \
                  and evolving Earth system model configurations from a Process Knowledgebase."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Knowledgebase database path.
    #[arg(long, default_value = "maesma.db", global = true)]
    db: String,

    /// Verbosity level.
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new knowledgebase.
    Init,

    /// Knowledgebase operations.
    Kb {
        #[command(subcommand)]
        action: KbCommands,
    },

    /// Validate a SAPG configuration.
    Validate {
        /// Path to SAPG configuration file (JSON).
        #[arg(short, long)]
        config: String,
    },

    /// Check closure properties of a SAPG.
    CheckClosure {
        /// Path to SAPG configuration file.
        #[arg(short, long)]
        config: String,
    },

    /// Run a simulation.
    Run {
        /// Path to SAPG configuration file.
        #[arg(short, long)]
        config: String,

        /// Number of timesteps.
        #[arg(short, long, default_value = "365")]
        steps: u64,
    },

    /// Start the API server.
    Serve {
        /// Port to listen on.
        #[arg(short, long, default_value = "3001")]
        port: u16,
    },

    /// Show system info.
    Info,
}

#[derive(Subcommand)]
enum KbCommands {
    /// List all process manifests.
    List {
        /// Filter by family code.
        #[arg(short, long)]
        family: Option<String>,
    },

    /// Show details of a specific process.
    Show {
        /// Process ID.
        id: String,
    },

    /// Import manifests from a JSON file.
    Import {
        /// Path to JSON file.
        path: String,
    },

    /// Export the knowledgebase to JSON.
    Export {
        /// Output path.
        #[arg(short, long, default_value = "kb_export.json")]
        output: String,
    },

    /// Show knowledgebase statistics.
    Stats,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let filter = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .init();

    match cli.command {
        Commands::Init => {
            let _kb = maesma_knowledgebase::KnowledgebaseStore::open(&cli.db)?;
            println!("✓ Knowledgebase initialized at {}", cli.db);
        }

        Commands::Kb { action } => match action {
            KbCommands::List { family } => {
                let kb = maesma_knowledgebase::KnowledgebaseStore::open(&cli.db)?;
                let manifests = kb.list_manifests()?;
                for (id, name, fam) in &manifests {
                    if let Some(ref f) = family {
                        if !fam.contains(f) {
                            continue;
                        }
                    }
                    println!("{id}  {fam:<20}  {name}");
                }
                println!("\n{} manifests total", manifests.len());
            }
            KbCommands::Show { id } => {
                println!("TODO: show manifest {id}");
            }
            KbCommands::Import { path } => {
                println!("TODO: import from {path}");
            }
            KbCommands::Export { output } => {
                println!("TODO: export to {output}");
            }
            KbCommands::Stats => {
                let kb = maesma_knowledgebase::KnowledgebaseStore::open(&cli.db)?;
                println!("Manifests:     {}", kb.manifest_count()?);
                println!("Skill records: {}", kb.skill_count()?);
            }
        },

        Commands::Validate { config } => {
            println!("TODO: validate SAPG from {config}");
        }

        Commands::CheckClosure { config } => {
            println!("TODO: check closure from {config}");
        }

        Commands::Run { config, steps } => {
            println!("TODO: run simulation from {config} for {steps} steps");
        }

        Commands::Serve { port } => {
            println!("TODO: start API server on port {port}");
        }

        Commands::Info => {
            println!(
                "MAESMA — Agentic AI for Autonomous Earth System Observation, Model Discovery, and Simulation"
            );
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!(
                "Process families: {}",
                maesma_core::ProcessFamily::all().len()
            );
            println!("Fidelity rungs:   R0 → R3");
        }
    }

    Ok(())
}
