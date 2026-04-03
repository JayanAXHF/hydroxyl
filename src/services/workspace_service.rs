use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    domain::{
        files::{
            detect::{is_server_root, is_world_root},
            kind::FileKind,
            naming::parse_uuid_from_path,
        },
        minecraft::server::{ServerContext, WorkspaceEntry},
    },
    util::{error::HydroxylError, fs::stem, result::Result},
};

pub struct WorkspaceService;

impl WorkspaceService {
    pub fn load(&self, path: &Path) -> Result<ServerContext> {
        let (root, world_path, properties) = if is_server_root(path) {
            let properties = parse_properties(&path.join("server.properties"))?;
            let level_name = properties
                .get("level-name")
                .cloned()
                .unwrap_or_else(|| "world".to_owned());
            let world_path = path.join(&level_name);
            (path.to_path_buf(), world_path, properties)
        } else if is_world_root(path) {
            (path.to_path_buf(), path.to_path_buf(), Default::default())
        } else {
            return Err(HydroxylError::invalid_data(format!(
                "{} is not a server root or world directory",
                path.display()
            )));
        };

        let level_name = world_path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("world")
            .to_owned();
        let online_mode = properties
            .get("online-mode")
            .map(|value| value.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let player_files = list_files(&world_path.join("playerdata"), "dat")?;
        let stats_files = list_files(&world_path.join("stats"), "json")?;
        let advancements_files = list_files(&world_path.join("advancements"), "json")?;

        let player_entries = player_files
            .iter()
            .map(|path| WorkspaceEntry {
                label: stem(path),
                path: path.clone(),
                kind: FileKind::PlayerData,
                uuid: parse_uuid_from_path(path),
                resolved_name: None,
            })
            .collect();
        let stats_entries = stats_files
            .iter()
            .map(|path| WorkspaceEntry {
                label: stem(path),
                path: path.clone(),
                kind: FileKind::Stats,
                uuid: None,
                resolved_name: None,
            })
            .collect();
        let advancements_entries = advancements_files
            .iter()
            .map(|path| WorkspaceEntry {
                label: stem(path),
                path: path.clone(),
                kind: FileKind::Advancements,
                uuid: None,
                resolved_name: None,
            })
            .collect();

        Ok(ServerContext {
            root,
            world_path,
            level_name,
            online_mode,
            player_files,
            stats_files,
            advancements_files,
            player_entries,
            stats_entries,
            advancements_entries,
        })
    }
}

fn parse_properties(path: &Path) -> Result<std::collections::HashMap<String, String>> {
    let mut map = std::collections::HashMap::new();
    if !path.exists() {
        return Ok(map);
    }

    let contents = fs::read_to_string(path)?;
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            map.insert(key.trim().to_owned(), value.trim().to_owned());
        }
    }
    Ok(map)
}

fn list_files(path: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut files = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| entry.is_file())
        .filter(|entry| {
            entry
                .extension()
                .and_then(|value| value.to_str())
                .map(|value| value.eq_ignore_ascii_case(extension))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}
