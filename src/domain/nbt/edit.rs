use std::collections::HashMap;

use fast_nbt::{ByteArray, IntArray, LongArray, Value};

use crate::{
    domain::nbt::{
        path::{NbtPath, NbtPathSegment},
        value::{NbtValue, type_name},
    },
    util::{error::HydroxylError, result::Result},
};

#[derive(Debug, Clone)]
pub struct NbtEntry {
    pub path: NbtPath,
    pub depth: usize,
    pub label: String,
    pub type_name: &'static str,
    pub preview: String,
    pub editable: bool,
}

pub fn flatten(root: &NbtValue) -> Vec<NbtEntry> {
    let mut entries = Vec::new();
    flatten_inner(root, &NbtPath::default(), 0, "root", &mut entries);
    entries
}

fn flatten_inner(
    value: &NbtValue,
    path: &NbtPath,
    depth: usize,
    label: &str,
    output: &mut Vec<NbtEntry>,
) {
    output.push(NbtEntry {
        path: path.clone(),
        depth,
        label: label.to_owned(),
        type_name: type_name(value),
        preview: preview(value),
        editable: is_scalar(value),
    });

    match value {
        Value::Compound(map) => {
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by(|left, right| left.0.cmp(right.0));
            for (key, child) in entries {
                flatten_inner(child, &path.child_key(key), depth + 1, key, output);
            }
        }
        Value::List(values) => {
            for (index, child) in values.iter().enumerate() {
                flatten_inner(
                    child,
                    &path.child_index(index),
                    depth + 1,
                    &index.to_string(),
                    output,
                );
            }
        }
        _ => {}
    }
}

pub fn preview(value: &NbtValue) -> String {
    match value {
        Value::Byte(v) => v.to_string(),
        Value::Short(v) => v.to_string(),
        Value::Int(v) => v.to_string(),
        Value::Long(v) => v.to_string(),
        Value::Float(v) => format!("{v:.3}"),
        Value::Double(v) => format!("{v:.3}"),
        Value::String(v) => v.clone(),
        Value::ByteArray(values) => format!("{} bytes", values.len()),
        Value::IntArray(values) => format!("{} ints", values.len()),
        Value::LongArray(values) => format!("{} longs", values.len()),
        Value::List(values) => format!("{} entries", values.len()),
        Value::Compound(map) => format!("{} keys", map.len()),
    }
}

pub fn get<'a>(root: &'a NbtValue, path: &NbtPath) -> Option<&'a NbtValue> {
    let mut current = root;
    for segment in &path.0 {
        current = match (current, segment) {
            (Value::Compound(map), NbtPathSegment::Key(key)) => map.get(key)?,
            (Value::List(values), NbtPathSegment::Index(index)) => values.get(*index)?,
            _ => return None,
        };
    }

    Some(current)
}

pub fn get_mut<'a>(root: &'a mut NbtValue, path: &NbtPath) -> Option<&'a mut NbtValue> {
    let mut current = root;
    for segment in &path.0 {
        current = match (current, segment) {
            (Value::Compound(map), NbtPathSegment::Key(key)) => map.get_mut(key)?,
            (Value::List(values), NbtPathSegment::Index(index)) => values.get_mut(*index)?,
            _ => return None,
        };
    }

    Some(current)
}

pub fn is_scalar(value: &NbtValue) -> bool {
    !matches!(value, Value::Compound(_) | Value::List(_))
}

pub fn set_scalar_from_string(root: &mut NbtValue, path: &NbtPath, input: &str) -> Result<()> {
    let current =
        get_mut(root, path).ok_or_else(|| HydroxylError::invalid_data("missing NBT path"))?;
    *current = parse_scalar_like(current, input)?;
    Ok(())
}

fn parse_scalar_like(current: &NbtValue, input: &str) -> Result<NbtValue> {
    Ok(match current {
        Value::Byte(_) => Value::Byte(input.parse()?),
        Value::Short(_) => Value::Short(input.parse()?),
        Value::Int(_) => Value::Int(input.parse()?),
        Value::Long(_) => Value::Long(input.parse()?),
        Value::Float(_) => Value::Float(input.parse()?),
        Value::Double(_) => Value::Double(input.parse()?),
        Value::String(_) => Value::String(input.to_owned()),
        Value::ByteArray(_) => Value::ByteArray(ByteArray::new(parse_number_list::<i8>(input)?)),
        Value::IntArray(_) => Value::IntArray(IntArray::new(parse_number_list::<i32>(input)?)),
        Value::LongArray(_) => Value::LongArray(LongArray::new(parse_number_list::<i64>(input)?)),
        Value::List(_) | Value::Compound(_) => {
            return Err(HydroxylError::invalid_data(
                "editing compound and list values is not supported in inline mode",
            ));
        }
    })
}

fn parse_number_list<T>(input: &str) -> Result<Vec<T>>
where
    T: std::str::FromStr,
    HydroxylError: From<<T as std::str::FromStr>::Err>,
{
    if input.trim().is_empty() {
        return Ok(Vec::new());
    }

    input
        .split(',')
        .map(|value| value.trim().parse().map_err(Into::into))
        .collect()
}

pub fn compound() -> HashMap<String, Value> {
    HashMap::new()
}
