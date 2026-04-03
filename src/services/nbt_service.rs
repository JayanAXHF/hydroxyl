use std::path::Path;

use crate::{
    app::document::{DocumentKind, DocumentMeta, NbtDocument},
    app::tab_id::DocumentId,
    domain::{
        files::source::DocumentSource,
        nbt::edit::{flatten, set_scalar_from_string},
        nbt::value::NbtValue,
    },
    persistence::{dirty::DirtyState, nbt_codec},
    util::{fs::file_name, result::Result},
};

pub struct NbtService;

impl NbtService {
    pub fn open(&self, id: DocumentId, path: &Path, source: DocumentSource) -> Result<NbtDocument> {
        let file = nbt_codec::read_file(path)?;
        Ok(NbtDocument {
            meta: DocumentMeta {
                id,
                kind: DocumentKind::Nbt,
                path: path.to_path_buf(),
                title: file_name(path),
                source,
                dirty: DirtyState::clean(),
            },
            compression: file.compression,
            entries: flatten(&file.root),
            root: file.root,
            selected: 0,
        })
    }

    pub fn refresh(&self, document: &mut NbtDocument) {
        document.entries = flatten(&document.root);
        document.selected = document
            .selected
            .min(document.entries.len().saturating_sub(1));
    }

    pub fn edit_scalar(&self, document: &mut NbtDocument, input: &str) -> Result<()> {
        if let Some(entry) = document.entries.get(document.selected) {
            set_scalar_from_string(&mut document.root, &entry.path, input)?;
            document.meta.dirty.mark_dirty();
            self.refresh(document);
        }
        Ok(())
    }

    pub fn set_root(&self, document: &mut NbtDocument, root: NbtValue) {
        document.root = root;
        self.refresh(document);
    }
}
