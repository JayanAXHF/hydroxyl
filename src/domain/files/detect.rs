use std::path::{Path, PathBuf};

use crate::{domain::files::kind::FileKind, util::fs::ancestors_including};

pub fn detect_file_kind(path: &Path) -> FileKind {
    if is_server_root(path) || is_world_root(path) {
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
        ("dat", Some("playerdata")) => FileKind::PlayerData,
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

pub fn is_world_root(path: &Path) -> bool {
    path.join("playerdata").exists()
        || path.join("stats").exists()
        || path.join("advancements").exists()
}

pub fn infer_server_root(path: &Path) -> Option<PathBuf> {
    ancestors_including(path).find(|ancestor| ancestor.join("server.properties").exists())
}

pub fn infer_world_root(path: &Path) -> Option<PathBuf> {
    ancestors_including(path).find(|ancestor| is_world_root(ancestor))
}
