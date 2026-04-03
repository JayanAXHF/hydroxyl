use crate::{
    app::{
        document::{JsonPath, JsonPathSegment},
        tab_id::DocumentId,
    },
    domain::{minecraft::player::PlayerSection, nbt::path::NbtPath},
};

#[derive(Debug, Clone)]
pub enum EditTarget {
    PlayerField {
        document_id: DocumentId,
        field: PlayerField,
    },
    NbtValue {
        document_id: DocumentId,
        path: NbtPath,
    },
    JsonValue {
        document_id: DocumentId,
        path: JsonPath,
    },
}

#[derive(Debug, Clone)]
pub enum PlayerField {
    Health,
    FoodLevel,
    FoodSaturation,
    XpLevel,
    XpProgress,
    XpTotal,
    Air,
    PosX,
    PosY,
    PosZ,
    Yaw,
    Pitch,
    Dimension,
    Flying,
    MayFly,
    Instabuild,
    Invulnerable,
    MayBuild,
    WalkSpeed,
    FlySpeed,
    InventoryCount,
    InventoryId,
    RawSection(PlayerSection),
}

#[derive(Debug, Clone)]
pub enum Action {
    NextTab,
    PreviousTab,
    FocusNext,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    SaveActive,
    RequestQuit,
    StartEdit,
    ConfirmEdit,
    CancelEdit,
    InputChar(char),
    Backspace,
    OpenSelected,
    Noop,
}

impl JsonPathSegment {
    pub fn label(&self) -> String {
        match self {
            Self::Key(value) => value.clone(),
            Self::Index(value) => value.to_string(),
        }
    }
}
