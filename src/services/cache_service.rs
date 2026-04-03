use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::minecraft::profile::Face8x8;

#[derive(Debug, Clone)]
pub enum SkinRecord {
    Ready {
        face: Face8x8,
        name: Option<String>,
        skin_url: Option<String>,
    },
    Unavailable {
        message: String,
        name: Option<String>,
        skin_url: Option<String>,
    },
}

#[derive(Default)]
pub struct SkinCache {
    entries: HashMap<Uuid, SkinRecord>,
}

impl SkinCache {
    pub fn get(&self, uuid: &Uuid) -> Option<&SkinRecord> {
        self.entries.get(uuid)
    }

    pub fn insert(&mut self, uuid: Uuid, record: SkinRecord) {
        self.entries.insert(uuid, record);
    }
}
