use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[command(
    name = "hydroxyl",
    author,
    version,
    about = "Minecraft server files manager"
)]
pub struct Cli {
    #[arg(
        long,
        value_name = "PATH",
        help = "Open a server root or world directory"
    )]
    pub world: Option<PathBuf>,
    #[arg(long, value_name = "PATH", help = "Open a single player .dat file")]
    pub player: Option<PathBuf>,
    #[arg(long, value_name = "PATH", help = "Open a standalone NBT file")]
    pub nbt: Option<PathBuf>,
    #[arg(long, value_name = "PATH", help = "Open a standalone stats JSON file")]
    pub stats: Option<PathBuf>,
    #[arg(
        long,
        value_name = "PATH",
        help = "Open a standalone advancements JSON file"
    )]
    pub advancements: Option<PathBuf>,
}
