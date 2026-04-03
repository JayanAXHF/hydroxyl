use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

use crate::tui::{frame::titled_block, layout::centered_rect};

pub fn render(frame: &mut Frame, area: Rect, title: &str, message: &str) {
    let popup = centered_rect(50, 20, area);
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(message.to_owned()).block(titled_block(title)),
        popup,
    );
}
