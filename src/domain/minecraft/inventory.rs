use crate::domain::minecraft::item::ItemStack;

pub const MAIN_SLOT_COUNT: usize = 36;
pub const ARMOR_SLOT_COUNT: usize = 4;
pub const HOTBAR_VISUAL_START: usize = 27;
pub const OFFHAND_SLOT_ID: i32 = -106;

const ARMOR_SLOT_IDS: [i32; ARMOR_SLOT_COUNT] = [103, 102, 101, 100];

#[derive(Debug, Clone)]
pub struct InventoryCell {
    pub slot_id: i32,
    pub item: Option<ItemStack>,
}

impl InventoryCell {
    pub fn empty(slot_id: i32) -> Self {
        Self {
            slot_id,
            item: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.item.is_none()
    }

    pub fn count(&self) -> i32 {
        self.item
            .as_ref()
            .map(|item| item.count)
            .unwrap_or_default()
    }

    pub fn short_label(&self, width: usize) -> String {
        let Some(item) = &self.item else {
            return "Empty".to_owned();
        };

        let leaf = item.item_id.rsplit(':').next().unwrap_or(&item.item_id);
        truncate(leaf, width)
    }

    pub fn quantity_label(&self) -> String {
        if self.is_empty() {
            "-".to_owned()
        } else {
            format!("x{}", self.count())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryRegion {
    Main,
    Armor,
    Offhand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventorySelection {
    pub region: InventoryRegion,
    pub index: usize,
}

impl Default for InventorySelection {
    fn default() -> Self {
        Self {
            region: InventoryRegion::Main,
            index: HOTBAR_VISUAL_START,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InventoryModel {
    pub main: Vec<InventoryCell>,
    pub armor: Vec<InventoryCell>,
    pub offhand: InventoryCell,
    pub selection: InventorySelection,
}

impl Default for InventoryModel {
    fn default() -> Self {
        Self {
            main: (0..MAIN_SLOT_COUNT)
                .map(|index| InventoryCell::empty(main_slot_id_for_index(index)))
                .collect(),
            armor: ARMOR_SLOT_IDS
                .iter()
                .copied()
                .map(InventoryCell::empty)
                .collect(),
            offhand: InventoryCell::empty(OFFHAND_SLOT_ID),
            selection: InventorySelection::default(),
        }
    }
}

impl InventoryModel {
    pub fn place(&mut self, item: ItemStack) {
        if let Some(index) = main_index_for_slot_id(item.slot) {
            self.main[index].item = Some(item);
        } else if let Some(index) = armor_index_for_slot_id(item.slot) {
            self.armor[index].item = Some(item);
        } else if item.slot == OFFHAND_SLOT_ID {
            self.offhand.item = Some(item);
        }
    }

    pub fn selected_cell(&self) -> &InventoryCell {
        match self.selection.region {
            InventoryRegion::Main => &self.main[self.selection.index.min(self.main.len() - 1)],
            InventoryRegion::Armor => &self.armor[self.selection.index.min(self.armor.len() - 1)],
            InventoryRegion::Offhand => &self.offhand,
        }
    }

    pub fn selected_item(&self) -> Option<&ItemStack> {
        self.selected_cell().item.as_ref()
    }

    pub fn selected_slot_id(&self) -> i32 {
        self.selected_cell().slot_id
    }

    pub fn occupied_count(&self) -> usize {
        self.main.iter().filter(|cell| !cell.is_empty()).count()
            + self.armor.iter().filter(|cell| !cell.is_empty()).count()
            + usize::from(!self.offhand.is_empty())
    }

    pub fn move_horizontal(&mut self, delta: isize) {
        if delta > 0 {
            for _ in 0..delta as usize {
                self.step_right();
            }
        } else {
            for _ in 0..delta.unsigned_abs() {
                self.step_left();
            }
        }
    }

    pub fn move_vertical(&mut self, delta: isize) {
        if delta > 0 {
            for _ in 0..delta as usize {
                self.step_down();
            }
        } else {
            for _ in 0..delta.unsigned_abs() {
                self.step_up();
            }
        }
    }

    fn step_left(&mut self) {
        match self.selection.region {
            InventoryRegion::Main => {
                let column = self.selection.index % 9;
                if column > 0 {
                    self.selection.index -= 1;
                }
            }
            InventoryRegion::Armor => {
                let row = self.selection.index.min(3);
                self.selection = InventorySelection {
                    region: InventoryRegion::Main,
                    index: row * 9 + 8,
                };
            }
            InventoryRegion::Offhand => {
                self.selection = InventorySelection {
                    region: InventoryRegion::Main,
                    index: 35,
                };
            }
        }
    }

    fn step_right(&mut self) {
        match self.selection.region {
            InventoryRegion::Main => {
                let column = self.selection.index % 9;
                let row = self.selection.index / 9;
                if column < 8 {
                    self.selection.index += 1;
                } else {
                    self.selection = InventorySelection {
                        region: InventoryRegion::Armor,
                        index: row.min(3),
                    };
                }
            }
            InventoryRegion::Armor | InventoryRegion::Offhand => {}
        }
    }

    fn step_up(&mut self) {
        match self.selection.region {
            InventoryRegion::Main => {
                if self.selection.index >= 9 {
                    self.selection.index -= 9;
                }
            }
            InventoryRegion::Armor => {
                if self.selection.index > 0 {
                    self.selection.index -= 1;
                }
            }
            InventoryRegion::Offhand => {
                self.selection = InventorySelection {
                    region: InventoryRegion::Armor,
                    index: 3,
                };
            }
        }
    }

    fn step_down(&mut self) {
        match self.selection.region {
            InventoryRegion::Main => {
                if self.selection.index + 9 < self.main.len() {
                    self.selection.index += 9;
                }
            }
            InventoryRegion::Armor => {
                if self.selection.index < 3 {
                    self.selection.index += 1;
                } else {
                    self.selection = InventorySelection {
                        region: InventoryRegion::Offhand,
                        index: 0,
                    };
                }
            }
            InventoryRegion::Offhand => {}
        }
    }
}

pub fn main_slot_id_for_index(index: usize) -> i32 {
    match index {
        0..=26 => index as i32 + 9,
        27..=35 => (index - HOTBAR_VISUAL_START) as i32,
        _ => 0,
    }
}

pub fn main_index_for_slot_id(slot_id: i32) -> Option<usize> {
    match slot_id {
        9..=35 => Some((slot_id - 9) as usize),
        0..=8 => Some(HOTBAR_VISUAL_START + slot_id as usize),
        _ => None,
    }
}

pub fn armor_index_for_slot_id(slot_id: i32) -> Option<usize> {
    match slot_id {
        103 => Some(0),
        102 => Some(1),
        101 => Some(2),
        100 => Some(3),
        _ => None,
    }
}

pub fn equipment_slot_title(slot_id: i32) -> &'static str {
    match slot_id {
        103 => "Helmet",
        102 => "Chest",
        101 => "Legs",
        100 => "Boots",
        OFFHAND_SLOT_ID => "Hand",
        _ => "Slot",
    }
}

pub fn slot_label(slot: i32) -> String {
    match slot {
        0..=8 => format!("Hotbar {}", slot + 1),
        9..=35 => format!("Inventory {}", slot - 8),
        100 => "Boots".to_owned(),
        101 => "Leggings".to_owned(),
        102 => "Chestplate".to_owned(),
        103 => "Helmet".to_owned(),
        OFFHAND_SLOT_ID => "Offhand".to_owned(),
        _ => format!("Slot {slot}"),
    }
}

fn truncate(input: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let mut output = String::new();
    for ch in input.chars().take(width) {
        output.push(ch);
    }
    if output.is_empty() {
        input.to_owned()
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::minecraft::inventory::{
        HOTBAR_VISUAL_START, InventoryModel, InventoryRegion, armor_index_for_slot_id,
        main_index_for_slot_id, main_slot_id_for_index,
    };

    #[test]
    fn main_slot_mapping_matches_visual_layout() {
        assert_eq!(main_slot_id_for_index(0), 9);
        assert_eq!(main_slot_id_for_index(26), 35);
        assert_eq!(main_slot_id_for_index(HOTBAR_VISUAL_START), 0);
        assert_eq!(main_slot_id_for_index(35), 8);

        assert_eq!(main_index_for_slot_id(9), Some(0));
        assert_eq!(main_index_for_slot_id(35), Some(26));
        assert_eq!(main_index_for_slot_id(0), Some(HOTBAR_VISUAL_START));
        assert_eq!(main_index_for_slot_id(8), Some(35));
        assert_eq!(armor_index_for_slot_id(103), Some(0));
    }

    #[test]
    fn selection_walks_between_main_armor_and_offhand() {
        let mut inventory = InventoryModel::default();
        inventory.selection.index = 8;
        inventory.move_horizontal(1);
        assert_eq!(inventory.selection.region, InventoryRegion::Armor);
        assert_eq!(inventory.selection.index, 0);

        inventory.move_vertical(3);
        assert_eq!(inventory.selection.region, InventoryRegion::Armor);
        assert_eq!(inventory.selection.index, 3);

        inventory.move_vertical(1);
        assert_eq!(inventory.selection.region, InventoryRegion::Offhand);

        inventory.move_horizontal(-1);
        assert_eq!(inventory.selection.region, InventoryRegion::Main);
        assert_eq!(inventory.selection.index, 35);
    }
}
