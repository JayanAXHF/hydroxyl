use crate::{
    app::document::Document,
    persistence::{json_codec, nbt_codec},
    util::result::Result,
};

pub struct SaveService;

impl SaveService {
    pub fn save(&self, document: &mut Document) -> Result<()> {
        match document {
            Document::Workspace(_) => Ok(()),
            Document::Player(document) => {
                nbt_codec::write_file(&document.meta.path, &document.root, document.compression)?;
                document.meta.dirty.mark_clean();
                Ok(())
            }
            Document::Nbt(document) => {
                nbt_codec::write_file(&document.meta.path, &document.root, document.compression)?;
                document.meta.dirty.mark_clean();
                Ok(())
            }
            Document::Stats(document) => {
                json_codec::write_file(&document.meta.path, &document.root)?;
                document.meta.dirty.mark_clean();
                Ok(())
            }
            Document::Advancements(document) => {
                json_codec::write_file(&document.meta.path, &document.root)?;
                document.meta.dirty.mark_clean();
                Ok(())
            }
        }
    }
}
