use ratatui::{buffer::Buffer, prelude::*};

use crate::{
    domain::minecraft::inventory::{
        InventoryCell, InventoryModel, InventoryRegion, equipment_slot_title,
    },
    tui::{frame::titled_block, theme::Theme},
};

const MAIN_COLUMNS: usize = 9;
const MAIN_ROWS: usize = 4;
const CELL_HEIGHT: u16 = 2;
const ROW_GAP: u16 = 1;
const GRID_GAP: u16 = 2;
const EQUIPMENT_WIDTH: u16 = 14;

pub fn render(frame: &mut Frame, area: Rect, inventory: &InventoryModel, focused: bool) {
    let block = titled_block("Inventory");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 36 || inner.height < 8 {
        frame.render_widget(
            Paragraph::new("Terminal too small for inventory grid.").style(Theme::muted()),
            inner,
        );
        return;
    }

    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(EQUIPMENT_WIDTH)])
        .split(inner);

    render_main_grid(frame.buffer_mut(), split[0], inventory, focused);
    render_equipment(frame.buffer_mut(), split[1], inventory, focused);
}

fn render_main_grid(buf: &mut Buffer, area: Rect, inventory: &InventoryModel, focused: bool) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let gap_x = if area.width >= 70 { 1 } else { 0 };
    let total_gap = gap_x * (MAIN_COLUMNS.saturating_sub(1) as u16);
    let cell_width = (area.width.saturating_sub(total_gap + GRID_GAP)).max(9) / MAIN_COLUMNS as u16;
    let total_used_width = cell_width * MAIN_COLUMNS as u16 + total_gap;
    let origin_x = area.x + area.width.saturating_sub(total_used_width) / 2;
    let origin_y = area.y;

    for row in 0..MAIN_ROWS {
        let y = origin_y + row as u16 * (CELL_HEIGHT + ROW_GAP);
        for column in 0..MAIN_COLUMNS {
            let index = row * MAIN_COLUMNS + column;
            let x = origin_x + column as u16 * (cell_width + gap_x);
            let rect = Rect::new(x, y, cell_width, CELL_HEIGHT);
            let selected = inventory.selection.region == InventoryRegion::Main
                && inventory.selection.index == index;
            render_slot(buf, rect, &inventory.main[index], selected, focused, false);
        }
    }

    let footer_y = origin_y + MAIN_ROWS as u16 * (CELL_HEIGHT + ROW_GAP) - ROW_GAP;
    if footer_y < area.bottom() {
        let footer = "Top rows: inventory  Bottom row: hotbar";
        buf.set_string(origin_x, footer_y, footer, Theme::muted());
    }
}

fn render_equipment(buf: &mut Buffer, area: Rect, inventory: &InventoryModel, focused: bool) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let label_width = 5.min(area.width.saturating_sub(4));
    let slot_x = area.x + label_width + 1;
    let slot_width = area.width.saturating_sub(label_width + 1);
    let mut y = area.y;

    for cell in &inventory.armor {
        let selected = inventory.selection.region == InventoryRegion::Armor
            && inventory.selected_slot_id() == cell.slot_id;
        render_equipment_row(
            buf,
            Rect::new(area.x, y, area.width, CELL_HEIGHT),
            cell,
            selected,
            focused,
            label_width,
            slot_x,
            slot_width,
        );
        y += CELL_HEIGHT + ROW_GAP;
    }

    if y + CELL_HEIGHT <= area.bottom() {
        let selected = inventory.selection.region == InventoryRegion::Offhand;
        render_equipment_row(
            buf,
            Rect::new(area.x, y, area.width, CELL_HEIGHT),
            &inventory.offhand,
            selected,
            focused,
            label_width,
            slot_x,
            slot_width,
        );
    }
}

fn render_equipment_row(
    buf: &mut Buffer,
    area: Rect,
    cell: &InventoryCell,
    selected: bool,
    focused: bool,
    label_width: u16,
    slot_x: u16,
    slot_width: u16,
) {
    let label = truncate(equipment_slot_title(cell.slot_id), label_width as usize);
    buf.set_string(area.x, area.y, label, Theme::muted());
    render_slot(
        buf,
        Rect::new(slot_x, area.y, slot_width, CELL_HEIGHT),
        cell,
        selected,
        focused,
        true,
    );
}

fn render_slot(
    buf: &mut Buffer,
    area: Rect,
    cell: &InventoryCell,
    selected: bool,
    focused: bool,
    compact: bool,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let base_style = if selected {
        if focused {
            Theme::highlight()
        } else {
            Style::default().bg(Color::Rgb(40, 46, 54)).fg(Color::White)
        }
    } else if cell.is_empty() {
        Style::default()
            .bg(Color::Rgb(24, 28, 34))
            .fg(Color::Rgb(120, 130, 140))
    } else {
        Style::default().bg(Color::Rgb(32, 38, 46)).fg(Color::White)
    };

    fill_rect(buf, area, base_style);

    let title_width = area.width.saturating_sub(1) as usize;
    let title = if compact {
        cell.short_label(title_width.min(5))
    } else {
        cell.short_label(title_width.min(7))
    };
    let quantity = cell.quantity_label();

    buf.set_string(
        area.x,
        area.y,
        truncate(&title, area.width as usize),
        base_style,
    );
    if area.height > 1 {
        let qty_x = area.x + area.width.saturating_sub(quantity.len() as u16);
        buf.set_string(qty_x, area.y + 1, quantity, base_style);
    }

    if selected && area.width >= 3 {
        let marker = if focused { ">" } else { "+" };
        buf.set_string(area.x, area.y, marker, base_style);
    }
}

fn fill_rect(buf: &mut Buffer, area: Rect, style: Style) {
    for y in area.y..area.bottom() {
        for x in area.x..area.right() {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_symbol(" ").set_style(style);
            }
        }
    }
}

fn truncate(input: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    input.chars().take(width).collect()
}

use ratatui::widgets::Paragraph;

#[cfg(test)]
mod tests {
    use ratatui::{buffer::Buffer, layout::Rect};

    use super::render_main_grid;
    use crate::domain::minecraft::{
        inventory::{InventoryModel, main_index_for_slot_id},
        item::ItemStack,
    };

    #[test]
    fn main_grid_places_hotbar_item_on_bottom_row() {
        let mut inventory = InventoryModel::default();
        inventory.place(ItemStack {
            slot: 0,
            item_id: "minecraft:stone".to_owned(),
            count: 64,
            damage: None,
            tag: None,
        });

        let mut buffer = Buffer::empty(Rect::new(0, 0, 90, 14));
        render_main_grid(&mut buffer, Rect::new(0, 0, 90, 14), &inventory, true);

        let hotbar_index = main_index_for_slot_id(0).unwrap();
        assert_eq!(hotbar_index, 27);
        assert!(
            buffer
                .content()
                .iter()
                .any(|cell| cell.symbol().contains('6'))
        );
    }
}
