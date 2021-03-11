pub mod hash_type;
pub mod tool_type;
pub mod component;
pub mod organization;
pub mod license;
pub mod attached_text;
pub mod swid;
pub mod pedigree_type;
pub mod external_reference;
pub mod classification;
pub mod scope;

use chrono::{DateTime, Utc};
use std::time::SystemTime;
use serde::{Serialize};
use yaserde_derive::YaSerialize;
use crate::metadata::tool_type::ToolType;
use crate::metadata::component::Component;
use crate::metadata::organization::{OrganizationalContact, OrganizationalEntity};

#[derive(Debug, Serialize, YaSerialize)]
pub struct Metadata {
    time_stamp: String,
    tools: Vec<ToolType>,
    authors: Vec<OrganizationalContact>,
    component: Option<Component>,
    manufacturer: Vec<OrganizationalEntity>
}

impl Metadata {
    pub fn new(tools: Vec<ToolType>,
               authors: Vec<OrganizationalContact>,
               component: Option<Component>,
               manufacturer: Vec<OrganizationalEntity>
    ) -> Metadata {
        let time_stamp: DateTime<Utc> = SystemTime::now().into();
        Metadata {
            time_stamp: time_stamp.to_rfc3339(),
            tools, authors,
            component,
            manufacturer
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::metadata::hash_type::*;
    use crate::metadata::organization::*;
    use crate::metadata::tool_type::*;
    use crate::metadata::hash_type::HashAlg::*;

    #[test]
    fn tool_builder() {
        let mut tool_builder = ToolTypeBuilder::default();
        let tool: ToolType = tool_builder
            .name("name".to_string())
            .version("version".to_string())
            .vendor("vendor".to_string())
            .hashes([ HashType::new(Sha1, "1234567890".to_string()), HashType::new(Sha256, "0987654321".to_string())].to_vec())
            .build().unwrap();

        assert_eq!(tool.name, "name");
        assert_eq!(tool.version, "version");
        assert_eq!(tool.vendor, "vendor");
        let vec = tool.hashes;
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0].alg, Sha1);
        assert_eq!(vec[0].value, "1234567890".to_string());
        assert_eq!(vec[1].alg, Sha256);
        assert_eq!(vec[1].value, "0987654321".to_string());
    }

    #[test]
    fn author_builder() {
        let author: OrganizationalContact = OrganizationalContactBuilder::default()
            .name(Some("name".to_owned()))
            .phone(["phone".to_owned()].to_vec())
            .email(["email".to_owned()].to_vec())
            .build().unwrap();

        assert_eq!(author.name, Some(String::from("name")));
        assert_eq!(author.email, [String::from("email")].to_vec());
        assert_eq!(author.phone, [String::from("phone")].to_vec());
    }
}
