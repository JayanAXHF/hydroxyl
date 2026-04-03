use crate::{
    app::context::{LaunchConfig, OpenTarget},
    cli::{args::Cli, validate::validate},
    util::result::Result,
};

pub fn resolve(cli: &Cli) -> Result<LaunchConfig> {
    validate(cli)?;

    let target = if let Some(path) = &cli.world {
        OpenTarget::World(path.clone())
    } else if let Some(path) = &cli.player {
        OpenTarget::Player(path.clone())
    } else if let Some(path) = &cli.nbt {
        OpenTarget::Nbt(path.clone())
    } else if let Some(path) = &cli.stats {
        OpenTarget::Stats(path.clone())
    } else if let Some(path) = &cli.advancements {
        OpenTarget::Advancements(path.clone())
    } else {
        OpenTarget::Home
    };

    Ok(LaunchConfig { target })
}
