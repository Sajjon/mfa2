use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeySpace {
    Unsecurified,
    Securified,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CAP26EntityIndex {
    Securified(u32),
    Unsecurified(u32),
}
impl CAP26EntityIndex {
    pub fn next(&self) -> Self {
        todo!()
    }
    pub fn key_space(&self) -> KeySpace {
        match self {
            CAP26EntityIndex::Securified(_) => KeySpace::Securified,
            CAP26EntityIndex::Unsecurified(_) => KeySpace::Unsecurified,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DerivationPath {
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    pub entity_index: CAP26EntityIndex,
}

impl DerivationPath {
    pub fn key_space(&self) -> KeySpace {
        self.entity_index.key_space()
    }
}
