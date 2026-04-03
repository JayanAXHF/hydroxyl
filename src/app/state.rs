use std::collections::VecDeque;

use crate::{
    app::{
        action::EditTarget, context::AppContext, document::Document, message::Message,
        tab::TabState, tab_id::DocumentId,
    },
    util::fs::file_name,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusArea {
    HomePlayers,
    HomeStats,
    HomeAdvancements,
    PlayerSections,
    PlayerInventory,
    PlayerFields,
    RawNbt,
    GenericTree,
}

#[derive(Debug, Clone)]
pub struct InputState {
    pub title: String,
    pub value: String,
    pub cursor: usize,
    pub target: EditTarget,
}

#[derive(Debug, Clone, Copy)]
pub enum ConfirmAction {
    Quit,
    CloseTab(DocumentId),
}

#[derive(Debug, Clone)]
pub struct ConfirmState {
    pub title: String,
    pub message: String,
    pub action: ConfirmAction,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub context: AppContext,
    pub documents: Vec<Document>,
    pub tabs: Vec<TabState>,
    pub active_tab: usize,
    pub focus: FocusArea,
    pub input: Option<InputState>,
    pub confirm: Option<ConfirmState>,
    pub messages: VecDeque<Message>,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(context: AppContext) -> Self {
        Self {
            context,
            documents: Vec::new(),
            tabs: Vec::new(),
            active_tab: 0,
            focus: FocusArea::HomePlayers,
            input: None,
            confirm: None,
            messages: VecDeque::new(),
            should_quit: false,
        }
    }

    pub fn push_message(&mut self, message: Message) {
        self.messages.push_front(message);
        while self.messages.len() > 4 {
            self.messages.pop_back();
        }
    }

    pub fn active_tab(&self) -> Option<&TabState> {
        self.tabs.get(self.active_tab)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut TabState> {
        self.tabs.get_mut(self.active_tab)
    }

    pub fn active_document(&self) -> Option<&Document> {
        let tab = self.active_tab()?;
        let id = match tab.kind {
            crate::app::tab::TabKind::Home(id)
            | crate::app::tab::TabKind::Player(id)
            | crate::app::tab::TabKind::Nbt(id)
            | crate::app::tab::TabKind::Stats(id)
            | crate::app::tab::TabKind::Advancements(id) => id,
        };
        self.documents.iter().find(|document| document.id() == id)
    }

    pub fn active_document_mut(&mut self) -> Option<&mut Document> {
        let tab = self.active_tab()?.clone();
        let id = match tab.kind {
            crate::app::tab::TabKind::Home(id)
            | crate::app::tab::TabKind::Player(id)
            | crate::app::tab::TabKind::Nbt(id)
            | crate::app::tab::TabKind::Stats(id)
            | crate::app::tab::TabKind::Advancements(id) => id,
        };
        self.documents
            .iter_mut()
            .find(|document| document.id() == id)
    }

    pub fn tab_titles(&self) -> Vec<String> {
        self.tabs
            .iter()
            .map(|tab| {
                let active_title = self
                    .documents
                    .iter()
                    .find(|document| document.title() == tab.title)
                    .map(|document| {
                        if document.is_dirty() {
                            format!("{}*", document.title())
                        } else {
                            document.title().to_owned()
                        }
                    });
                active_title.unwrap_or_else(|| tab.title.clone())
            })
            .collect()
    }

    pub fn current_path_label(&self) -> String {
        self.active_document()
            .map(|document| file_name(&document.meta().path))
            .unwrap_or_else(|| "No document".to_owned())
    }
}
