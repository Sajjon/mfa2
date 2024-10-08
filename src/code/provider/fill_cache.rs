use crate::prelude::*;

pub const CACHE_SIZE: u32 = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FillCacheQuantitiesForFactor {
    pub factor_source_id: FactorSourceID,

    /// Number of "account veci" instances to derive, using
    /// `factor_source_id` as the factor source
    pub account_vecis: u32,

    /// Number of "account mfa" instances to derive
    /// `factor_source_id` as the factor source
    pub account_mfa: u32,
}
impl FillCacheQuantitiesForFactor {
    pub fn fill(factor_source_id: FactorSourceID) -> Self {
        Self::new(factor_source_id, CACHE_SIZE, CACHE_SIZE)
    }
    pub fn new(factor_source_id: FactorSourceID, account_vecis: u32, account_mfa: u32) -> Self {
        Self {
            factor_source_id,
            account_mfa,
            account_vecis,
        }
    }

    pub fn subtracting_existing(
        self,
        existing: impl Into<Option<CollectionsOfFactorInstances>>,
    ) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FillCacheQuantitiesPerFactor {
    hidden_constructor: HiddenConstructor,
    pub per_factor_source: IndexMap<FactorSourceID, FillCacheQuantitiesForFactor>,
}
impl FillCacheQuantitiesPerFactor {
    pub fn just(item: FillCacheQuantitiesForFactor) -> Self {
        Self {
            hidden_constructor: HiddenConstructor,
            per_factor_source: IndexMap::from_iter([(item.factor_source_id.clone(), item)]),
        }
    }
}
