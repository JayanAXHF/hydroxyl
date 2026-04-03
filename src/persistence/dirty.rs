#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DirtyState {
    dirty: bool,
}

impl DirtyState {
    pub fn clean() -> Self {
        Self { dirty: false }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(self) -> bool {
        self.dirty
    }
}
