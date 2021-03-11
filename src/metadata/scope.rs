use serde::Serialize;
use std::fmt;
use std::fmt::{Display, Formatter};
use yaserde_derive::YaSerialize;

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub enum Scope {
    Required,
    Optional,
    Excluded,
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Required => write!(f, "Required"),
            Scope::Optional => write!(f, "Optional"),
            Scope::Excluded => write!(f, "exluded"),
        }
    }
}
