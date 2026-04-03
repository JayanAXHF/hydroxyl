use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}

pub fn stem(path: &Path) -> String {
    path.file_stem()
        .and_then(|value| value.to_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| file_name(path))
}

pub fn read_to_string_if_exists(path: &Path) -> std::io::Result<Option<String>> {
    if path.exists() {
        fs::read_to_string(path).map(Some)
    } else {
        Ok(None)
    }
}

pub fn ancestors_including(path: &Path) -> impl Iterator<Item = PathBuf> + '_ {
    path.ancestors().map(Path::to_path_buf)
}
