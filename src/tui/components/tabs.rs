use ratatui::{prelude::*, widgets::Tabs};

use crate::{app::state::AppState, tui::theme::Theme};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let titles = state.tab_titles();
    let widget = Tabs::new(titles)
        .select(state.active_tab)
        .highlight_style(Theme::highlight())
        .style(Theme::muted());
    frame.render_widget(widget, area);
}
