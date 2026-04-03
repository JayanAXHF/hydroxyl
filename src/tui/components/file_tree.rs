use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};

use crate::{
    domain::minecraft::server::WorkspaceEntry,
    tui::{frame::titled_block, theme::Theme},
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    entries: &[WorkspaceEntry],
    selected: usize,
    focused: bool,
) {
    let items = if entries.is_empty() {
        vec![ListItem::new(Span::styled("No files", Theme::muted()))]
    } else {
        entries
            .iter()
            .map(|entry| ListItem::new(entry.display_label()))
            .collect::<Vec<_>>()
    };
    let list = List::new(items)
        .block(titled_block(title))
        .highlight_style(if focused {
            Theme::highlight()
        } else {
            Style::default().bg(Color::Rgb(40, 46, 54))
        });
    let mut state = ListState::default();
    if !entries.is_empty() {
        state.select(Some(selected.min(entries.len() - 1)));
    }
    frame.render_stateful_widget(list, area, &mut state);
}
