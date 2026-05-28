use crate::{
    app::context::{LaunchConfig, OpenTarget, WorldFileStructure},
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

    let world_file_structure = if cli.new_file_structure {
        WorldFileStructure::New
    } else {
        WorldFileStructure::Legacy
    };

    Ok(LaunchConfig {
        target,
        world_file_structure,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use clap::Parser;

    use crate::{
        app::context::{OpenTarget, WorldFileStructure},
        cli::{
            args::{Cli, CliCommand},
            targets::resolve,
        },
    };

    #[test]
    fn resolves_home_when_no_subcommand_is_provided() {
        let cli = Cli {
            new_file_structure: false,
            command: None,
        };
        let launch = resolve(&cli).unwrap();
        assert_eq!(launch.target, OpenTarget::Home);
        assert_eq!(launch.world_file_structure, WorldFileStructure::Legacy);
    }

    #[test]
    fn resolves_world_subcommand() {
        let cli = Cli {
            new_file_structure: false,
            command: Some(CliCommand::World {
                path: PathBuf::from("world"),
            }),
        };
        let launch = resolve(&cli).unwrap();
        assert_eq!(launch.target, OpenTarget::World(PathBuf::from("world")));
    }

    #[test]
    fn resolves_new_file_structure_flag() {
        let cli = Cli {
            new_file_structure: true,
            command: Some(CliCommand::World {
                path: PathBuf::from("world"),
            }),
        };

        let launch = resolve(&cli).unwrap();

        assert_eq!(launch.world_file_structure, WorldFileStructure::New);
    }

    #[test]
    fn parses_new_file_structure_as_global_flag() {
        let before = Cli::try_parse_from(["hc", "--new-file-structure", "world", "world"]).unwrap();
        let after = Cli::try_parse_from(["hc", "world", "--new-file-structure", "world"]).unwrap();

        assert!(before.new_file_structure);
        assert!(after.new_file_structure);
    }
}
