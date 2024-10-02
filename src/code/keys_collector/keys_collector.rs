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
    pub async fn collect_keys(self) -> IndexSet<HDFactorInstance> {
        todo!()
    }
}
