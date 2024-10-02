use std::{collections::HashMap, os::unix::net};

use indexmap::{IndexMap, IndexSet};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Error)]
pub enum CommonError {
    #[error("Error")]
    Fail,
}

pub type Result<T, E = CommonError> = std::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HiddenConstructor;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSourceID(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HDFactorInstance {
    pub derivation_path: DerivationPath,
    pub factor_source_id: FactorSourceID,
}

/// A FactorInstance with a derivation path that is used for
/// Account, Unsecurified, TransactionSigning
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccountVeci {
    hidden_constructor: HiddenConstructor,
    instance: HDFactorInstance,
}
impl AccountVeci {
    pub fn new(instance: HDFactorInstance) -> Result<Self> {
        let derivation_path = &instance.derivation_path;
        if derivation_path.entity_kind == CAP26EntityKind::Account
            && derivation_path.key_space() == KeySpace::Unsecurified
            && derivation_path.key_kind == CAP26KeyKind::TransactionSigning
        {
            Ok(Self {
                hidden_constructor: HiddenConstructor,
                instance,
            })
        } else {
            Err(CommonError::Fail)
        }
    }
    pub fn network_id(&self) -> NetworkID {
        self.instance.derivation_path.network_id.clone()
    }
}

/// A FactorInstance with a derivation path that is used for
/// Identity, Unsecurified, TransactionSigning
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IdentityVeci {
    hidden_constructor: HiddenConstructor,
    instance: HDFactorInstance,
}
impl IdentityVeci {
    pub fn new(instance: HDFactorInstance) -> Result<Self> {
        let derivation_path = &instance.derivation_path;
        if derivation_path.entity_kind == CAP26EntityKind::Identity
            && derivation_path.key_space() == KeySpace::Unsecurified
            && derivation_path.key_kind == CAP26KeyKind::TransactionSigning
        {
            Ok(Self {
                hidden_constructor: HiddenConstructor,
                instance,
            })
        } else {
            Err(CommonError::Fail)
        }
    }
    pub fn network_id(&self) -> NetworkID {
        self.instance.derivation_path.network_id.clone()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NetworkID {
    Mainnet,
    Testnet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CAP26EntityKind {
    Account,
    Identity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CAP26KeyKind {
    TransactionSigning,
    AuthenticationSigning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CAP26EntityIndex {
    Securified(u32),
    Unsecurified(u32),
}
impl CAP26EntityIndex {
    pub fn key_space(&self) -> KeySpace {
        match self {
            CAP26EntityIndex::Securified(_) => KeySpace::Securified,
            CAP26EntityIndex::Unsecurified(_) => KeySpace::Unsecurified,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeySpace {
    Unsecurified,
    Securified,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivationTemplate {
    /// Account, Unsecurified, TransactionSigning,
    /// Veci: Virtual Entity Creating (Factor)Instance
    AccountVeci,

    /// Identity, Unsecurified, TransactionSigning
    /// Veci: Virtual Entity Creating (Factor)Instance
    IdentityVeci,

    /// Account, Securified, AuthenticationSigning
    AccountRola,

    /// Account, Securified, TransactionSigning
    AccountMfa,

    /// Identity, Securified, TransactionSigning
    IdentityMfa,
}

/// A collection of sets of FactorInstances, all
/// on the same network, for different DerivationTemplates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionsOfFactorInstances {
    hidden_constructor: HiddenConstructor,
    pub network: NetworkID,
    pub unsecurified_accounts: IndexSet<AccountVeci>,
    pub unsecurified_identities: IndexSet<IdentityVeci>,
}
impl CollectionsOfFactorInstances {
    pub fn empty(network: NetworkID) -> Self {
        Self::new(network, IndexSet::new(), IndexSet::new()).unwrap()
    }
    pub fn new(
        network: NetworkID,
        unsecurified_accounts: IndexSet<AccountVeci>,
        unsecurified_identities: IndexSet<IdentityVeci>,
    ) -> Result<Self> {
        if unsecurified_accounts
            .iter()
            .all(|f| f.network_id() == network)
            && unsecurified_identities
                .iter()
                .all(|f| f.network_id() == network)
        {
            Ok(Self {
                hidden_constructor: HiddenConstructor,
                network,
                unsecurified_accounts,
                unsecurified_identities,
            })
        } else {
            Err(CommonError::Fail)
        }
    }
}

/// On one specific network
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorInstancesForSpecificNetworkCache(CollectionsOfFactorInstances);

impl FactorInstancesForSpecificNetworkCache {
    pub fn empty(network: NetworkID) -> Self {
        Self::new(CollectionsOfFactorInstances::empty(network))
    }
    pub fn new(instances: CollectionsOfFactorInstances) -> Self {
        Self(instances)
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

pub struct FactorInstancesProvider {
    /// A Clone of a cache, the caller MUST commit the changes to the
    /// original cache if they want to persist them.
    #[allow(dead_code)]
    cache: FactorInstancesForSpecificNetworkCache,
}

impl FactorInstancesProvider {
    fn for_specific_network(cache: FactorInstancesForSpecificNetworkCache) -> Self {
        Self { cache }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HDFactorSource {
    factor_source_id: FactorSourceID,
}

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

pub enum InstancesQuery {
    AccountMfa {
        accounts: usize,
        factor_sources: IndexSet<HDFactorSource>,
    },
    // IdentitiesMfa { identities: usize, factor_sources: IndexSet<HDFactorSource> },
    // AccountVeci,
    // IdentityVeci,
    // PreDeriveKeysForFactorSource
}

impl FactorInstancesProvider {
    pub fn new(
        cache: FactorInstancesForEachNetworkCache,
        network_id: NetworkID,
        query: InstancesQuery,
    ) -> Self {
        let cache = cache.clone_for_network(network_id);
        Self::for_specific_network(cache)
    }
    pub async fn provide(self) -> Result<ProvidedInstances> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvidedInstances {
    hidden_constructor: HiddenConstructor,

    /// The caller of FactorInstancesProvider::provide MUST override their
    /// original cache with this updated one if they want to persist the changes.
    pub cache_to_persist: FactorInstancesForEachNetworkCache,

    /// The factor instances that were provided to be used directly, this is sometimes
    /// empty, e.g. in the case of PreDeriveKeys for new FactorSource.
    ///
    /// And often this contains just some of the newly created instances, because
    /// some might have gone into the `cache_to_persist` instead.
    pub instances_to_be_used: IndexSet<HDFactorInstance>,
}
