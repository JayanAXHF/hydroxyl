use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    domain::minecraft::position::PlayerPosition,
    tui::{frame::titled_block, theme::Theme},
};

pub fn render(frame: &mut Frame, area: Rect, position: &PlayerPosition) {
    let lines = vec![
        Line::from(vec![
            Span::styled("X: ", Theme::muted()),
            Span::raw(format!("{:.2}", position.x)),
        ]),
        Line::from(vec![
            Span::styled("Y: ", Theme::muted()),
            Span::raw(format!("{:.2}", position.y)),
        ]),
        Line::from(vec![
            Span::styled("Z: ", Theme::muted()),
            Span::raw(format!("{:.2}", position.z)),
        ]),
        Line::from(vec![
            Span::styled("Dim: ", Theme::muted()),
            Span::raw(position.dimension.clone()),
        ]),
    ];
    frame.render_widget(Paragraph::new(lines).block(titled_block("Position")), area);
}
