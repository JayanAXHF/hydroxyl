use std::path::{Path, PathBuf};

use crate::{
    app::context::WorldFileStructure, domain::files::kind::FileKind, util::fs::ancestors_including,
};

pub fn detect_file_kind(path: &Path, structure: WorldFileStructure) -> FileKind {
    if is_server_root(path) || is_world_root(path, structure) {
        return FileKind::Workspace;
    }

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    let parent = path
        .parent()
        .and_then(|value| value.file_name())
        .and_then(|value| value.to_str());

    match (extension, parent) {
        ("dat", Some("playerdata")) if structure == WorldFileStructure::Legacy => {
            FileKind::PlayerData
        }
        ("dat", Some("data"))
            if structure == WorldFileStructure::New && is_new_player_data_path(path) =>
        {
            FileKind::PlayerData
        }
        ("json", Some("stats")) => FileKind::Stats,
        ("json", Some("advancements")) => FileKind::Advancements,
        ("dat", _) | ("nbt", _) => FileKind::Nbt,
        ("json", _) => FileKind::Stats,
        _ => FileKind::Unknown,
    }
}

pub fn is_server_root(path: &Path) -> bool {
    path.join("server.properties").exists()
}

pub fn is_world_root(path: &Path, structure: WorldFileStructure) -> bool {
    match structure {
        WorldFileStructure::Legacy => {
            path.join("playerdata").exists()
                || path.join("stats").exists()
                || path.join("advancements").exists()
        }
        WorldFileStructure::New => {
            path.join("players").join("data").exists()
                || path.join("players").join("stats").exists()
                || path.join("players").join("advancements").exists()
                || path
                    .join("dimensions")
                    .join("minecraft")
                    .join("overworld")
                    .exists()
        }
    }
}

pub fn infer_server_root(path: &Path) -> Option<PathBuf> {
    ancestors_including(path).find(|ancestor| ancestor.join("server.properties").exists())
}

pub fn infer_world_root(path: &Path, structure: WorldFileStructure) -> Option<PathBuf> {
    ancestors_including(path).find(|ancestor| is_world_root(ancestor, structure))
}

fn is_new_player_data_path(path: &Path) -> bool {
    path.parent()
        .and_then(|parent| parent.parent())
        .and_then(|parent| parent.file_name())
        .and_then(|value| value.to_str())
        == Some("players")
}
