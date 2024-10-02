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
}

pub type Result<T, E = CommonError> = std::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorSourceID(String);

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HDFactorSource {
    pub factor_source_id: FactorSourceID,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Profile;
