use std::path::Path;

use crate::{
    app::{
        context::OpenTarget,
        document::{Document, DocumentKind, DocumentMeta, WorkspaceDocument},
        tab::{TabKind, TabState},
        tab_id::{DocumentId, TabId},
    },
    domain::{
        files::{
            detect::{detect_file_kind, infer_server_root, infer_world_root},
            kind::FileKind,
            source::DocumentSource,
        },
        minecraft::server::{ServerContext, WorkspaceSelection},
    },
    persistence::dirty::DirtyState,
    services::AppServices,
    util::{error::HydroxylError, fs::file_name, result::Result},
};

#[derive(Default)]
pub struct DocumentService {
    next_document: usize,
    next_tab: usize,
}

impl DocumentService {
    fn next_document_id(&mut self) -> DocumentId {
        let id = self.next_document;
        self.next_document += 1;
        DocumentId(id)
    }

    fn next_tab_id(&mut self) -> TabId {
        let id = self.next_tab;
        self.next_tab += 1;
        TabId(id)
    }

    pub fn open_initial(
        &mut self,
        services: &AppServices,
        target: &OpenTarget,
    ) -> Result<Vec<(Document, TabState)>> {
        match target {
            OpenTarget::Home => Ok(vec![self.blank_home_tab()]),
            OpenTarget::World(path) => {
                let server = services.workspace.load(path)?;
                let document = self.workspace_document(path, server, true);
                Ok(vec![document])
            }
            OpenTarget::Player(path) => {
                let server = infer_server_root(path)
                    .or_else(|| infer_world_root(path))
                    .map(|root| services.workspace.load(&root))
                    .transpose()?;
                let document =
                    self.open_path(services, path, DocumentSource::Direct, server.as_ref())?;
                Ok(vec![document])
            }
            OpenTarget::Nbt(path) | OpenTarget::Stats(path) | OpenTarget::Advancements(path) => {
                let document = self.open_path(services, path, DocumentSource::Direct, None)?;
                Ok(vec![document])
            }
        }
    }

    pub fn open_path(
        &mut self,
        services: &AppServices,
        path: &Path,
        source: DocumentSource,
        server: Option<&ServerContext>,
    ) -> Result<(Document, TabState)> {
        let kind = detect_file_kind(path);
        let id = self.next_document_id();
        let (document, tab_kind) = match kind {
            FileKind::Workspace => {
                let server = services.workspace.load(path)?;
                let (document, tab) = self.workspace_document(path, server, false);
                return Ok((document, tab));
            }
            FileKind::PlayerData => {
                let document =
                    Document::Player(services.player.open(id, path, source, server.cloned())?);
                (document, TabKind::Player(id))
            }
            FileKind::Stats => {
                let document = Document::Stats(services.stats.open(id, path, source)?);
                (document, TabKind::Stats(id))
            }
            FileKind::Advancements => {
                let document =
                    Document::Advancements(services.advancements.open(id, path, source)?);
                (document, TabKind::Advancements(id))
            }
            FileKind::Nbt => {
                let document = Document::Nbt(services.nbt.open(id, path, source)?);
                (document, TabKind::Nbt(id))
            }
            FileKind::Unknown => {
                return Err(HydroxylError::invalid_data(format!(
                    "unsupported file type: {}",
                    path.display()
                )));
            }
        };

        let tab = TabState {
            id: self.next_tab_id(),
            title: file_name(path),
            kind: tab_kind,
        };
        Ok((document, tab))
    }

    pub fn workspace_document(
        &mut self,
        path: &Path,
        server: ServerContext,
        title_is_home: bool,
    ) -> (Document, TabState) {
        let id = self.next_document_id();
        let title = if title_is_home {
            format!("Home: {}", server.level_name)
        } else {
            file_name(path)
        };
        let document = Document::Workspace(WorkspaceDocument {
            meta: DocumentMeta {
                id,
                kind: DocumentKind::Workspace,
                path: path.to_path_buf(),
                title: title.clone(),
                source: DocumentSource::Direct,
                dirty: DirtyState::clean(),
            },
            server,
            selection: WorkspaceSelection::default(),
        });
        let tab = TabState {
            id: self.next_tab_id(),
            title,
            kind: TabKind::Home(id),
        };
        (document, tab)
    }

    pub fn create_home_tab(&mut self) -> (Document, TabState) {
        self.blank_home_tab()
    }

    fn blank_home_tab(&mut self) -> (Document, TabState) {
        let id = self.next_document_id();
        let title = "Home".to_owned();
        let document = Document::Workspace(WorkspaceDocument {
            meta: DocumentMeta {
                id,
                kind: DocumentKind::Workspace,
                path: std::path::PathBuf::new(),
                title: title.clone(),
                source: DocumentSource::Direct,
                dirty: DirtyState::clean(),
            },
            server: ServerContext::default(),
            selection: WorkspaceSelection::default(),
        });
        let tab = TabState {
            id: self.next_tab_id(),
            title,
            kind: TabKind::Home(id),
        };
        (document, tab)
    }
}
