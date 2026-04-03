use crate::domain::nbt::value::NbtValue;

pub fn is_editable_scalar(value: &NbtValue) -> bool {
    !matches!(value, NbtValue::Compound(_) | NbtValue::List(_))
}
