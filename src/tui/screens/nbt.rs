use ratatui::prelude::*;

use crate::{
    app::{
        document::NbtDocument,
        state::{AppState, FocusArea},
    },
    tui::components::nbt_tree,
};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState, document: &NbtDocument) {
    nbt_tree::render_nbt(
        frame,
        area,
        &document.entries,
        document.selected,
        "NBT Tree",
        state.focus == FocusArea::GenericTree,
    );
}
