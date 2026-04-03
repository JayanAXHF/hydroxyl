use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    domain::minecraft::{inventory::slot_label, item::ItemStack},
    tui::{frame::titled_block, theme::Theme},
};

pub fn render(frame: &mut Frame, area: Rect, item: Option<&ItemStack>) {
    let lines = if let Some(item) = item {
        vec![
            Line::from(vec![
                Span::styled("Slot: ", Theme::muted()),
                Span::raw(slot_label(item.slot)),
            ]),
            Line::from(vec![
                Span::styled("Item: ", Theme::muted()),
                Span::raw(item.item_id.clone()),
            ]),
            Line::from(vec![
                Span::styled("Count:", Theme::muted()),
                Span::raw(format!(" {}", item.count)),
            ]),
            Line::from(vec![
                Span::styled("Damage:", Theme::muted()),
                Span::raw(format!(" {}", item.damage.unwrap_or_default())),
            ]),
            Line::from(vec![
                Span::styled("Tag: ", Theme::muted()),
                Span::raw(if item.tag.is_some() {
                    "present"
                } else {
                    "none"
                }),
            ]),
        ]
    } else {
        vec![Line::raw("No slot selected")]
    };

    frame.render_widget(
        Paragraph::new(lines).block(titled_block("Selected Item")),
        area,
    );
}
