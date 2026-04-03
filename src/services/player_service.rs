use fast_nbt::Value;

use crate::{
    app::tab_id::DocumentId,
    app::{
        action::PlayerField,
        document::{DocumentKind, DocumentMeta, PlayerDocument},
    },
    domain::{
        files::source::DocumentSource,
        minecraft::{player::PlayerData, profile::SkinState, server::ServerContext},
        nbt::{
            edit::{flatten, get_mut},
            path::NbtPath,
            value::NbtValue,
        },
    },
    persistence::{dirty::DirtyState, nbt_codec},
    util::{error::HydroxylError, fs::file_name, result::Result},
};

pub struct PlayerService;

impl PlayerService {
    pub fn open(
        &self,
        id: DocumentId,
        path: &std::path::Path,
        source: DocumentSource,
        server: Option<ServerContext>,
    ) -> Result<PlayerDocument> {
        let file = nbt_codec::read_file(path)?;
        let data = PlayerData::from_root(path, &file.root);
        Ok(PlayerDocument {
            meta: DocumentMeta {
                id,
                kind: DocumentKind::Player,
                path: path.to_path_buf(),
                title: file_name(path),
                source,
                dirty: DirtyState::clean(),
            },
            server,
            compression: file.compression,
            root: file.root,
            data,
            skin_state: SkinState::NotRequested,
        })
    }

    pub fn refresh(&self, document: &mut PlayerDocument) {
        let preserved_skin = document.skin_state.clone();
        let preserved_server = document.server.clone();
        let preserved_section = document.data.selected_section;
        let preserved_field = document.data.field_selected;
        let preserved_inventory = document.data.inventory.selected_index;
        let preserved_raw = document.data.raw_selected;

        document.data = PlayerData::from_root(&document.meta.path, &document.root);
        document.data.selected_section =
            preserved_section.min(crate::domain::minecraft::player::PlayerSection::ALL.len() - 1);
        document.data.field_selected = preserved_field;
        document.data.inventory.selected_index =
            preserved_inventory.min(document.data.inventory.slots.len().saturating_sub(1));
        document.data.raw_selected =
            preserved_raw.min(document.data.raw_entries.len().saturating_sub(1));
        document.data.raw_entries = flatten(&document.root);
        document.server = preserved_server;
        document.skin_state = preserved_skin;
    }

    pub fn edit_field(
        &self,
        document: &mut PlayerDocument,
        field: &PlayerField,
        input: &str,
    ) -> Result<()> {
        match field {
            PlayerField::Health => {
                set_number(&mut document.root, "Health", Value::Float(input.parse()?))?
            }
            PlayerField::FoodLevel => {
                set_number(&mut document.root, "foodLevel", Value::Int(input.parse()?))?
            }
            PlayerField::FoodSaturation => set_number(
                &mut document.root,
                "foodSaturationLevel",
                Value::Float(input.parse()?),
            )?,
            PlayerField::XpLevel => {
                set_number(&mut document.root, "XpLevel", Value::Int(input.parse()?))?
            }
            PlayerField::XpProgress => {
                set_number(&mut document.root, "XpP", Value::Float(input.parse()?))?
            }
            PlayerField::XpTotal => {
                set_number(&mut document.root, "XpTotal", Value::Int(input.parse()?))?
            }
            PlayerField::Air => {
                set_number(&mut document.root, "Air", Value::Short(input.parse()?))?
            }
            PlayerField::PosX => {
                set_list_number(&mut document.root, "Pos", 0, Value::Double(input.parse()?))?
            }
            PlayerField::PosY => {
                set_list_number(&mut document.root, "Pos", 1, Value::Double(input.parse()?))?
            }
            PlayerField::PosZ => {
                set_list_number(&mut document.root, "Pos", 2, Value::Double(input.parse()?))?
            }
            PlayerField::Yaw => set_list_number(
                &mut document.root,
                "Rotation",
                0,
                Value::Float(input.parse()?),
            )?,
            PlayerField::Pitch => set_list_number(
                &mut document.root,
                "Rotation",
                1,
                Value::Float(input.parse()?),
            )?,
            PlayerField::Dimension => set_number(
                &mut document.root,
                "Dimension",
                Value::String(input.to_owned()),
            )?,
            PlayerField::Flying => set_ability(&mut document.root, "flying", parse_bool(input))?,
            PlayerField::MayFly => set_ability(&mut document.root, "mayfly", parse_bool(input))?,
            PlayerField::Instabuild => {
                set_ability(&mut document.root, "instabuild", parse_bool(input))?
            }
            PlayerField::Invulnerable => {
                set_ability(&mut document.root, "invulnerable", parse_bool(input))?
            }
            PlayerField::MayBuild => {
                set_ability(&mut document.root, "mayBuild", parse_bool(input))?
            }
            PlayerField::WalkSpeed => {
                set_ability_f32(&mut document.root, "walkSpeed", input.parse()?)?
            }
            PlayerField::FlySpeed => {
                set_ability_f32(&mut document.root, "flySpeed", input.parse()?)?
            }
            PlayerField::InventoryCount => self.edit_inventory_count(document, input)?,
            PlayerField::InventoryId => self.edit_inventory_id(document, input)?,
            PlayerField::RawSection(_) => {}
        }

        document.meta.dirty.mark_dirty();
        self.refresh(document);
        Ok(())
    }

    fn edit_inventory_count(&self, document: &mut PlayerDocument, input: &str) -> Result<()> {
        let Some(selected) = document.data.inventory.selected() else {
            return Ok(());
        };
        if let Some(slot) = find_inventory_slot_mut(&mut document.root, selected.slot) {
            slot.insert("Count".to_owned(), Value::Byte(input.parse()?));
        }
        Ok(())
    }

    fn edit_inventory_id(&self, document: &mut PlayerDocument, input: &str) -> Result<()> {
        let Some(selected) = document.data.inventory.selected() else {
            return Ok(());
        };
        if let Some(slot) = find_inventory_slot_mut(&mut document.root, selected.slot) {
            slot.insert("id".to_owned(), Value::String(input.to_owned()));
        }
        Ok(())
    }
}

fn set_number(root: &mut NbtValue, key: &str, value: Value) -> Result<()> {
    let path = NbtPath(vec![crate::domain::nbt::path::NbtPathSegment::Key(
        key.to_owned(),
    )]);
    let target = get_mut(root, &path)
        .ok_or_else(|| HydroxylError::invalid_data(format!("missing key {key}")))?;
    *target = value;
    Ok(())
}

fn set_list_number(root: &mut NbtValue, key: &str, index: usize, value: Value) -> Result<()> {
    let path = NbtPath(vec![
        crate::domain::nbt::path::NbtPathSegment::Key(key.to_owned()),
        crate::domain::nbt::path::NbtPathSegment::Index(index),
    ]);
    let target = get_mut(root, &path)
        .ok_or_else(|| HydroxylError::invalid_data(format!("missing list key {key}")))?;
    *target = value;
    Ok(())
}

fn set_ability(root: &mut NbtValue, key: &str, value: bool) -> Result<()> {
    let abilities = find_compound_mut(root, "abilities")?;
    abilities.insert(key.to_owned(), Value::Byte(i8::from(value)));
    Ok(())
}

fn set_ability_f32(root: &mut NbtValue, key: &str, value: f32) -> Result<()> {
    let abilities = find_compound_mut(root, "abilities")?;
    abilities.insert(key.to_owned(), Value::Float(value));
    Ok(())
}

fn find_compound_mut<'a>(
    root: &'a mut NbtValue,
    key: &str,
) -> Result<&'a mut std::collections::HashMap<String, Value>> {
    match root {
        Value::Compound(map) => match map.get_mut(key) {
            Some(Value::Compound(compound)) => Ok(compound),
            _ => Err(HydroxylError::invalid_data(format!(
                "missing compound {key}"
            ))),
        },
        _ => Err(HydroxylError::invalid_data("root is not a compound")),
    }
}

fn find_inventory_slot_mut(
    root: &mut NbtValue,
    slot_id: i32,
) -> Option<&mut std::collections::HashMap<String, Value>> {
    let Value::Compound(map) = root else {
        return None;
    };
    let Value::List(items) = map.get_mut("Inventory")? else {
        return None;
    };
    for item in items {
        if let Value::Compound(compound) = item {
            let current = compound
                .get("Slot")
                .and_then(Value::as_i64)
                .unwrap_or_default() as i32;
            if current == slot_id {
                return Some(compound);
            }
        }
    }
    None
}

fn parse_bool(input: &str) -> bool {
    matches!(
        input.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}
