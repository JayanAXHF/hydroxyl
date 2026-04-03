use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    domain::minecraft::inventory::{InventoryCell, slot_label},
    tui::{frame::titled_block, theme::Theme},
};

pub fn render(frame: &mut Frame, area: Rect, cell: Option<&InventoryCell>) {
    let lines = if let Some(cell) = cell {
        if let Some(item) = &cell.item {
            vec![
                Line::from(vec![
                    Span::styled("Slot: ", Theme::muted()),
                    Span::raw(slot_label(cell.slot_id)),
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
            vec![
                Line::from(vec![
                    Span::styled("Slot: ", Theme::muted()),
                    Span::raw(slot_label(cell.slot_id)),
                ]),
                Line::from(vec![
                    Span::styled("Item: ", Theme::muted()),
                    Span::raw("Empty"),
                ]),
                Line::from(vec![
                    Span::styled("Count:", Theme::muted()),
                    Span::raw(" 0"),
                ]),
                Line::from(vec![
                    Span::styled("Notes:", Theme::muted()),
                    Span::raw(" empty slot"),
                ]),
            ]
        }
    } else {
        vec![Line::raw("No slot selected")]
    };

    frame.render_widget(Paragraph::new(lines).block(titled_block("Preview")), area);
}
