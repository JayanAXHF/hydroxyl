use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};

use crate::{
    app::document::JsonEntry,
    domain::nbt::edit::NbtEntry,
    tui::{frame::titled_block, theme::Theme},
};

pub fn render_nbt(
    frame: &mut Frame,
    area: Rect,
    entries: &[NbtEntry],
    selected: usize,
    title: &str,
    focused: bool,
) {
    let items = entries
        .iter()
        .map(|entry| {
            let indent = "  ".repeat(entry.depth);
            ListItem::new(Line::from(vec![
                Span::raw(indent),
                Span::styled(format!("{} ", entry.label), Theme::muted()),
                Span::raw(format!("{} = {}", entry.type_name, entry.preview)),
            ]))
        })
        .collect::<Vec<_>>();
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

pub fn render_json(
    frame: &mut Frame,
    area: Rect,
    entries: &[JsonEntry],
    selected: usize,
    title: &str,
    focused: bool,
) {
    let items = entries
        .iter()
        .map(|entry| {
            let indent = "  ".repeat(entry.depth);
            ListItem::new(Line::from(vec![
                Span::raw(indent),
                Span::styled(format!("{} ", entry.label), Theme::muted()),
                Span::raw(entry.preview.clone()),
            ]))
        })
        .collect::<Vec<_>>();
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
