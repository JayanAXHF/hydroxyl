use ratatui::style::{Color, Modifier, Style};

pub struct Theme;

impl Theme {
    pub fn background() -> Color {
        Color::Rgb(16, 20, 24)
    }

    pub fn panel_border() -> Color {
        Color::Rgb(94, 110, 125)
    }

    pub fn accent() -> Color {
        Color::Rgb(88, 165, 255)
    }

    pub fn highlight() -> Style {
        Style::default()
            .fg(Color::Black)
            .bg(Self::accent())
            .add_modifier(Modifier::BOLD)
    }

    pub fn muted() -> Style {
        Style::default().fg(Color::Rgb(150, 160, 170))
    }
}
