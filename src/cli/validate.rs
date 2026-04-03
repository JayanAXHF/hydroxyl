use crate::{
    cli::args::Cli,
    util::{error::HydroxylError, result::Result},
};

pub fn validate(cli: &Cli) -> Result<()> {
    let selected = [
        cli.world.is_some(),
        cli.player.is_some(),
        cli.nbt.is_some(),
        cli.stats.is_some(),
        cli.advancements.is_some(),
    ]
    .into_iter()
    .filter(|value| *value)
    .count();

    if selected > 1 {
        return Err(HydroxylError::invalid_cli(
            "choose only one startup target at a time",
        ));
    }

    Ok(())
}
