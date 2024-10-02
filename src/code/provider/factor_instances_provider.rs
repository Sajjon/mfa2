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
        cache: FactorInstancesForEachNetworkCache,
        network_id: NetworkID,
        query: InstancesQuery,
        profile: Option<Profile>,
    ) -> Self {
        let cache = cache.clone_for_network(network_id);
        Self::for_specific_network(
            cache,
            query,
            NextDerivationEntityIndexAssigner::new(network_id, profile),
        )
    }

    pub async fn provide(self) -> Result<ProvidedInstances> {
        match self.query {
            InstancesQuery::AccountMfa {
                number_of_instances_per_factor_source,
                ref factor_sources,
            } => {
                self.provide_accounts_mfa(
                    number_of_instances_per_factor_source,
                    factor_sources.clone(),
                )
                .await
            }
            InstancesQuery::AccountVeci { ref factor_source } => {
                self.provide_account_veci(factor_source.clone()).await
            }
        }
    }
}

impl FactorInstancesProvider {
    async fn provide_account_veci(
        &self,
        factor_source: HDFactorSource,
    ) -> Result<ProvidedInstances> {
        let maybe_cached = self
            .cache
            .get_account_veci(factor_source.clone().factor_source_id);
        let mut maybe_next_index_for_derivation: Option<CAP26EntityIndex> = None;
        let mut veci: Option<HDFactorInstance> = None;
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
                    .next_account_veci(factor_source.clone().factor_source_id),
            )
        }
        assert!(!(veci.is_none() && maybe_next_index_for_derivation.is_none()));
        if let Some(next_index_for_derivation) = maybe_next_index_for_derivation {
            // must derive, should set `veci` if it is none.
            // furthermore, since we are deriving ANYWAY, we should also derive to Fill The Cache....
            let fill_cache_maybe_over_estimated =
                FillCacheQuantitiesForFactor::fill(factor_source.factor_source_id.clone());
            let existing = self
                .cache
                .peek_for_filling(factor_source.factor_source_id.clone());
            todo!()
        }

        todo!()
    }

    async fn provide_accounts_mfa(
        &self,
        number_of_instances_per_factor_source: usize,
        factor_sources: IndexSet<HDFactorSource>,
    ) -> Result<ProvidedInstances> {
        todo!()
    }
}
