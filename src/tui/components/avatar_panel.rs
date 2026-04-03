use ratatui::{buffer::Buffer, prelude::*, widgets::Paragraph};

use crate::{
    domain::minecraft::profile::{Face8x8, SkinState},
    tui::{frame::titled_block, theme::Theme},
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    skin_state: &SkinState,
    name: Option<&str>,
    uuid: Option<String>,
) {
    let block = titled_block("Avatar");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width == 0 || inner.height == 0 {
        return;
    }

    let placeholder = Face8x8::placeholder();
    let (face, status) = match skin_state {
        SkinState::Ready(face) => (face, "Mojang skin loaded"),
        SkinState::Loading => (&placeholder, "Loading Mojang skin..."),
        SkinState::Unavailable(_) => (&placeholder, "Mojang skin unavailable"),
        SkinState::NotRequested => (&placeholder, "Skin lookup idle"),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(8),
            Constraint::Length(match skin_state {
                SkinState::Unavailable(_) => 2,
                _ => 1,
            }),
        ])
        .split(inner);

    let info = vec![
        Line::from(vec![
            Span::styled("Name: ", Theme::muted()),
            Span::raw(name.unwrap_or("Unknown")),
        ]),
        Line::from(vec![
            Span::styled("UUID: ", Theme::muted()),
            Span::raw(uuid.unwrap_or_else(|| "n/a".to_owned())),
        ]),
    ];
    frame.render_widget(Paragraph::new(info), chunks[0]);

    render_face_grid(frame.buffer_mut(), chunks[1], face);

    let mut footer = vec![Line::styled(status, Theme::muted())];
    if let SkinState::Unavailable(message) = skin_state {
        footer.push(Line::raw(message.clone()));
    }
    frame.render_widget(Paragraph::new(footer), chunks[2]);
}

fn render_face_grid(buf: &mut Buffer, area: Rect, face: &Face8x8) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let pixel_width = if area.width >= 16 { 2 } else { 1 };
    let face_width = 8 * pixel_width;
    let face_height = 8;
    let origin_x = area.x + area.width.saturating_sub(face_width) / 2;
    let origin_y = area.y + area.height.saturating_sub(face_height) / 2;
    let symbol = if pixel_width == 2 { "██" } else { "█" };

    for (row_index, row) in face.pixels.iter().enumerate() {
        let y = origin_y + row_index as u16;
        if y >= area.bottom() {
            break;
        }

        for (column_index, pixel) in row.iter().enumerate() {
            let x = origin_x + (column_index as u16 * pixel_width);
            if x >= area.right() {
                break;
            }

            let style = Style::default().fg(pixel.color).bg(Color::Black);
            if pixel_width == 2 && x + 1 < area.right() {
                buf.set_string(x, y, symbol, style);
            } else {
                buf.set_string(x, y, "█", style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{buffer::Buffer, layout::Rect, style::Color};

    use super::render_face_grid;
    use crate::domain::minecraft::profile::Face8x8;

    #[test]
    fn face_grid_uses_visible_blocks() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 12));
        render_face_grid(
            &mut buffer,
            Rect::new(0, 0, 20, 12),
            &Face8x8::placeholder(),
        );

        let block_count = buffer
            .content()
            .iter()
            .filter(|cell| cell.symbol() == "█")
            .count();
        assert!(block_count >= 64);
        assert!(
            buffer
                .content()
                .iter()
                .any(|cell| matches!(cell.fg, Color::Rgb(_, _, _)))
        );
    }
}
