use serde::{Serialize};
use yaserde_derive::YaSerialize;

use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, YaSerialize)]
pub enum HashAlg {
    Sha1,
    Sha256
}

impl Display for HashAlg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HashAlg::Sha1 => write!(f, "SHA-1"),
            HashAlg::Sha256 => write!(f, "SHA-256")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, YaSerialize)]
pub struct HashType {
    #[yaserde(attribute)]
    pub alg: HashAlg,
    pub value: String
}

impl HashType {
    pub fn new(alg: HashAlg, value: String) -> HashType {
        HashType {
            alg,
            value
        }
    }
}
