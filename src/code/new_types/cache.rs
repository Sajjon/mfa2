use crate::prelude::*;

/// On one specific network
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorInstancesForSpecificNetworkCache {
    network_id: NetworkID,
    per_factor_source: IndexMap<FactorSourceID, CollectionsOfFactorInstances>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorInstanceFromCache {
    hidden_constructor: HiddenConstructor,
    pub instance: HDFactorInstance,
    /// if this was the last instance in the collection of instances, if it is,
    /// we SHOULD derive more!
    pub was_last_used: bool,
}

impl FactorInstancesForSpecificNetworkCache {
    pub fn empty(network: NetworkID) -> Self {
        Self {
            network_id: network,
            per_factor_source: IndexMap::new(),
        }
    }

    /// Does NOT mutate self
    pub fn peek_account_veci(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Option<FactorInstanceFromCache> {
        todo!()
    }

    /// Does NOT mutate self
    pub fn peek_all_instances_for_factor_source(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Option<CollectionsOfFactorInstances> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorInstancesForEachNetworkCache {
    #[allow(dead_code)]
    hidden_constructor: HiddenConstructor,
    pub networks: HashMap<NetworkID, FactorInstancesForSpecificNetworkCache>,
}
impl FactorInstancesForEachNetworkCache {
    /// Reads out the existing `FactorInstancesForSpecificNetworkCache` if any,
    /// otherwise creates a new empty one (mutates self with interior mutability).
    pub fn clone_for_network(self, network: NetworkID) -> FactorInstancesForSpecificNetworkCache {
        self.networks
            .get(&network)
            .cloned()
            .unwrap_or(FactorInstancesForSpecificNetworkCache::empty(network))
    }
}
