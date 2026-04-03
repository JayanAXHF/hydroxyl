use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

use crate::{
    app::state::InputState,
    tui::{frame::titled_block, layout::centered_rect},
};

pub fn render(frame: &mut Frame, area: Rect, input: &InputState) {
    let popup = centered_rect(60, 20, area);
    frame.render_widget(Clear, popup);
    let text = vec![
        Line::raw(input.value.clone()),
        Line::raw(""),
        Line::raw("Enter to confirm, Esc to cancel"),
    ];
    let paragraph = Paragraph::new(text).block(titled_block(&input.title));
    frame.render_widget(paragraph, popup);
}
