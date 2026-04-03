use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};

use crate::tui::{frame::titled_block, theme::Theme};

#[derive(Debug, Clone)]
pub struct PropertyRow {
    pub label: String,
    pub value: String,
}

pub fn render(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    rows: &[PropertyRow],
    selected: usize,
    focused: bool,
) {
    let items = rows
        .iter()
        .map(|row| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:<18}", row.label), Theme::muted()),
                Span::raw(row.value.clone()),
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
    if !rows.is_empty() {
        state.select(Some(selected.min(rows.len() - 1)));
    }
    frame.render_stateful_widget(list, area, &mut state);
}
