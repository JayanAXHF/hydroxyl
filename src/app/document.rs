use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

use serde_json::Value as JsonValue;

use crate::{
    app::tab_id::DocumentId,
    domain::{
        files::source::DocumentSource,
        minecraft::{
            advancements::AdvancementsFile,
            player::PlayerData,
            profile::SkinState,
            server::{ServerContext, WorkspaceEntry, WorkspacePane, WorkspaceSelection},
            stats::StatsFile,
        },
        nbt::edit::NbtEntry,
        nbt::value::NbtValue,
    },
    persistence::{dirty::DirtyState, nbt_codec::CompressionKind},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentKind {
    Workspace,
    Player,
    Nbt,
    Stats,
    Advancements,
}

#[derive(Debug, Clone)]
pub struct DocumentMeta {
    pub id: DocumentId,
    pub kind: DocumentKind,
    pub path: PathBuf,
    pub title: String,
    pub source: DocumentSource,
    pub dirty: DirtyState,
}

impl DocumentMeta {
    pub fn is_dirty(&self) -> bool {
        self.dirty.is_dirty()
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceDocument {
    pub meta: DocumentMeta,
    pub server: ServerContext,
    pub selection: WorkspaceSelection,
}

impl WorkspaceDocument {
    pub fn active_entries(&self) -> &[WorkspaceEntry] {
        match self.selection.active_pane {
            WorkspacePane::Players => &self.server.player_entries,
            WorkspacePane::Stats => &self.server.stats_entries,
            WorkspacePane::Advancements => &self.server.advancements_entries,
        }
    }

    pub fn selected_index(&self) -> usize {
        match self.selection.active_pane {
            WorkspacePane::Players => self.selection.player_index,
            WorkspacePane::Stats => self.selection.stats_index,
            WorkspacePane::Advancements => self.selection.advancements_index,
        }
    }

    pub fn selected_index_mut(&mut self) -> &mut usize {
        match self.selection.active_pane {
            WorkspacePane::Players => &mut self.selection.player_index,
            WorkspacePane::Stats => &mut self.selection.stats_index,
            WorkspacePane::Advancements => &mut self.selection.advancements_index,
        }
    }

    pub fn selected_entry(&self) -> Option<&WorkspaceEntry> {
        self.active_entries().get(self.selected_index())
    }
}

#[derive(Debug, Clone)]
pub struct PlayerDocument {
    pub meta: DocumentMeta,
    pub server: Option<ServerContext>,
    pub compression: CompressionKind,
    pub root: NbtValue,
    pub data: PlayerData,
    pub skin_state: SkinState,
}

#[derive(Debug, Clone)]
pub struct NbtDocument {
    pub meta: DocumentMeta,
    pub compression: CompressionKind,
    pub root: NbtValue,
    pub entries: Vec<NbtEntry>,
    pub selected: usize,
}

#[derive(Debug, Clone)]
pub struct StatsDocument {
    pub meta: DocumentMeta,
    pub typed: StatsFile,
    pub root: JsonValue,
    pub entries: Vec<JsonEntry>,
    pub selected: usize,
}

#[derive(Debug, Clone)]
pub struct AdvancementsDocument {
    pub meta: DocumentMeta,
    pub typed: AdvancementsFile,
    pub root: JsonValue,
    pub entries: Vec<JsonEntry>,
    pub selected: usize,
}

#[derive(Debug, Clone)]
pub enum Document {
    Workspace(WorkspaceDocument),
    Player(PlayerDocument),
    Nbt(NbtDocument),
    Stats(StatsDocument),
    Advancements(AdvancementsDocument),
}

impl Document {
    pub fn meta(&self) -> &DocumentMeta {
        match self {
            Self::Workspace(value) => &value.meta,
            Self::Player(value) => &value.meta,
            Self::Nbt(value) => &value.meta,
            Self::Stats(value) => &value.meta,
            Self::Advancements(value) => &value.meta,
        }
    }

    pub fn meta_mut(&mut self) -> &mut DocumentMeta {
        match self {
            Self::Workspace(value) => &mut value.meta,
            Self::Player(value) => &mut value.meta,
            Self::Nbt(value) => &mut value.meta,
            Self::Stats(value) => &mut value.meta,
            Self::Advancements(value) => &mut value.meta,
        }
    }

    pub fn title(&self) -> &str {
        &self.meta().title
    }

    pub fn is_dirty(&self) -> bool {
        self.meta().is_dirty()
    }

    pub fn id(&self) -> DocumentId {
        self.meta().id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsonPathSegment {
    Key(String),
    Index(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct JsonPath(pub Vec<JsonPathSegment>);

impl JsonPath {
    pub fn child_key(&self, key: impl Into<String>) -> Self {
        let mut next = self.0.clone();
        next.push(JsonPathSegment::Key(key.into()));
        Self(next)
    }

    pub fn child_index(&self, index: usize) -> Self {
        let mut next = self.0.clone();
        next.push(JsonPathSegment::Index(index));
        Self(next)
    }
}

impl Display for JsonPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return f.write_str("/");
        }

        for segment in &self.0 {
            match segment {
                JsonPathSegment::Key(key) => write!(f, "/{key}")?,
                JsonPathSegment::Index(index) => write!(f, "[{index}]")?,
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct JsonEntry {
    pub path: JsonPath,
    pub depth: usize,
    pub label: String,
    pub preview: String,
    pub editable: bool,
}

pub fn flatten_json(root: &JsonValue) -> Vec<JsonEntry> {
    let mut entries = Vec::new();
    flatten_json_inner(root, &JsonPath::default(), 0, "root", &mut entries);
    entries
}

fn flatten_json_inner(
    value: &JsonValue,
    path: &JsonPath,
    depth: usize,
    label: &str,
    entries: &mut Vec<JsonEntry>,
) {
    entries.push(JsonEntry {
        path: path.clone(),
        depth,
        label: label.to_owned(),
        preview: json_preview(value),
        editable: !matches!(value, JsonValue::Object(_) | JsonValue::Array(_)),
    });

    match value {
        JsonValue::Object(map) => {
            let ordered: BTreeMap<_, _> = map.iter().collect();
            for (key, child) in ordered {
                flatten_json_inner(child, &path.child_key(key), depth + 1, key, entries);
            }
        }
        JsonValue::Array(values) => {
            for (index, child) in values.iter().enumerate() {
                flatten_json_inner(
                    child,
                    &path.child_index(index),
                    depth + 1,
                    &index.to_string(),
                    entries,
                );
            }
        }
        _ => {}
    }
}

pub fn json_preview(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_owned(),
        JsonValue::Bool(value) => value.to_string(),
        JsonValue::Number(value) => value.to_string(),
        JsonValue::String(value) => value.clone(),
        JsonValue::Array(values) => format!("{} items", values.len()),
        JsonValue::Object(values) => format!("{} keys", values.len()),
    }
}

pub fn get_json_mut<'a>(value: &'a mut JsonValue, path: &JsonPath) -> Option<&'a mut JsonValue> {
    let mut current = value;
    for segment in &path.0 {
        current = match (current, segment) {
            (JsonValue::Object(map), JsonPathSegment::Key(key)) => map.get_mut(key)?,
            (JsonValue::Array(values), JsonPathSegment::Index(index)) => values.get_mut(*index)?,
            _ => return None,
        };
    }
    Some(current)
}

pub fn set_json_scalar_from_string(value: &mut JsonValue, path: &JsonPath, input: &str) -> bool {
    let Some(current) = get_json_mut(value, path) else {
        return false;
    };

    let next = match current {
        JsonValue::Null => JsonValue::String(input.to_owned()),
        JsonValue::Bool(_) => JsonValue::Bool(matches!(input.trim(), "true" | "1" | "yes")),
        JsonValue::Number(number) => if number.is_i64() {
            input
                .parse::<i64>()
                .ok()
                .map(Into::into)
                .map(JsonValue::Number)
        } else {
            input
                .parse::<f64>()
                .ok()
                .and_then(serde_json::Number::from_f64)
                .map(JsonValue::Number)
        }
        .unwrap_or_else(|| JsonValue::String(input.to_owned())),
        JsonValue::String(_) => JsonValue::String(input.to_owned()),
        JsonValue::Array(_) | JsonValue::Object(_) => return false,
    };

    *current = next;
    true
}
