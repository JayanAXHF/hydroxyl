use std::path::{Path, PathBuf};

use crate::{persistence::backup, util::result::Result};

pub struct BackupService;

impl BackupService {
    pub fn create_backup(&self, path: &Path) -> Result<PathBuf> {
        backup::create_backup(path)
    }
}
