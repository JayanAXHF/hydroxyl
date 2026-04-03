use ratatui::prelude::*;

use crate::{
    app::{
        document::AdvancementsDocument,
        state::{AppState, FocusArea},
    },
    tui::components::nbt_tree,
};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState, document: &AdvancementsDocument) {
    nbt_tree::render_json(
        frame,
        area,
        &document.entries,
        document.selected,
        "Advancements",
        state.focus == FocusArea::GenericTree,
    );
}
