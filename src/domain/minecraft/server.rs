use std::path::PathBuf;

use uuid::Uuid;

use crate::domain::files::kind::FileKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspacePane {
    Players,
    Stats,
    Advancements,
}

impl WorkspacePane {
    pub const ALL: [Self; 3] = [Self::Players, Self::Stats, Self::Advancements];

    pub fn title(self) -> &'static str {
        match self {
            Self::Players => "Player Data",
            Self::Stats => "Stats",
            Self::Advancements => "Advancements",
        }
    }
}

impl Default for WorkspacePane {
    fn default() -> Self {
        Self::Players
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceSelection {
    pub active_pane: WorkspacePane,
    pub player_index: usize,
    pub stats_index: usize,
    pub advancements_index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceEntry {
    pub label: String,
    pub path: PathBuf,
    pub kind: FileKind,
    pub uuid: Option<Uuid>,
    pub resolved_name: Option<String>,
}

impl WorkspaceEntry {
    pub fn display_label(&self) -> String {
        match (&self.resolved_name, self.kind) {
            (Some(name), FileKind::PlayerData) => format!("{name} ({})", self.label),
            _ => self.label.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ServerContext {
    pub root: PathBuf,
    pub world_path: PathBuf,
    pub level_name: String,
    pub online_mode: bool,
    pub player_files: Vec<PathBuf>,
    pub stats_files: Vec<PathBuf>,
    pub advancements_files: Vec<PathBuf>,
    pub player_entries: Vec<WorkspaceEntry>,
    pub stats_entries: Vec<WorkspaceEntry>,
    pub advancements_entries: Vec<WorkspaceEntry>,
}
