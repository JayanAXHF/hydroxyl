use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenTarget {
    Home,
    World(PathBuf),
    Player(PathBuf),
    Nbt(PathBuf),
    Stats(PathBuf),
    Advancements(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchConfig {
    pub target: OpenTarget,
}

impl Default for LaunchConfig {
    fn default() -> Self {
        Self {
            target: OpenTarget::Home,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppContext {
    pub launch_config: LaunchConfig,
}

impl AppContext {
    pub fn new(launch_config: LaunchConfig) -> Self {
        Self { launch_config }
    }
}
