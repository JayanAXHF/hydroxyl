use ratatui::{
    style::Style,
    widgets::{Block, Borders},
};

use crate::tui::theme::Theme;

pub fn titled_block(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Theme::panel_border()))
}
