use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    persistence::nbt_codec::CompressionKind,
    util::{result::Result, time::unix_timestamp},
};

pub fn create_backup(path: &Path) -> Result<PathBuf> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    let suffix = if extension.is_empty() {
        format!("hydroxyl-{}.bak", unix_timestamp())
    } else {
        format!("{extension}.hydroxyl-{}.bak", unix_timestamp())
    };
    let backup_path = path.with_extension(suffix);
    fs::copy(path, &backup_path)?;
    Ok(backup_path)
}

pub fn preserve_compression(path: &Path) -> CompressionKind {
    let name = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if name.eq_ignore_ascii_case("dat") {
        CompressionKind::Gzip
    } else {
        CompressionKind::Raw
    }
}
