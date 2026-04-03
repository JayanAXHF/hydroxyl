use crate::app::tab_id::{DocumentId, TabId};

#[derive(Debug, Clone)]
pub enum TabKind {
    Home(DocumentId),
    Player(DocumentId),
    Nbt(DocumentId),
    Stats(DocumentId),
    Advancements(DocumentId),
}

#[derive(Debug, Clone)]
pub struct TabState {
    pub id: TabId,
    pub title: String,
    pub kind: TabKind,
}
