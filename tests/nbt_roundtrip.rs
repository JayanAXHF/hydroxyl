use std::collections::HashMap;

use fast_nbt::Value;
use hydroxyl::persistence::nbt_codec::{self, CompressionKind};

#[test]
fn nbt_codec_roundtrips_raw_and_gzip_payloads() {
    let temp = tempfile::tempdir().unwrap();
    let raw_path = temp.path().join("sample.nbt");
    let gzip_path = temp.path().join("sample.dat");

    let mut root = HashMap::new();
    root.insert("Health".to_owned(), Value::Float(18.5));
    root.insert(
        "Pos".to_owned(),
        Value::List(vec![
            Value::Double(1.0),
            Value::Double(64.0),
            Value::Double(-32.0),
        ]),
    );
    let value = Value::Compound(root);

    nbt_codec::write_file(&raw_path, &value, CompressionKind::Raw).unwrap();
    nbt_codec::write_file(&gzip_path, &value, CompressionKind::Gzip).unwrap();

    let raw = nbt_codec::read_file(&raw_path).unwrap();
    let gzip = nbt_codec::read_file(&gzip_path).unwrap();

    assert_eq!(raw.compression, CompressionKind::Raw);
    assert_eq!(gzip.compression, CompressionKind::Gzip);
    assert_eq!(raw.root, value);
    assert_eq!(gzip.root, value);
}
