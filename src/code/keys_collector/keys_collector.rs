use crate::prelude::*;

pub struct KeysCollector;
impl KeysCollector {
    pub fn new(
        factors: IndexSet<HDFactorSource>,
        derivation_paths: IndexMap<FactorSourceID, IndexSet<DerivationPath>>,
    ) -> Self {
        todo!()
    }
    #[allow(unused)]
    pub async fn collect_keys(self) -> KeyDerivationOutcome {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyDerivationOutcome {
    instances: IndexSet<HDFactorInstance>,
}
