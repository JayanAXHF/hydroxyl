use crate::domain::minecraft::item::ItemStack;

#[derive(Debug, Clone, Default)]
pub struct InventoryModel {
    pub slots: Vec<ItemStack>,
    pub selected_index: usize,
}

impl InventoryModel {
    pub fn selected(&self) -> Option<&ItemStack> {
        self.slots.get(self.selected_index)
    }

    pub fn selected_mut(&mut self) -> Option<&mut ItemStack> {
        self.slots.get_mut(self.selected_index)
    }
}

pub fn slot_label(slot: i32) -> String {
    match slot {
        0..=8 => format!("Hotbar {slot}"),
        9..=35 => format!("Inventory {slot}"),
        100 => "Boots".to_owned(),
        101 => "Leggings".to_owned(),
        102 => "Chestplate".to_owned(),
        103 => "Helmet".to_owned(),
        -106 => "Offhand".to_owned(),
        _ => format!("Slot {slot}"),
    }
}
