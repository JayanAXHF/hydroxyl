pub type NbtValue = fast_nbt::Value;

pub fn type_name(value: &NbtValue) -> &'static str {
    match value {
        NbtValue::Byte(_) => "Byte",
        NbtValue::Short(_) => "Short",
        NbtValue::Int(_) => "Int",
        NbtValue::Long(_) => "Long",
        NbtValue::Float(_) => "Float",
        NbtValue::Double(_) => "Double",
        NbtValue::String(_) => "String",
        NbtValue::ByteArray(_) => "ByteArray",
        NbtValue::IntArray(_) => "IntArray",
        NbtValue::LongArray(_) => "LongArray",
        NbtValue::List(_) => "List",
        NbtValue::Compound(_) => "Compound",
    }
}
