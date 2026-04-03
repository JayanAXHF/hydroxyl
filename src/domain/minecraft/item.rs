use crate::domain::nbt::value::NbtValue;

#[derive(Debug, Clone, Default)]
pub struct ItemStack {
    pub slot: i32,
    pub item_id: String,
    pub count: i32,
    pub damage: Option<i32>,
    pub tag: Option<NbtValue>,
}

impl ItemStack {
    pub fn title(&self) -> &str {
        if self.item_id.is_empty() {
            "Empty"
        } else {
            &self.item_id
        }
    }
}
