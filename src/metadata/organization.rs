use serde::{Serialize};
use yaserde_derive::YaSerialize;
use derive_builder::{Builder};

#[derive(Clone, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct OrganizationalEntity {
    pub name: Option<String>,
    pub url: Vec<String>,
    pub contact: Vec<OrganizationalContact>
}

#[derive(Default, Clone, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct OrganizationalContact {
    pub name: Option<String>,
    pub email: Vec<String>,
    pub phone: Vec<String>
}

impl OrganizationalContact {
    pub fn new(name: Option<String>, email: Vec<String>, phone: Vec<String>) -> OrganizationalContact {
        OrganizationalContact { name, email, phone }
    }
}
