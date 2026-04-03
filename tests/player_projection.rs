use std::collections::HashMap;

use fast_nbt::Value;
use hydroxyl::{
    app::tab_id::DocumentId,
    domain::files::source::DocumentSource,
    persistence::nbt_codec::{self, CompressionKind},
    services::player_service::PlayerService,
};

#[test]
fn player_service_projects_inventory_and_core_fields() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("123e4567-e89b-12d3-a456-426614174000.dat");

    let mut root = HashMap::new();
    root.insert("Health".to_owned(), Value::Float(17.0));
    root.insert("foodLevel".to_owned(), Value::Int(14));
    root.insert("foodSaturationLevel".to_owned(), Value::Float(2.0));
    root.insert("XpLevel".to_owned(), Value::Int(12));
    root.insert("XpP".to_owned(), Value::Float(0.5));
    root.insert("XpTotal".to_owned(), Value::Int(350));
    root.insert("Air".to_owned(), Value::Short(280));
    root.insert(
        "Pos".to_owned(),
        Value::List(vec![
            Value::Double(3.5),
            Value::Double(70.0),
            Value::Double(-8.0),
        ]),
    );
    root.insert(
        "Rotation".to_owned(),
        Value::List(vec![Value::Float(90.0), Value::Float(15.0)]),
    );
    root.insert(
        "Dimension".to_owned(),
        Value::String("minecraft:the_nether".to_owned()),
    );
    root.insert(
        "abilities".to_owned(),
        Value::Compound(HashMap::from([
            ("flying".to_owned(), Value::Byte(1)),
            ("mayfly".to_owned(), Value::Byte(1)),
            ("instabuild".to_owned(), Value::Byte(0)),
            ("invulnerable".to_owned(), Value::Byte(0)),
            ("mayBuild".to_owned(), Value::Byte(1)),
            ("walkSpeed".to_owned(), Value::Float(0.15)),
            ("flySpeed".to_owned(), Value::Float(0.08)),
        ])),
    );
    root.insert(
        "Inventory".to_owned(),
        Value::List(vec![Value::Compound(HashMap::from([
            ("Slot".to_owned(), Value::Byte(0)),
            ("id".to_owned(), Value::String("minecraft:stone".to_owned())),
            ("Count".to_owned(), Value::Byte(32)),
        ]))]),
    );

    nbt_codec::write_file(&path, &Value::Compound(root), CompressionKind::Gzip).unwrap();

    let document = PlayerService
        .open(DocumentId(0), &path, DocumentSource::Direct, None)
        .unwrap();

    assert_eq!(document.data.attributes.health, 17.0);
    assert_eq!(document.data.attributes.food_level, 14);
    assert_eq!(document.data.position.dimension, "minecraft:the_nether");
    assert!(document.data.abilities.flying);
    assert_eq!(document.data.inventory.slots.len(), 1);
    assert_eq!(document.data.inventory.slots[0].item_id, "minecraft:stone");
    assert_eq!(
        document.data.identity.uuid.unwrap().to_string(),
        "123e4567-e89b-12d3-a456-426614174000"
    );
}
