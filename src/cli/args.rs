use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser, Clone)]
#[command(name = "hc", author, version, about = "Minecraft server files manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum CliCommand {
    #[command(about = "Open a server root or world directory")]
    World {
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
    #[command(about = "Open a single player .dat file")]
    Player {
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
    #[command(about = "Open a standalone NBT file")]
    Nbt {
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
    #[command(about = "Open a standalone stats JSON file")]
    Stats {
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
    #[command(about = "Open a standalone advancements JSON file")]
    Advancements {
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
}
