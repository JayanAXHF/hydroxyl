use fast_nbt::Value;

use crate::{
    domain::{
        files::naming::parse_uuid_from_path,
        minecraft::{
            abilities::PlayerAbilities, attributes::PlayerAttributes, inventory::InventoryModel,
            item::ItemStack, position::PlayerPosition, profile::PlayerIdentity,
        },
        nbt::{edit::flatten, value::NbtValue},
    },
    util::fs::file_name,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSection {
    Overview,
    Inventory,
    Attributes,
    Position,
    Abilities,
    RawNbt,
}

impl PlayerSection {
    pub const ALL: [Self; 6] = [
        Self::Overview,
        Self::Inventory,
        Self::Attributes,
        Self::Position,
        Self::Abilities,
        Self::RawNbt,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Inventory => "Inventory",
            Self::Attributes => "Attributes",
            Self::Position => "Position",
            Self::Abilities => "Abilities",
            Self::RawNbt => "Raw NBT",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerData {
    pub title: String,
    pub identity: PlayerIdentity,
    pub inventory: InventoryModel,
    pub attributes: PlayerAttributes,
    pub position: PlayerPosition,
    pub abilities: PlayerAbilities,
    pub selected_section: usize,
    pub field_selected: usize,
    pub raw_entries: Vec<crate::domain::nbt::edit::NbtEntry>,
    pub raw_selected: usize,
}

impl PlayerData {
    pub fn from_root(path: &std::path::Path, root: &NbtValue) -> Self {
        let uuid = parse_uuid_from_path(path);
        let title = file_name(path);

        Self {
            title,
            identity: PlayerIdentity {
                uuid,
                ..Default::default()
            },
            inventory: parse_inventory(root),
            attributes: parse_attributes(root),
            position: parse_position(root),
            abilities: parse_abilities(root),
            selected_section: 0,
            field_selected: 0,
            raw_entries: flatten(root),
            raw_selected: 0,
        }
    }

    pub fn active_section(&self) -> PlayerSection {
        PlayerSection::ALL[self.selected_section.min(PlayerSection::ALL.len() - 1)]
    }
}

fn parse_inventory(root: &NbtValue) -> InventoryModel {
    let mut model = InventoryModel::default();

    if let Some(items) = compound_get(root, "Inventory").and_then(as_list) {
        for item in items.iter().filter_map(parse_item_stack) {
            model.place(item);
        }
    }

    model
}

fn parse_item_stack(value: &NbtValue) -> Option<ItemStack> {
    let slot = compound_get(value, "Slot")?.as_i64()? as i32;
    let item_id = compound_get(value, "id")?.as_str()?.to_owned();
    let count = compound_get(value, "Count")
        .and_then(Value::as_i64)
        .unwrap_or(1) as i32;
    let damage = compound_get(value, "Damage")
        .and_then(Value::as_i64)
        .map(|value| value as i32);
    let tag = compound_get(value, "tag").cloned();

    Some(ItemStack {
        slot,
        item_id,
        count,
        damage,
        tag,
    })
}

fn parse_attributes(root: &NbtValue) -> PlayerAttributes {
    PlayerAttributes {
        health: compound_get(root, "Health")
            .and_then(Value::as_f64)
            .unwrap_or(20.0) as f32,
        food_level: compound_get(root, "foodLevel")
            .and_then(Value::as_i64)
            .unwrap_or(20) as i32,
        food_saturation: compound_get(root, "foodSaturationLevel")
            .and_then(Value::as_f64)
            .unwrap_or(5.0) as f32,
        xp_level: compound_get(root, "XpLevel")
            .and_then(Value::as_i64)
            .unwrap_or(0) as i32,
        xp_progress: compound_get(root, "XpP")
            .and_then(Value::as_f64)
            .unwrap_or(0.0) as f32,
        xp_total: compound_get(root, "XpTotal")
            .and_then(Value::as_i64)
            .unwrap_or(0) as i32,
        air: compound_get(root, "Air")
            .and_then(Value::as_i64)
            .unwrap_or(300) as i32,
    }
}

fn parse_position(root: &NbtValue) -> PlayerPosition {
    let mut position = PlayerPosition::default();
    if let Some(values) = compound_get(root, "Pos").and_then(as_list) {
        position.x = values.first().and_then(Value::as_f64).unwrap_or(position.x);
        position.y = values.get(1).and_then(Value::as_f64).unwrap_or(position.y);
        position.z = values.get(2).and_then(Value::as_f64).unwrap_or(position.z);
    }
    if let Some(values) = compound_get(root, "Rotation").and_then(as_list) {
        position.yaw = values
            .first()
            .and_then(Value::as_f64)
            .unwrap_or(position.yaw as f64) as f32;
        position.pitch = values
            .get(1)
            .and_then(Value::as_f64)
            .unwrap_or(position.pitch as f64) as f32;
    }
    if let Some(value) = compound_get(root, "Dimension") {
        if let Some(text) = value.as_str() {
            position.dimension = text.to_owned();
        } else if let Some(id) = value.as_i64() {
            position.dimension = id.to_string();
        }
    }
    position
}

fn parse_abilities(root: &NbtValue) -> PlayerAbilities {
    let mut abilities = PlayerAbilities::default();
    if let Some(compound) = compound_get(root, "abilities").and_then(as_compound) {
        abilities.flying = compound.get("flying").and_then(Value::as_i64).unwrap_or(0) != 0;
        abilities.may_fly = compound.get("mayfly").and_then(Value::as_i64).unwrap_or(0) != 0;
        abilities.instabuild = compound
            .get("instabuild")
            .and_then(Value::as_i64)
            .unwrap_or(0)
            != 0;
        abilities.invulnerable = compound
            .get("invulnerable")
            .and_then(Value::as_i64)
            .unwrap_or(0)
            != 0;
        abilities.may_build = compound
            .get("mayBuild")
            .and_then(Value::as_i64)
            .unwrap_or(1)
            != 0;
        abilities.walk_speed = compound
            .get("walkSpeed")
            .and_then(Value::as_f64)
            .unwrap_or(0.1) as f32;
        abilities.fly_speed = compound
            .get("flySpeed")
            .and_then(Value::as_f64)
            .unwrap_or(0.05) as f32;
    }
    abilities
}

fn compound_get<'a>(value: &'a NbtValue, key: &str) -> Option<&'a NbtValue> {
    as_compound(value)?.get(key)
}

fn as_compound(value: &NbtValue) -> Option<&std::collections::HashMap<String, Value>> {
    match value {
        Value::Compound(map) => Some(map),
        _ => None,
    }
}

fn as_list(value: &NbtValue) -> Option<&Vec<Value>> {
    match value {
        Value::List(values) => Some(values),
        _ => None,
    }
}
