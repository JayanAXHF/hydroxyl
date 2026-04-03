use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentSource {
    Direct,
    Workspace { root: PathBuf },
}
