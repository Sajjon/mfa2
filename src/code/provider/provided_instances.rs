use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvidedInstances {
    hidden_constructor: HiddenConstructor,

    /// The caller of FactorInstancesProvider::provide MUST override their
    /// original cache with this updated one if they want to persist the changes.
    pub cache_to_persist: FactorInstancesForSpecificNetworkCache,

    /// The factor instances that were provided to be used directly, this is sometimes
    /// empty, e.g. in the case of PreDeriveKeys for new FactorSource.
    ///
    /// And often this contains just some of the newly created instances, because
    /// some might have gone into the `cache_to_persist` instead.
    pub instances_to_be_used: IndexSet<HDFactorInstance>,
}
impl ProvidedInstances {
    pub fn new(
        cache: FactorInstancesForSpecificNetworkCache,
        to_use_directly: ToUseDirectly,
        to_cache: ToCache,
    ) -> Self {
        todo!()
    }
    pub fn for_account_veci(
        cache: FactorInstancesForSpecificNetworkCache,
        instance: HDFactorInstance,
        to_cache: Option<ToCache>,
    ) -> Self {
        Self::new(
            cache,
            ToUseDirectly::just(instance),
            to_cache.unwrap_or_default(),
        )
    }
}
