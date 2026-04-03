use crate::app::{state::FocusArea, tab::TabKind};

pub fn next_focus(current: FocusArea, tab: &TabKind) -> FocusArea {
    match tab {
        TabKind::Home(_) => match current {
            FocusArea::HomePlayers => FocusArea::HomeStats,
            FocusArea::HomeStats => FocusArea::HomeAdvancements,
            _ => FocusArea::HomePlayers,
        },
        TabKind::Player(_) => match current {
            FocusArea::PlayerSections => FocusArea::PlayerInventory,
            FocusArea::PlayerInventory => FocusArea::PlayerFields,
            FocusArea::PlayerFields => FocusArea::RawNbt,
            _ => FocusArea::PlayerSections,
        },
        TabKind::Nbt(_) | TabKind::Stats(_) | TabKind::Advancements(_) => FocusArea::GenericTree,
    }
}
