use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NbtPathSegment {
    Key(String),
    Index(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NbtPath(pub Vec<NbtPathSegment>);

impl NbtPath {
    pub fn child_key(&self, key: impl Into<String>) -> Self {
        let mut next = self.0.clone();
        next.push(NbtPathSegment::Key(key.into()));
        Self(next)
    }

    pub fn child_index(&self, index: usize) -> Self {
        let mut next = self.0.clone();
        next.push(NbtPathSegment::Index(index));
        Self(next)
    }
}

impl Display for NbtPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return f.write_str("/");
        }

        for segment in &self.0 {
            match segment {
                NbtPathSegment::Key(key) => {
                    write!(f, "/{key}")?;
                }
                NbtPathSegment::Index(index) => {
                    write!(f, "[{index}]")?;
                }
            }
        }

        Ok(())
    }
}
