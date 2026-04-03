use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

use crate::{
    app::state::ConfirmState,
    tui::{frame::titled_block, layout::centered_rect},
};

pub fn render(frame: &mut Frame, area: Rect, confirm: &ConfirmState) {
    let popup = centered_rect(50, 20, area);
    frame.render_widget(Clear, popup);
    let text = vec![
        Line::raw(confirm.message.clone()),
        Line::raw(""),
        Line::raw("Enter to confirm, Esc to cancel"),
    ];
    frame.render_widget(
        Paragraph::new(text).block(titled_block(&confirm.title)),
        popup,
    );
}
