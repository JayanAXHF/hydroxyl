use std::path::Path;

use crate::{
    app::document::{
        DocumentKind, DocumentMeta, StatsDocument, flatten_json, set_json_scalar_from_string,
    },
    app::tab_id::DocumentId,
    domain::{files::source::DocumentSource, minecraft::stats::StatsFile},
    persistence::{dirty::DirtyState, json_codec},
    util::{fs::file_name, result::Result},
};

pub struct StatsService;

impl StatsService {
    pub fn open(
        &self,
        id: DocumentId,
        path: &Path,
        source: DocumentSource,
    ) -> Result<StatsDocument> {
        let root = json_codec::read_file(path)?;
        let typed = serde_json::from_value(root.clone()).unwrap_or_default();
        Ok(StatsDocument {
            meta: DocumentMeta {
                id,
                kind: DocumentKind::Stats,
                path: path.to_path_buf(),
                title: file_name(path),
                source,
                dirty: DirtyState::clean(),
            },
            typed,
            entries: flatten_json(&root),
            root,
            selected: 0,
        })
    }

    pub fn refresh(&self, document: &mut StatsDocument) {
        document.typed =
            serde_json::from_value(document.root.clone()).unwrap_or_else(|_| StatsFile::default());
        document.entries = flatten_json(&document.root);
        document.selected = document
            .selected
            .min(document.entries.len().saturating_sub(1));
    }

    pub fn edit_selected(&self, document: &mut StatsDocument, input: &str) -> Result<()> {
        if let Some(entry) = document.entries.get(document.selected) {
            if set_json_scalar_from_string(&mut document.root, &entry.path, input) {
                document.meta.dirty.mark_dirty();
                self.refresh(document);
            }
        }
        Ok(())
    }
}
