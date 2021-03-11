use serde::{Serialize};
use yaserde_derive::YaSerialize;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub enum Scope {
    Required,
    Optional,
    Excluded
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Required => write!(f, "Required"),
            Scope::Optional => write!(f, "Optional"),
            Scope::Excluded => write!(f, "exluded")
        }
    }
}
