#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Workspace,
    PlayerData,
    Nbt,
    Stats,
    Advancements,
    Unknown,
}

impl Default for FileKind {
    fn default() -> Self {
        Self::Unknown
    }
}
