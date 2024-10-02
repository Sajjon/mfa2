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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixOfFactorInstances {
    pub threshold: u16,
    pub threshold_factors: IndexSet<HDFactorInstance>,
    pub override_factors: IndexSet<HDFactorInstance>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntitySecurityState {
    Unsecurified(HDFactorInstance),
    Securified(MatrixOfFactorInstances),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    entity_security_state: EntitySecurityState,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Persona {
    entity_security_state: EntitySecurityState,
}

pub struct NextDerivationEntityIndexProfileAnalyzingAssigner {
    /// might be empty
    accounts_on_network: IndexSet<Account>,
    /// might be empty
    personas_on_network: IndexSet<Persona>,
}
impl NextDerivationEntityIndexProfileAnalyzingAssigner {
    pub fn new(profile: Option<Profile>) -> Self {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct NextDerivationEntityIndexWithLocalOffsets {
    local_offsets: HashMap<FactorSourceID, u32>,
}
pub struct NextDerivationEntityIndexAssigner {
    profile_analyzing: NextDerivationEntityIndexProfileAnalyzingAssigner,
    local_offsets: NextDerivationEntityIndexWithLocalOffsets,
}
impl NextDerivationEntityIndexAssigner {
    pub fn new(profile: Option<Profile>) -> Self {
        let profile_analyzing = NextDerivationEntityIndexProfileAnalyzingAssigner::new(profile);
        Self {
            profile_analyzing,
            local_offsets: NextDerivationEntityIndexWithLocalOffsets::default(),
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
            NextDerivationEntityIndexAssigner::new(profile),
        )
    }

    async fn provide_account_veci(
        &self,
        factor_source: HDFactorSource,
    ) -> Result<ProvidedInstances> {
        todo!()
    }

    async fn provide_accounts_mfa(
        &self,
        number_of_instances_per_factor_source: usize,
        factor_sources: IndexSet<HDFactorSource>,
    ) -> Result<ProvidedInstances> {
        todo!()
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Profile;

pub enum InstancesQuery {
    /// Uses the "next" derivation entity index for the derivation path
    /// The network is already known by the FactorInstancesProvider
    AccountVeci {
        /// The factor to use to derive the instance, typically the main BDFS.
        factor_source: HDFactorSource,
    },

    /// Uses a range of derivation paths, starting at the next, per factor source
    /// The network is already known by the FactorInstancesProvider
    ///
    /// N.B. we COULD have made this more advance/complex by passing a:
    /// `number_of_instances_for_each_factor_source: HashMap<HDFactorSource, usize>`
    /// but we don't need that complexity for now, we assume we want to get
    /// `number_of_instances_per_factor_source` for **each** factor source.
    ///
    /// `number_of_instances_per_factor_source` should be interpreted as
    /// `number_of_accounts_to_securify`.
    AccountMfa {
        number_of_instances_per_factor_source: usize,
        factor_sources: IndexSet<HDFactorSource>,
    },
    // PreDeriveKeysForFactorSource
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
