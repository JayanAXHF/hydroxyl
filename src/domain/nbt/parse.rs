use crate::{domain::nbt::value::NbtValue, util::result::Result};

pub fn from_bytes(bytes: &[u8]) -> Result<NbtValue> {
    Ok(fast_nbt::from_bytes(bytes)?)
}

pub fn to_bytes(value: &NbtValue) -> Result<Vec<u8>> {
    Ok(fast_nbt::to_bytes(value)?)
}
