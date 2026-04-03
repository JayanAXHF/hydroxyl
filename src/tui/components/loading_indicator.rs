use ratatui::{prelude::*, widgets::Paragraph};

pub fn render(frame: &mut Frame, area: Rect, label: &str) {
    frame.render_widget(Paragraph::new(label.to_owned()), area);
}
