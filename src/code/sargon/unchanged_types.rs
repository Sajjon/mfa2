use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Error)]
pub enum CommonError {
    #[error("Network Discrepancy")]
    NetworkDiscrepancy,

    #[error("FactorSource Discrepancy")]
    FactorSourceDiscrepancy,

    #[error("EntityKind Discrepancy")]
    EntityKindDiscrepancy,

    #[error("KeySpace Discrepancy")]
    KeySpaceDiscrepancy,

    #[error("KeyKind Discrepancy")]
    KeyKindDiscrepancy,

    #[error("Expected Value")]
    ExpectedValue,
}

pub type Result<T, E = CommonError> = std::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FactorSourceID([u8; 32]);
impl FactorSourceID {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    pub fn sample() -> Self {
        Self::new([0xaa; 32])
    }
    pub fn sample_other() -> Self {
        Self::new([0xbb; 32])
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HDFactorInstance {
    pub derivation_path: DerivationPath,
    pub factor_source_id: FactorSourceID,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatrixOfFactorInstances {
    pub threshold: u16,
    threshold_factors: Vec<HDFactorInstance>, // IndexSet, but need Hash.
    override_factors: Vec<HDFactorInstance>,  // IndexSet, but need Hash.
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EntitySecurityState {
    Unsecurified(HDFactorInstance),
    Securified(MatrixOfFactorInstances),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Account {
    entity_security_state: EntitySecurityState,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Persona {
    entity_security_state: EntitySecurityState,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HDFactorSource {
    pub factor_source_id: FactorSourceID,
}
impl HDFactorSource {
    pub fn new(factor_source_id: FactorSourceID) -> Self {
        Self { factor_source_id }
    }
    pub fn sample() -> Self {
        Self::new(FactorSourceID::sample())
    }
    pub fn sample_other() -> Self {
        Self::new(FactorSourceID::sample_other())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Profile {
    pub networks: IndexMap<NetworkID, ProfileOnNetwork>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileOnNetwork {
    pub network_id: NetworkID,
    pub accounts: IndexSet<Account>,
    pub personas: IndexSet<Persona>,
}
