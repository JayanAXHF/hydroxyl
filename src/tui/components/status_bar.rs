use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    app::{document::Document, state::AppState},
    tui::theme::Theme,
};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let dirty = state
        .active_document()
        .map(Document::is_dirty)
        .unwrap_or(false);
    let status = if dirty { "dirty" } else { "saved" };
    let text = format!("{}  |  {}", state.current_path_label(), status);
    frame.render_widget(Paragraph::new(text).style(Theme::muted()), area);
}
