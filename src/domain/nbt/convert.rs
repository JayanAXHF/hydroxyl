use fast_nbt::Value;

use crate::domain::nbt::value::NbtValue;

pub fn as_compound(value: &NbtValue) -> Option<&std::collections::HashMap<String, Value>> {
    match value {
        Value::Compound(map) => Some(map),
        _ => None,
    }
}

pub fn as_compound_mut(
    value: &mut NbtValue,
) -> Option<&mut std::collections::HashMap<String, Value>> {
    match value {
        Value::Compound(map) => Some(map),
        _ => None,
    }
}
