use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    app::{
        action::{EditTarget, PlayerField},
        document::PlayerDocument,
        state::{AppState, FocusArea},
    },
    domain::minecraft::player::PlayerSection,
    tui::{
        components::{
            avatar_panel, inventory_grid, item_details, nbt_tree, position_panel,
            property_list::{self, PropertyRow},
            section_list, stats_panel,
        },
        frame::titled_block,
    },
};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState, document: &PlayerDocument) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(22),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let sections = PlayerSection::ALL
        .iter()
        .map(|section| section.title().to_owned())
        .collect::<Vec<_>>();
    section_list::render(
        frame,
        columns[0],
        "Sections",
        &sections,
        document.data.selected_section,
        state.focus == FocusArea::PlayerSections,
    );

    match document.data.active_section() {
        PlayerSection::Inventory => {
            render_inventory_view(frame, columns[1], columns[2], state, document)
        }
        PlayerSection::RawNbt => render_raw_view(frame, columns[1], columns[2], state, document),
        PlayerSection::Overview
        | PlayerSection::Attributes
        | PlayerSection::Position
        | PlayerSection::Abilities => {
            render_structured_view(frame, columns[1], columns[2], state, document)
        }
    }
}

fn render_inventory_view(
    frame: &mut Frame,
    left: Rect,
    right: Rect,
    state: &AppState,
    document: &PlayerDocument,
) {
    inventory_grid::render(
        frame,
        left,
        &document.data.inventory.slots,
        document.data.inventory.selected_index,
        state.focus == FocusArea::PlayerInventory,
    );

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(15),
            Constraint::Length(10),
            Constraint::Min(8),
        ])
        .split(right);

    avatar_panel::render(
        frame,
        right_chunks[0],
        &document.skin_state,
        document.data.identity.name.as_deref(),
        document.data.identity.uuid.map(|uuid| uuid.to_string()),
    );
    item_details::render(frame, right_chunks[1], document.data.inventory.selected());
    stats_panel::render(frame, right_chunks[2], &document.data.attributes);
}

fn render_structured_view(
    frame: &mut Frame,
    left: Rect,
    right: Rect,
    state: &AppState,
    document: &PlayerDocument,
) {
    let rows = property_rows(document);
    property_list::render(
        frame,
        left,
        document.data.active_section().title(),
        &rows,
        document.data.field_selected,
        state.focus == FocusArea::PlayerFields,
    );

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(15),
            Constraint::Length(6),
            Constraint::Min(8),
        ])
        .split(right);
    avatar_panel::render(
        frame,
        right_chunks[0],
        &document.skin_state,
        document.data.identity.name.as_deref(),
        document.data.identity.uuid.map(|uuid| uuid.to_string()),
    );
    stats_panel::render(frame, right_chunks[1], &document.data.attributes);
    position_panel::render(frame, right_chunks[2], &document.data.position);
}

fn render_raw_view(
    frame: &mut Frame,
    left: Rect,
    right: Rect,
    state: &AppState,
    document: &PlayerDocument,
) {
    nbt_tree::render_nbt(
        frame,
        left,
        &document.data.raw_entries,
        document.data.raw_selected,
        "Raw NBT",
        state.focus == FocusArea::RawNbt,
    );
    frame.render_widget(
        Paragraph::new("Raw NBT fallback. Select a scalar node and press e to edit inline.")
            .block(titled_block("Notes")),
        right,
    );
}

pub fn property_rows(document: &PlayerDocument) -> Vec<PropertyRow> {
    match document.data.active_section() {
        PlayerSection::Overview => vec![
            PropertyRow {
                label: "UUID".to_owned(),
                value: document
                    .data
                    .identity
                    .uuid
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "n/a".to_owned()),
            },
            PropertyRow {
                label: "Skin".to_owned(),
                value: skin_status(&document.skin_state),
            },
            PropertyRow {
                label: "Item Count".to_owned(),
                value: document.data.inventory.slots.len().to_string(),
            },
        ],
        PlayerSection::Attributes => vec![
            PropertyRow {
                label: "Health".to_owned(),
                value: format!("{:.1}", document.data.attributes.health),
            },
            PropertyRow {
                label: "Food Level".to_owned(),
                value: document.data.attributes.food_level.to_string(),
            },
            PropertyRow {
                label: "Saturation".to_owned(),
                value: format!("{:.1}", document.data.attributes.food_saturation),
            },
            PropertyRow {
                label: "XP Level".to_owned(),
                value: document.data.attributes.xp_level.to_string(),
            },
            PropertyRow {
                label: "XP Progress".to_owned(),
                value: format!("{:.3}", document.data.attributes.xp_progress),
            },
            PropertyRow {
                label: "XP Total".to_owned(),
                value: document.data.attributes.xp_total.to_string(),
            },
            PropertyRow {
                label: "Air".to_owned(),
                value: document.data.attributes.air.to_string(),
            },
        ],
        PlayerSection::Position => vec![
            PropertyRow {
                label: "X".to_owned(),
                value: format!("{:.3}", document.data.position.x),
            },
            PropertyRow {
                label: "Y".to_owned(),
                value: format!("{:.3}", document.data.position.y),
            },
            PropertyRow {
                label: "Z".to_owned(),
                value: format!("{:.3}", document.data.position.z),
            },
            PropertyRow {
                label: "Yaw".to_owned(),
                value: format!("{:.3}", document.data.position.yaw),
            },
            PropertyRow {
                label: "Pitch".to_owned(),
                value: format!("{:.3}", document.data.position.pitch),
            },
            PropertyRow {
                label: "Dimension".to_owned(),
                value: document.data.position.dimension.clone(),
            },
        ],
        PlayerSection::Abilities => vec![
            PropertyRow {
                label: "Flying".to_owned(),
                value: document.data.abilities.flying.to_string(),
            },
            PropertyRow {
                label: "May Fly".to_owned(),
                value: document.data.abilities.may_fly.to_string(),
            },
            PropertyRow {
                label: "Instabuild".to_owned(),
                value: document.data.abilities.instabuild.to_string(),
            },
            PropertyRow {
                label: "Invulnerable".to_owned(),
                value: document.data.abilities.invulnerable.to_string(),
            },
            PropertyRow {
                label: "May Build".to_owned(),
                value: document.data.abilities.may_build.to_string(),
            },
            PropertyRow {
                label: "Walk Speed".to_owned(),
                value: format!("{:.3}", document.data.abilities.walk_speed),
            },
            PropertyRow {
                label: "Fly Speed".to_owned(),
                value: format!("{:.3}", document.data.abilities.fly_speed),
            },
        ],
        PlayerSection::Inventory | PlayerSection::RawNbt => Vec::new(),
    }
}

pub fn edit_target(
    document: &PlayerDocument,
    focus: FocusArea,
) -> Option<(String, String, EditTarget)> {
    match (document.data.active_section(), focus) {
        (PlayerSection::Inventory, FocusArea::PlayerInventory) => {
            let selected = document.data.inventory.selected()?;
            Some((
                format!("Edit count for {}", selected.item_id),
                selected.count.to_string(),
                EditTarget::PlayerField {
                    document_id: document.meta.id,
                    field: PlayerField::InventoryCount,
                },
            ))
        }
        (PlayerSection::RawNbt, FocusArea::RawNbt) => {
            let entry = document.data.raw_entries.get(document.data.raw_selected)?;
            if !entry.editable {
                return None;
            }
            Some((
                format!("Edit {}", entry.path),
                entry.preview.clone(),
                EditTarget::NbtValue {
                    document_id: document.meta.id,
                    path: entry.path.clone(),
                },
            ))
        }
        (_, FocusArea::PlayerFields) => {
            let (title, value, field) = selected_field(document)?;
            Some((
                format!("Edit {title}"),
                value,
                EditTarget::PlayerField {
                    document_id: document.meta.id,
                    field,
                },
            ))
        }
        _ => None,
    }
}

fn selected_field(document: &PlayerDocument) -> Option<(String, String, PlayerField)> {
    let index = document.data.field_selected;
    let result = match document.data.active_section() {
        PlayerSection::Attributes => match index {
            0 => (
                "Health",
                document.data.attributes.health.to_string(),
                PlayerField::Health,
            ),
            1 => (
                "Food Level",
                document.data.attributes.food_level.to_string(),
                PlayerField::FoodLevel,
            ),
            2 => (
                "Saturation",
                document.data.attributes.food_saturation.to_string(),
                PlayerField::FoodSaturation,
            ),
            3 => (
                "XP Level",
                document.data.attributes.xp_level.to_string(),
                PlayerField::XpLevel,
            ),
            4 => (
                "XP Progress",
                document.data.attributes.xp_progress.to_string(),
                PlayerField::XpProgress,
            ),
            5 => (
                "XP Total",
                document.data.attributes.xp_total.to_string(),
                PlayerField::XpTotal,
            ),
            6 => (
                "Air",
                document.data.attributes.air.to_string(),
                PlayerField::Air,
            ),
            _ => return None,
        },
        PlayerSection::Position => match index {
            0 => ("X", document.data.position.x.to_string(), PlayerField::PosX),
            1 => ("Y", document.data.position.y.to_string(), PlayerField::PosY),
            2 => ("Z", document.data.position.z.to_string(), PlayerField::PosZ),
            3 => (
                "Yaw",
                document.data.position.yaw.to_string(),
                PlayerField::Yaw,
            ),
            4 => (
                "Pitch",
                document.data.position.pitch.to_string(),
                PlayerField::Pitch,
            ),
            5 => (
                "Dimension",
                document.data.position.dimension.clone(),
                PlayerField::Dimension,
            ),
            _ => return None,
        },
        PlayerSection::Abilities => match index {
            0 => (
                "Flying",
                document.data.abilities.flying.to_string(),
                PlayerField::Flying,
            ),
            1 => (
                "May Fly",
                document.data.abilities.may_fly.to_string(),
                PlayerField::MayFly,
            ),
            2 => (
                "Instabuild",
                document.data.abilities.instabuild.to_string(),
                PlayerField::Instabuild,
            ),
            3 => (
                "Invulnerable",
                document.data.abilities.invulnerable.to_string(),
                PlayerField::Invulnerable,
            ),
            4 => (
                "May Build",
                document.data.abilities.may_build.to_string(),
                PlayerField::MayBuild,
            ),
            5 => (
                "Walk Speed",
                document.data.abilities.walk_speed.to_string(),
                PlayerField::WalkSpeed,
            ),
            6 => (
                "Fly Speed",
                document.data.abilities.fly_speed.to_string(),
                PlayerField::FlySpeed,
            ),
            _ => return None,
        },
        _ => return None,
    };

    Some((result.0.to_owned(), result.1, result.2))
}

fn skin_status(state: &crate::domain::minecraft::profile::SkinState) -> String {
    match state {
        crate::domain::minecraft::profile::SkinState::NotRequested => "idle".to_owned(),
        crate::domain::minecraft::profile::SkinState::Loading => "loading".to_owned(),
        crate::domain::minecraft::profile::SkinState::Ready(_) => "ready".to_owned(),
        crate::domain::minecraft::profile::SkinState::Unavailable(message) => {
            format!("unavailable: {message}")
        }
    }
}
