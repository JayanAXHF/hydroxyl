use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    domain::minecraft::{inventory::slot_label, item::ItemStack},
    tui::frame::titled_block,
};

pub fn render(frame: &mut Frame, area: Rect, slots: &[ItemStack], selected: usize, focused: bool) {
    let mut lines = Vec::new();
    for row in slots.chunks(9) {
        let mut spans = Vec::new();
        for item in row {
            let title = if item.item_id.is_empty() {
                "Empty".to_owned()
            } else {
                item.item_id
                    .rsplit(':')
                    .next()
                    .unwrap_or(&item.item_id)
                    .chars()
                    .take(6)
                    .collect()
            };
            let marker = slots
                .get(selected)
                .map(|selected_item| selected_item.slot == item.slot)
                .unwrap_or(false);
            let style = if marker {
                if focused {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Rgb(88, 165, 255))
                } else {
                    Style::default().bg(Color::Rgb(40, 46, 54))
                }
            } else {
                Style::default()
            };
            spans.push(Span::styled(format!(" {:<6}", title), style));
        }
        lines.push(Line::from(spans));
    }

    if let Some(item) = slots.get(selected) {
        lines.push(Line::raw(""));
        lines.push(Line::styled(
            format!("Selected: {}", slot_label(item.slot)),
            Style::default().fg(Color::Rgb(150, 160, 170)),
        ));
    }

    frame.render_widget(Paragraph::new(lines).block(titled_block("Inventory")), area);
}
