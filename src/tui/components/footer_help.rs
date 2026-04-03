use ratatui::{prelude::*, widgets::Paragraph};

use crate::tui::{keymap, theme::Theme};

pub fn render(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new(keymap::help_text()).style(Theme::muted()),
        area,
    );
}
