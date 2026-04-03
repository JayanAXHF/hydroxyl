use std::time::Duration;

use clap::Parser;
use crossterm::event::{self, Event};
use ratatui::{prelude::*, widgets::Clear};

use crate::{
    app::{
        action::{Action, EditTarget},
        bootstrap::build_state,
        context::{AppContext, LaunchConfig},
        document::Document,
        message::Message,
        router::action_from_key,
        state::{AppState, ConfirmState, FocusArea, InputState},
        tab::TabKind,
    },
    cli::{args::Cli, targets::resolve},
    domain::{
        files::source::DocumentSource,
        minecraft::{profile::SkinState, server::WorkspacePane},
        nbt::edit::set_scalar_from_string,
    },
    services::{AppServices, document_service::DocumentService},
    tui::{
        components::{confirm_modal, footer_help, status_bar, tabs, text_input},
        focus::next_focus,
        screens,
        terminal::TerminalGuard,
    },
    util::{error::HydroxylError, result::Result},
};

struct Runtime {
    state: AppState,
    services: AppServices,
    documents: DocumentService,
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();
    let launch = resolve(&cli)?;
    run(launch)
}

pub fn run(launch: LaunchConfig) -> Result<()> {
    let context = AppContext::new(launch);
    let state = build_state(context.clone())?;
    let services = AppServices::new()?;
    let documents = DocumentService::default();
    let mut runtime = Runtime {
        state,
        services,
        documents,
    };

    let initial = runtime
        .documents
        .open_initial(&runtime.services, &context.launch_config.target)?;
    for (document, tab) in initial {
        runtime.add_document(document, tab);
    }

    runtime.update_focus_for_active_tab();
    runtime.request_online_profiles_for_active_tab();

    let mut terminal = TerminalGuard::new()?;
    while !runtime.state.should_quit {
        for update in runtime.services.skins.drain_updates() {
            runtime.apply_skin_update(update);
        }

        terminal
            .terminal_mut()
            .draw(|frame| render_app(frame, &runtime.state))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if runtime.state.confirm.is_some() {
                    match key.code {
                        crossterm::event::KeyCode::Enter => runtime.state.should_quit = true,
                        crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('n') => {
                            runtime.state.confirm = None
                        }
                        _ => {}
                    }
                    continue;
                }

                let action = action_from_key(key, runtime.state.input.is_some());
                runtime.handle_action(action)?;
            }
        }
    }

    Ok(())
}

fn render_app(frame: &mut Frame, state: &AppState) {
    let area = frame.area();
    frame.render_widget(Clear, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    tabs::render(frame, chunks[0], state);
    render_active_screen(frame, chunks[1], state);
    status_bar::render(frame, chunks[2], state);
    footer_help::render(frame, chunks[3]);

    if let Some(input) = &state.input {
        text_input::render(frame, area, input);
    }

    if let Some(confirm) = &state.confirm {
        confirm_modal::render(frame, area, confirm);
    }
}

fn render_active_screen(frame: &mut Frame, area: Rect, state: &AppState) {
    let Some(document) = state.active_document() else {
        return;
    };

    match document {
        Document::Workspace(document) => screens::home::render(frame, area, state, document),
        Document::Player(document) => screens::player::render(frame, area, state, document),
        Document::Nbt(document) => screens::nbt::render(frame, area, state, document),
        Document::Stats(document) => screens::stats::render(frame, area, state, document),
        Document::Advancements(document) => {
            screens::advancements::render(frame, area, state, document)
        }
    }
}

impl Runtime {
    fn add_document(&mut self, document: Document, tab: crate::app::tab::TabState) {
        let path = document.meta().path.clone();
        if let Some((index, _)) = self
            .state
            .documents
            .iter()
            .enumerate()
            .find(|(_, existing)| existing.meta().path == path)
        {
            self.state.active_tab = index;
            self.request_online_profiles_for_active_tab();
            return;
        }

        self.state.documents.push(document);
        self.state.tabs.push(tab);
        self.state.active_tab = self.state.tabs.len().saturating_sub(1);
        self.update_focus_for_active_tab();
        self.request_online_profiles_for_active_tab();
    }

    fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Noop => {}
            Action::NextTab => self.next_tab(),
            Action::PreviousTab => self.previous_tab(),
            Action::FocusNext => self.focus_next(),
            Action::MoveUp => self.move_vertical(-1),
            Action::MoveDown => self.move_vertical(1),
            Action::MoveLeft => self.move_horizontal(-1),
            Action::MoveRight => self.move_horizontal(1),
            Action::SaveActive => self.save_active()?,
            Action::RequestQuit => self.request_quit(),
            Action::StartEdit => self.start_edit(),
            Action::ConfirmEdit => self.confirm_edit()?,
            Action::CancelEdit => self.state.input = None,
            Action::InputChar(value) => {
                if let Some(input) = &mut self.state.input {
                    input.value.insert(input.cursor, value);
                    input.cursor += 1;
                }
            }
            Action::Backspace => {
                if let Some(input) = &mut self.state.input {
                    if input.cursor > 0 {
                        input.cursor -= 1;
                        input.value.remove(input.cursor);
                    }
                }
            }
            Action::OpenSelected => self.open_selected()?,
        }

        Ok(())
    }

    fn next_tab(&mut self) {
        if !self.state.tabs.is_empty() {
            self.state.active_tab = (self.state.active_tab + 1) % self.state.tabs.len();
            self.update_focus_for_active_tab();
            self.request_online_profiles_for_active_tab();
        }
    }

    fn previous_tab(&mut self) {
        if !self.state.tabs.is_empty() {
            self.state.active_tab = if self.state.active_tab == 0 {
                self.state.tabs.len() - 1
            } else {
                self.state.active_tab - 1
            };
            self.update_focus_for_active_tab();
            self.request_online_profiles_for_active_tab();
        }
    }

    fn focus_next(&mut self) {
        let Some(tab_kind) = self.state.active_tab().map(|tab| tab.kind.clone()) else {
            return;
        };

        self.state.focus = next_focus(self.state.focus, &tab_kind);
        if matches!(tab_kind, TabKind::Home(_)) {
            let focus = self.state.focus;
            if let Some(Document::Workspace(document)) = self.state.active_document_mut() {
                document.selection.active_pane = workspace_pane_from_focus(focus);
            }
        }
    }

    fn move_vertical(&mut self, delta: isize) {
        let focus = self.state.focus;
        match self.state.active_document_mut() {
            Some(Document::Workspace(document)) => match focus {
                FocusArea::HomePlayers => {
                    document.selection.active_pane = WorkspacePane::Players;
                    offset_index(
                        &mut document.selection.player_index,
                        delta,
                        document.server.player_entries.len(),
                    )
                }
                FocusArea::HomeStats => {
                    document.selection.active_pane = WorkspacePane::Stats;
                    offset_index(
                        &mut document.selection.stats_index,
                        delta,
                        document.server.stats_entries.len(),
                    )
                }
                FocusArea::HomeAdvancements => {
                    document.selection.active_pane = WorkspacePane::Advancements;
                    offset_index(
                        &mut document.selection.advancements_index,
                        delta,
                        document.server.advancements_entries.len(),
                    )
                }
                _ => {}
            },
            Some(Document::Player(document)) => match focus {
                FocusArea::PlayerSections => offset_index(
                    &mut document.data.selected_section,
                    delta,
                    crate::domain::minecraft::player::PlayerSection::ALL.len(),
                ),
                FocusArea::PlayerInventory => document.data.inventory.move_vertical(delta),
                FocusArea::PlayerFields => {
                    let len = screens::player::property_rows(document).len();
                    offset_index(&mut document.data.field_selected, delta, len)
                }
                FocusArea::RawNbt => offset_index(
                    &mut document.data.raw_selected,
                    delta,
                    document.data.raw_entries.len(),
                ),
                FocusArea::HomePlayers
                | FocusArea::HomeStats
                | FocusArea::HomeAdvancements
                | FocusArea::GenericTree => {}
            },
            Some(Document::Nbt(document)) => {
                offset_index(&mut document.selected, delta, document.entries.len())
            }
            Some(Document::Stats(document)) => {
                offset_index(&mut document.selected, delta, document.entries.len())
            }
            Some(Document::Advancements(document)) => {
                offset_index(&mut document.selected, delta, document.entries.len())
            }
            None => {}
        }
    }

    fn move_horizontal(&mut self, delta: isize) {
        let focus = self.state.focus;
        match self.state.active_document_mut() {
            Some(Document::Workspace(document)) => {
                let next = shift_home_pane(document.selection.active_pane, delta);
                document.selection.active_pane = next;
                self.state.focus = focus_for_workspace_pane(next);
            }
            Some(Document::Player(document)) => match focus {
                FocusArea::PlayerInventory => document.data.inventory.move_horizontal(delta),
                FocusArea::PlayerSections => offset_index(
                    &mut document.data.selected_section,
                    delta,
                    crate::domain::minecraft::player::PlayerSection::ALL.len(),
                ),
                FocusArea::PlayerFields => {
                    let len = screens::player::property_rows(document).len();
                    offset_index(&mut document.data.field_selected, delta, len)
                }
                FocusArea::RawNbt => offset_index(
                    &mut document.data.raw_selected,
                    delta,
                    document.data.raw_entries.len(),
                ),
                _ => {}
            },
            Some(Document::Nbt(document)) => {
                offset_index(&mut document.selected, delta, document.entries.len())
            }
            Some(Document::Stats(document)) => {
                offset_index(&mut document.selected, delta, document.entries.len())
            }
            Some(Document::Advancements(document)) => {
                offset_index(&mut document.selected, delta, document.entries.len())
            }
            _ => {}
        }
    }

    fn start_edit(&mut self) {
        let focus = self.state.focus;
        let Some(document) = self.state.active_document() else {
            return;
        };

        let input = match document {
            Document::Player(document) => screens::player::edit_target(document, focus),
            Document::Nbt(document) => document.entries.get(document.selected).and_then(|entry| {
                entry.editable.then(|| {
                    (
                        format!("Edit {}", entry.path),
                        entry.preview.clone(),
                        EditTarget::NbtValue {
                            document_id: document.meta.id,
                            path: entry.path.clone(),
                        },
                    )
                })
            }),
            Document::Stats(document) => {
                document.entries.get(document.selected).and_then(|entry| {
                    entry.editable.then(|| {
                        (
                            format!("Edit {}", entry.path),
                            entry.preview.clone(),
                            EditTarget::JsonValue {
                                document_id: document.meta.id,
                                path: entry.path.clone(),
                            },
                        )
                    })
                })
            }
            Document::Advancements(document) => {
                document.entries.get(document.selected).and_then(|entry| {
                    entry.editable.then(|| {
                        (
                            format!("Edit {}", entry.path),
                            entry.preview.clone(),
                            EditTarget::JsonValue {
                                document_id: document.meta.id,
                                path: entry.path.clone(),
                            },
                        )
                    })
                })
            }
            Document::Workspace(_) => None,
        };

        if let Some((title, value, target)) = input {
            let cursor = value.len();
            self.state.input = Some(InputState {
                title,
                value,
                cursor,
                target,
            });
        }
    }

    fn confirm_edit(&mut self) -> Result<()> {
        let Some(input) = self.state.input.take() else {
            return Ok(());
        };

        let value = input.value;
        match input.target {
            EditTarget::PlayerField { document_id, field } => {
                let document = find_document_mut(&mut self.state, document_id)?;
                let Document::Player(document) = document else {
                    return Err(HydroxylError::invalid_data(
                        "edit target was not a player document",
                    ));
                };
                self.services.player.edit_field(document, &field, &value)?;
                self.request_skin_for_active_player();
            }
            EditTarget::NbtValue { document_id, path } => {
                let document = find_document_mut(&mut self.state, document_id)?;
                match document {
                    Document::Player(document) => {
                        set_scalar_from_string(&mut document.root, &path, &value)?;
                        document.meta.dirty.mark_dirty();
                        self.services.player.refresh(document);
                    }
                    Document::Nbt(document) => {
                        set_scalar_from_string(&mut document.root, &path, &value)?;
                        document.meta.dirty.mark_dirty();
                        self.services.nbt.refresh(document);
                    }
                    _ => return Err(HydroxylError::invalid_data("NBT edit target was invalid")),
                }
            }
            EditTarget::JsonValue { document_id, path } => {
                let document = find_document_mut(&mut self.state, document_id)?;
                match document {
                    Document::Stats(document) => {
                        if let Some(position) =
                            document.entries.iter().position(|entry| entry.path == path)
                        {
                            document.selected = position;
                            self.services.stats.edit_selected(document, &value)?;
                        }
                    }
                    Document::Advancements(document) => {
                        if let Some(position) =
                            document.entries.iter().position(|entry| entry.path == path)
                        {
                            document.selected = position;
                            self.services.advancements.edit_selected(document, &value)?;
                        }
                    }
                    _ => return Err(HydroxylError::invalid_data("JSON edit target was invalid")),
                }
            }
        }

        Ok(())
    }

    fn save_active(&mut self) -> Result<()> {
        let active_document_id = self.state.active_document().map(|document| document.id());
        let Some(document_id) = active_document_id else {
            return Ok(());
        };

        let message = {
            let document = find_document_mut(&mut self.state, document_id)?;
            if matches!(document, Document::Workspace(_)) {
                None
            } else {
                let backup = self.services.backup.create_backup(&document.meta().path)?;
                self.services.save.save(document)?;
                Some(format!(
                    "Saved {} (backup: {})",
                    document.title(),
                    backup.display()
                ))
            }
        };

        if let Some(message) = message {
            self.state.push_message(Message::info(message));
        }
        Ok(())
    }

    fn request_quit(&mut self) {
        if self.state.documents.iter().any(Document::is_dirty) {
            self.state.confirm = Some(ConfirmState {
                title: "Unsaved changes".to_owned(),
                message: "There are unsaved changes. Quit anyway?".to_owned(),
            });
        } else {
            self.state.should_quit = true;
        }
    }

    fn open_selected(&mut self) -> Result<()> {
        let Some(Document::Workspace(document)) = self.state.active_document() else {
            return Ok(());
        };
        let Some(entry) = document.selected_entry() else {
            return Ok(());
        };
        let source = DocumentSource::Workspace {
            root: document.server.root.clone(),
        };
        let server = Some(&document.server);
        let (document, tab) =
            self.documents
                .open_path(&self.services, &entry.path, source, server)?;
        self.add_document(document, tab);
        Ok(())
    }

    fn update_focus_for_active_tab(&mut self) {
        let Some(tab) = self.state.active_tab() else {
            return;
        };

        self.state.focus = match tab.kind {
            TabKind::Home(id) => self
                .state
                .documents
                .iter()
                .find_map(|document| match document {
                    Document::Workspace(document) if document.meta.id == id => {
                        Some(focus_for_workspace_pane(document.selection.active_pane))
                    }
                    _ => None,
                })
                .unwrap_or(FocusArea::HomePlayers),
            TabKind::Player(_) => FocusArea::PlayerSections,
            TabKind::Nbt(_) | TabKind::Stats(_) | TabKind::Advancements(_) => {
                FocusArea::GenericTree
            }
        };
    }

    fn request_online_profiles_for_active_tab(&mut self) {
        self.request_workspace_usernames_for_active_home();
        self.request_skin_for_active_player();
    }

    fn request_workspace_usernames_for_active_home(&mut self) {
        let Some(Document::Workspace(document)) = self.state.active_document() else {
            return;
        };
        if !document.server.online_mode {
            return;
        }

        let uuids = document
            .server
            .player_entries
            .iter()
            .filter_map(|entry| entry.uuid)
            .collect::<Vec<_>>();

        for uuid in uuids {
            if let Some(update) = self.services.skins.request(uuid) {
                self.apply_skin_update(update);
            }
        }
    }

    fn request_skin_for_active_player(&mut self) {
        let Some(Document::Player(document)) = self.state.active_document() else {
            return;
        };

        let online_mode = document
            .server
            .as_ref()
            .map(|server| server.online_mode)
            .unwrap_or(false);
        if !online_mode {
            return;
        }

        let document_id = document.meta.id;
        let Some(uuid) = document.data.identity.uuid else {
            if let Ok(Document::Player(document)) = find_document_mut(&mut self.state, document_id)
            {
                document.skin_state =
                    SkinState::Unavailable("No UUID available for Mojang lookup".to_owned());
            }
            return;
        };

        if let Ok(Document::Player(document)) = find_document_mut(&mut self.state, document_id) {
            document.skin_state = SkinState::Loading;
        }
        if let Some(update) = self.services.skins.request(uuid) {
            self.apply_skin_update(update);
        }
    }

    fn apply_skin_update(&mut self, update: crate::app::event::SkinUpdate) {
        for document in &mut self.state.documents {
            match document {
                Document::Player(document) => {
                    if document.data.identity.uuid != Some(update.uuid) {
                        continue;
                    }

                    if let Some(name) = &update.resolved_name {
                        document.data.identity.name = Some(name.clone());
                    }
                    if update.skin_url.is_some() {
                        document.data.identity.skin_url = update.skin_url.clone();
                    }

                    match &update.result {
                        Ok(face) => {
                            document.skin_state = SkinState::Ready(face.clone());
                        }
                        Err(message) => {
                            document.skin_state = SkinState::Unavailable(message.clone());
                        }
                    }
                }
                Document::Workspace(document) => {
                    for entry in &mut document.server.player_entries {
                        if entry.uuid == Some(update.uuid) {
                            entry.resolved_name = update.resolved_name.clone();
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn find_document_mut(
    state: &mut AppState,
    id: crate::app::tab_id::DocumentId,
) -> Result<&mut Document> {
    state
        .documents
        .iter_mut()
        .find(|document| document.id() == id)
        .ok_or_else(|| HydroxylError::invalid_data(format!("unknown document id {}", id.0)))
}

fn offset_index(index: &mut usize, delta: isize, len: usize) {
    if len == 0 {
        *index = 0;
        return;
    }

    let current = *index as isize;
    let next = (current + delta).clamp(0, (len - 1) as isize) as usize;
    *index = next;
}

fn shift_home_pane(current: WorkspacePane, delta: isize) -> WorkspacePane {
    let current_index = match current {
        WorkspacePane::Players => 0,
        WorkspacePane::Stats => 1,
        WorkspacePane::Advancements => 2,
    };
    let next_index = (current_index as isize + delta).clamp(0, 2) as usize;
    WorkspacePane::ALL[next_index]
}

fn focus_for_workspace_pane(pane: WorkspacePane) -> FocusArea {
    match pane {
        WorkspacePane::Players => FocusArea::HomePlayers,
        WorkspacePane::Stats => FocusArea::HomeStats,
        WorkspacePane::Advancements => FocusArea::HomeAdvancements,
    }
}

fn workspace_pane_from_focus(focus: FocusArea) -> WorkspacePane {
    match focus {
        FocusArea::HomePlayers => WorkspacePane::Players,
        FocusArea::HomeStats => WorkspacePane::Stats,
        FocusArea::HomeAdvancements => WorkspacePane::Advancements,
        _ => WorkspacePane::Players,
    }
}
