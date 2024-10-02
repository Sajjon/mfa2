use std::collections::HashMap;

use indexmap::IndexSet;
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

/// On one specific network
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorInstancesForSpecificNetworkCache {
    hidden_constructor: HiddenConstructor,
    pub network: NetworkID,
    pub unsecurified_accounts: IndexSet<AccountVeci>,
    pub unsecurified_identities: IndexSet<IdentityVeci>,
}
impl FactorInstancesForSpecificNetworkCache {
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
            .unwrap_or(FactorInstancesForSpecificNetworkCache {
                hidden_constructor: HiddenConstructor,
                network,
                unsecurified_accounts: IndexSet::new(),
                unsecurified_identities: IndexSet::new(),
            })
    }
}

pub struct FactorInstancesProvider {
    #[allow(dead_code)]
    cache: FactorInstancesForSpecificNetworkCache,
}

impl FactorInstancesProvider {
    fn for_specific_network(cache: FactorInstancesForSpecificNetworkCache) -> Self {
        Self { cache }
    }

    pub fn new(cache: FactorInstancesForEachNetworkCache, network_id: NetworkID) -> Self {
        let cache = cache.clone_for_network(network_id);
        Self::for_specific_network(cache)
    }
}
