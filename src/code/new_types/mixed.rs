use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HiddenConstructor;

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

        if derivation_path.entity_kind != CAP26EntityKind::Account {
            return Err(CommonError::EntityKindDiscrepancy);
        }

        if derivation_path.key_space() != KeySpace::Unsecurified {
            return Err(CommonError::KeySpaceDiscrepancy);
        }

        if derivation_path.key_kind != CAP26KeyKind::TransactionSigning {
            return Err(CommonError::KeyKindDiscrepancy);
        }

        Ok(Self {
            hidden_constructor: HiddenConstructor,
            instance,
        })
    }
    pub fn network_id(&self) -> NetworkID {
        self.instance.derivation_path.network_id.clone()
    }
    pub fn instance(&self) -> HDFactorInstance {
        self.instance.clone()
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
        if derivation_path.entity_kind != CAP26EntityKind::Identity {
            return Err(CommonError::EntityKindDiscrepancy);
        }

        if derivation_path.key_space() != KeySpace::Unsecurified {
            return Err(CommonError::KeySpaceDiscrepancy);
        }

        if derivation_path.key_kind != CAP26KeyKind::TransactionSigning {
            return Err(CommonError::KeyKindDiscrepancy);
        }

        Ok(Self {
            hidden_constructor: HiddenConstructor,
            instance,
        })
    }
    pub fn network_id(&self) -> NetworkID {
        self.instance.derivation_path.network_id.clone()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

/// A collection of sets of FactorInstances,
/// all on the same network
/// all from the same factor source
/// for different DerivationTemplates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionsOfFactorInstances {
    hidden_constructor: HiddenConstructor,
    pub network: NetworkID,
    pub factor_source_id: FactorSourceID,
    pub unsecurified_accounts: IndexSet<AccountVeci>,
    pub unsecurified_identities: IndexSet<IdentityVeci>,
}
impl CollectionsOfFactorInstances {
    pub fn empty(network: NetworkID, factor_source_id: FactorSourceID) -> Self {
        Self::new(network, factor_source_id, IndexSet::new(), IndexSet::new()).unwrap()
    }
    pub fn new(
        network: NetworkID,
        factor_source_id: FactorSourceID,
        unsecurified_accounts: IndexSet<AccountVeci>,
        unsecurified_identities: IndexSet<IdentityVeci>,
    ) -> Result<Self> {
        if !(unsecurified_accounts
            .iter()
            .all(|f| f.network_id() == network)
            && unsecurified_identities
                .iter()
                .all(|f| f.network_id() == network))
        {
            return Err(CommonError::NetworkDiscrepancy);
        }

        if !(unsecurified_accounts
            .iter()
            .all(|f| f.network_id() == network)
            && unsecurified_identities
                .iter()
                .all(|f| f.network_id() == network))
        {
            return Err(CommonError::FactorSourceDiscrepancy);
        }

        Ok(Self {
            hidden_constructor: HiddenConstructor,
            network,
            factor_source_id,
            unsecurified_accounts,
            unsecurified_identities,
        })
    }
}
