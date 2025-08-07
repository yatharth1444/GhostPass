use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ghostpass")]
#[command(about = "GhostPass - Zero-Residency Password Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    List,
}
