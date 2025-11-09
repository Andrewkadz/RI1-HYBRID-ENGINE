use clap::{Parser, Subcommand};
use ri1_core::modality::GenerationRequest;
use ri1_core::Orchestrator;
use ri1_symbolic_meta::MetaEngineImpl;
use ri1_text::BasicText;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "ri1", version, about = "RI1 Hybrid Generative Engine CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate content
    Gen {
        #[command(subcommand)]
        modality: GenModality,
    },
}

#[derive(Subcommand, Debug)]
enum GenModality {
    /// Text generation
    Text {
        /// Prompt string
        #[arg(short, long)]
        prompt: String,
    },
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Gen { modality } => match modality {
            GenModality::Text { prompt } => gen_text(prompt),
        },
    }
}

fn gen_text(prompt: String) {
    let mut orch = Orchestrator::new();
    orch.register_modality(BasicText);
    orch.set_meta_engine(MetaEngineImpl::new_default());

    info!("modalities = {:?}", orch.modalities());
    let req = GenerationRequest { prompt };
    if let Some(res) = orch.generate("text", req) {
        println!("{}", res.content);
        let log = orch.evaluate("text", &res.content);
        if !log.is_empty() {
            println!("--- constraints ---");
            for r in log {
                println!("{} [{}]: {}", r.name, r.severity, r.message.unwrap_or_else(|| "ok".into()));
            }
        }
    } else {
        warn!("generation failed or blocked by constraints");
        eprintln!("error: generation failed or blocked by constraints");
    }
}
