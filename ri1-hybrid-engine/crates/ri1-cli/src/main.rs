use clap::{Parser, Subcommand};
use ri1_core::modality::GenerationRequest;
use ri1_core::Orchestrator;
use ri1_core::constraints::{ResonanceEvent, OperatorClass, ConstraintResult};
use ri1_symbolic_meta::{MetaEngineImpl, InfluenceSnapshot, compute_influence};
use ri1_text::BasicText;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use serde_json;
use uuid::Uuid;
use std::path::PathBuf;
use std::fs;

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
        /// Print resonance events
        #[arg(long, default_value_t = false)]
        verbose: bool,
        /// Output resonance events as JSON
        #[arg(long, default_value_t = false)]
        json: bool,
        /// Show influence block in verbose output
        #[arg(long, default_value_t = false)]
        influence: bool,
        /// Optional correlation id (uuid); if not provided, generated per run
        #[arg(long)]
        cid: Option<String>,
        /// Write a JSON envelope (with cid, content, constraints, events) to file
        #[arg(long)]
        log_file: Option<PathBuf>,
    },
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Gen { modality } => match modality {
            GenModality::Text { prompt, verbose, json, influence, cid, log_file } => gen_text(prompt, verbose, json, influence, cid, log_file),
        },
    }
}

#[derive(serde::Serialize)]
struct LogEnvelope {
    correlation_id: String,
    modality: String,
    content: String,
    constraints: Vec<ConstraintResult>,
    events: Vec<ResonanceEvent>,
    timestamp_unix_s: u64,
    influence: InfluenceSnapshot,
}

fn gen_text(prompt: String, verbose: bool, json: bool, influence_flag: bool, cid: Option<String>, log_file: Option<PathBuf>) {
    let mut orch = Orchestrator::new();
    orch.register_modality(BasicText);
    orch.set_meta_engine(MetaEngineImpl::new_default());

    info!("modalities = {:?}", orch.modalities());
    let req = GenerationRequest { prompt };
    if let Some(res) = orch.generate("text", req) {
        println!("{}", res.content);
        let (log, mut events) = orch.evaluate_with_events("text", &res.content);
        // Correlation ID event injection
        let cid_val = cid.unwrap_or_else(|| Uuid::new_v4().to_string());
        let corr_event = ResonanceEvent {
            operator: OperatorClass::InteractionNotice,
            message: format!("correlation_id: {}", cid_val),
            section_ref: None,
            symbol: None,
        };
        events.insert(0, corr_event);

        // Influence snapshot (logging-only)
        let (influence, _infl_notice) = compute_influence(&events);

        // Optional file logging of JSON envelope
        if let Some(path) = log_file {
            let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
            let env = LogEnvelope {
                correlation_id: cid_val.clone(),
                modality: "text".into(),
                content: res.content.clone(),
                constraints: log.clone(),
                events: events.clone(),
                timestamp_unix_s: ts,
                influence: influence.clone(),
            };
            if let Ok(s) = serde_json::to_string_pretty(&env) {
                let _ = fs::write(path, s);
            }
        }
        if json {
            if !events.is_empty() {
                let s = serde_json::to_string_pretty(&events).unwrap_or_else(|_| "[]".into());
                println!("--- resonance(json) ---\n{}", s);
            }
        } else if verbose {
            if !events.is_empty() {
                println!("--- resonance ---");
                for e in events {
                    let sym = e.symbol.as_deref().unwrap_or("?");
                    let sec = e.section_ref.as_deref().unwrap_or("-");
                    println!("{} [{}] {}: {}", sym, sec, format!("{:?}", e.operator), e.message);
                }
            }
            if influence_flag {
                println!("--- influence ---");
                println!("resonance_index = {:.2}", influence.resonance_index);
                if !influence.operator_influence.is_empty() {
                    println!("top = {}", influence.operator_influence.iter().take(3).map(|ow| format!("{:?}:{:.2}", ow.operator, ow.weight)).collect::<Vec<_>>().join(", "));
                }
                println!("cooperation = {}  conflict = {}", influence.cooperation_count, influence.conflict_count);
            }
        }
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
