use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    domain::minecraft::attributes::PlayerAttributes,
    tui::{frame::titled_block, theme::Theme},
};

pub fn render(frame: &mut Frame, area: Rect, attributes: &PlayerAttributes) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Health: ", Theme::muted()),
            Span::raw(format!("{:.1}", attributes.health)),
        ]),
        Line::from(vec![
            Span::styled("Food:   ", Theme::muted()),
            Span::raw(attributes.food_level.to_string()),
        ]),
        Line::from(vec![
            Span::styled("XP Lv:  ", Theme::muted()),
            Span::raw(attributes.xp_level.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Air:    ", Theme::muted()),
            Span::raw(attributes.air.to_string()),
        ]),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(titled_block("Quick Stats")),
        area,
    );
}
