use ratatui::{prelude::*, widgets::Paragraph};

use crate::tui::theme::Theme;

pub fn render(frame: &mut Frame, area: Rect, text: &str) {
    frame.render_widget(Paragraph::new(text).style(Theme::muted()), area);
}
