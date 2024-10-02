mod code;

pub mod prelude {
    pub use crate::code::*;

    pub(crate) use std::collections::HashMap;

    pub(crate) use indexmap::{IndexMap, IndexSet};
    pub(crate) use itertools::Itertools;
    pub(crate) use thiserror::Error;
}

pub(crate) use prelude::*;
