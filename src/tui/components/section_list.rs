use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};

use crate::tui::{frame::titled_block, theme::Theme};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    items: &[String],
    selected: usize,
    focused: bool,
) {
    let list = List::new(
        items
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect::<Vec<_>>(),
    )
    .block(titled_block(title))
    .highlight_style(if focused {
        Theme::highlight()
    } else {
        Style::default().bg(Color::Rgb(40, 46, 54))
    });
    let mut state = ListState::default();
    if !items.is_empty() {
        state.select(Some(selected.min(items.len() - 1)));
    }
    frame.render_stateful_widget(list, area, &mut state);
}
