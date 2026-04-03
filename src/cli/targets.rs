use crate::{
    app::context::{LaunchConfig, OpenTarget},
    cli::args::{Cli, CliCommand},
    util::result::Result,
};

pub fn resolve(cli: &Cli) -> Result<LaunchConfig> {
    let target = match &cli.command {
        Some(CliCommand::World { path }) => OpenTarget::World(path.clone()),
        Some(CliCommand::Player { path }) => OpenTarget::Player(path.clone()),
        Some(CliCommand::Nbt { path }) => OpenTarget::Nbt(path.clone()),
        Some(CliCommand::Stats { path }) => OpenTarget::Stats(path.clone()),
        Some(CliCommand::Advancements { path }) => OpenTarget::Advancements(path.clone()),
        None => OpenTarget::Home,
    };

    Ok(LaunchConfig { target })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        app::context::OpenTarget,
        cli::{
            args::{Cli, CliCommand},
            targets::resolve,
        },
    };

    #[test]
    fn resolves_home_when_no_subcommand_is_provided() {
        let cli = Cli { command: None };
        let launch = resolve(&cli).unwrap();
        assert_eq!(launch.target, OpenTarget::Home);
    }

    #[test]
    fn resolves_world_subcommand() {
        let cli = Cli {
            command: Some(CliCommand::World {
                path: PathBuf::from("world"),
            }),
        };
        let launch = resolve(&cli).unwrap();
        assert_eq!(launch.target, OpenTarget::World(PathBuf::from("world")));
    }
}
