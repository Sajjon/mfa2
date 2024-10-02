use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;

pub struct FactorInstancesProvider {
    /// A Clone of a cache, the caller MUST commit the changes to the
    /// original cache if they want to persist them.
    #[allow(dead_code)]
    cache: FactorInstancesForSpecificNetworkCache,

    query: InstancesQuery,

    next_entity_index_assigner: NextDerivationEntityIndexAssigner,
}

impl FactorInstancesProvider {
    fn for_specific_network(
        cache: FactorInstancesForSpecificNetworkCache,
        query: InstancesQuery,
        next_entity_index_assigner: NextDerivationEntityIndexAssigner,
    ) -> Self {
        Self {
            cache,
            query,
            next_entity_index_assigner,
        }
    }
}

impl FactorInstancesProvider {
    /// `Profile` is optional since None in case of Onboarding Account Recovery Scan
    /// No need to pass Profile as mut, since we just need to read it for the
    /// next derivation entity indices.
    pub fn new(
        cache: Rc<RefCell<FactorInstancesForEachNetworkCache>>,
        network_id: NetworkID,
        profile: impl Into<Option<Profile>>,
        query: InstancesQuery,
    ) -> Self {
        // let cache = cache.clone_for_network(network_id);
        // Self::for_specific_network(
        //     cache,
        //     query,
        //     NextDerivationEntityIndexAssigner::new(network_id, profile),
        // )
        todo!()
    }

    pub async fn provide(self) -> Result<ProvidedInstances> {
        match self.query.clone() {
            InstancesQuery::AccountMfa {
                number_of_instances_per_factor_source,
                factor_sources,
            } => {
                self.provide_accounts_mfa(number_of_instances_per_factor_source, factor_sources)
                    .await
            }
            InstancesQuery::AccountVeci { factor_source } => {
                self.provide_account_veci(factor_source).await
            }
        }
    }
}

impl FactorInstancesProvider {
    fn paths_single_factor(
        &self,
        factor_source_id: FactorSourceID,
        known_indices_for_templates: IndexMap<DerivationTemplate, CAP26EntityIndex>,
        fill_cache: FillCacheQuantitiesForFactor,
    ) -> DerivationPathPerFactorSource {
        todo!()
    }

    async fn derive(&self, paths: DerivationPathPerFactorSource) -> Result<KeyDerivationOutcome> {
        todo!()
    }
    fn split(
        &self,
        from_cache: Option<HDFactorInstance>,
        derived: KeyDerivationOutcome,
    ) -> (ToUseDirectly, ToCache) {
        todo!()
    }
}
impl FactorInstancesProvider {
    async fn provide_account_veci(
        self,
        factor_source: HDFactorSource,
    ) -> Result<ProvidedInstances> {
        let factor_source_id = factor_source.factor_source_id;

        let maybe_cached = self.cache.consume_account_veci(factor_source_id);
        let mut maybe_next_index_for_derivation: Option<CAP26EntityIndex> = None;
        let mut veci: Option<HDFactorInstance> = None;
        let mut to_cache: Option<ToCache> = None;
        if let Some(cached) = maybe_cached {
            veci = Some(cached.instance.clone());
            if cached.was_last_used {
                // TODO: Must we check if `next` is in fact free??? Check against Profile that is...
                // lets try skipping it for now
                maybe_next_index_for_derivation =
                    Some(cached.instance.derivation_path.entity_index.next());
            }
        } else {
            maybe_next_index_for_derivation = Some(
                self.next_entity_index_assigner
                    .next_account_veci(factor_source_id),
            )
        }
        assert!(
            !(veci.is_none() && maybe_next_index_for_derivation.is_none()),
            "Programmer error, both 'veci' and 'maybe_next_index_for_derivation' cannot be none."
        );
        if let Some(next_index_for_derivation) = maybe_next_index_for_derivation {
            // furthermore, since we are deriving ANYWAY, we should also derive to Fill The Cache....
            let fill_cache_maybe_over_estimated =
                FillCacheQuantitiesForFactor::fill(factor_source_id);

            let existing = self
                .cache
                .peek_all_instances_for_factor_source(factor_source.factor_source_id.clone());

            let fill_cache = fill_cache_maybe_over_estimated.subtracting_existing(existing);

            let paths = self.paths_single_factor(
                factor_source_id,
                IndexMap::from_iter([(DerivationTemplate::AccountVeci, next_index_for_derivation)]),
                fill_cache,
            );

            let derived = self.derive(paths).await?;
            let (split_to_use_directly, split_to_cache) = self.split(veci, derived);

            // unconditionally set `veci`, since `split` should handle logic of it
            // being `None` or not.
            veci = Some(split_to_use_directly.account_veci()?.instance());
            to_cache = Some(split_to_cache);
        }
        let veci = veci.ok_or(CommonError::ExpectedValue)?;
        if let Some(to_cache) = to_cache {
            self.cache.append_for_factor(factor_source_id, to_cache)?;
        }
        Ok(ProvidedInstances::for_account_veci(self.cache, veci))
    }

    async fn provide_accounts_mfa(
        &self,
        number_of_instances_per_factor_source: usize,
        factor_sources: IndexSet<HDFactorSource>,
    ) -> Result<ProvidedInstances> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = FactorInstancesProvider;

    #[actix::test]
    async fn cache_is_always_filled_account_veci() {
        let cache = Rc::new(RefCell::new(FactorInstancesForEachNetworkCache::default()));
        let profile = Profile::default();
        let bdfs = HDFactorSource::sample();
        let sut = Sut::new(
            cache,
            NetworkID::Mainnet,
            profile,
            InstancesQuery::AccountVeci {
                factor_source: bdfs.clone(),
            },
        );
        let outcome = sut.provide().await.unwrap();
        assert!(outcome
            .cache_to_persist
            .peek_all_instances_for_factor_source(bdfs.factor_source_id)
            .unwrap()
            .is_full());
    }
}
