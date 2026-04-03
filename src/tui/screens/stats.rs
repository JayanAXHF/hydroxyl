use ratatui::prelude::*;

use crate::{
    app::{
        document::StatsDocument,
        state::{AppState, FocusArea},
    },
    tui::components::nbt_tree,
};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState, document: &StatsDocument) {
    nbt_tree::render_json(
        frame,
        area,
        &document.entries,
        document.selected,
        "Statistics",
        state.focus == FocusArea::GenericTree,
    );
}
