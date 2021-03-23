use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

use classification::Classification;
use external_reference::ExternalReference;
use pedigree_type::PedigreeType;
use scope::Scope;
use swid::SwidType;

use crate::common::hash_type::HashType;
use crate::common::license::Licenses;
use crate::common::organization::OrganizationalEntity;

pub mod classification;
pub mod external_reference;
pub mod pedigree_type;
pub mod scope;
pub mod swid;

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ns",
    default_namespace = "ns",
    namespace = "ns: http://cyclonedx.org/schema/bom/1.2"
)]
pub struct Component {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    pub component_type: Classification,

    #[serde(rename = "mime-type")]
    #[yaserde(rename = "mime-type", attribute)]
    pub mime_type: Option<String>,

    #[serde(rename = "bom-ref")]
    #[yaserde(rename = "bom-ref", attribute)]
    pub bom_ref: Option<String>,

    #[yaserde(prefix = "ns")]
    pub supplier: Option<OrganizationalEntity>,
    #[yaserde(prefix = "ns")]
    pub author: Option<String>,
    #[yaserde(prefix = "ns")]
    pub publisher: Option<String>,
    #[yaserde(prefix = "ns")]
    pub group: Option<String>,
    #[yaserde(prefix = "ns")]
    pub name: Option<String>,
    #[yaserde(prefix = "ns")]
    pub version: Option<String>,
    #[yaserde(prefix = "ns")]
    pub description: Option<String>,
    #[yaserde(prefix = "ns")]
    pub scope: Option<Scope>,
    #[yaserde(prefix = "ns")]
    pub hashes: Vec<HashType>,
    #[yaserde(prefix = "ns")]
    pub licenses: Vec<Licenses>,
    #[yaserde(prefix = "ns")]
    pub copyright: Option<String>,
    #[yaserde(prefix = "ns")]
    pub purl: Option<String>,
    #[yaserde(prefix = "ns")]
    pub swid: Option<SwidType>,
    #[yaserde(prefix = "ns")]
    pub modified: Option<bool>,
    pub pedigree: Option<PedigreeType>,
    pub external_references: Vec<ExternalReference>,
    pub components: Vec<Component>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::attached_text::BomEncoding;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;
    use yaserde::de;

    #[test]
    pub fn can_decode() {
        let reader = setup("component-1.2.xml");

        let component: Component = de::from_reader(reader).unwrap();

        assert_eq!(component.name.unwrap(), "Acme Application");
        assert_eq!(component.version.unwrap(), "9.1.1");
        let swid = component.swid.unwrap();
        assert_eq!(
            swid.tag_id,
            "swidgen-242eb18a-503e-ca37-393b-cf156ef09691_9.1.1"
        );
        assert_eq!(swid.name, "Acme Application");
        assert_eq!(swid.version.unwrap(), "9.1.1");
        let text_type = swid.text.unwrap();
        assert_eq!(text_type.content_type.unwrap(), "text/xml");
        assert_eq!(text_type.encoding.unwrap(), BomEncoding::Base64);
        assert_eq!(text_type.value, "text value");
    }

    fn setup(file: &str) -> BufReader<File> {
        let mut test_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_folder.push("resources/test/".to_owned() + file);
        let file = File::open(test_folder);
        let reader = BufReader::new(file.unwrap());
        reader
    }
}
