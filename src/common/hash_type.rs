use serde::Serialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Debug, PartialEq, Serialize, YaSerialize, YaDeserialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum HashAlg {
    #[yaserde(rename = "SHA-1")]
    Sha1,
    #[yaserde(rename = "SHA-256")]
    Sha256,
}

impl Default for HashAlg {
    fn default() -> Self {
        HashAlg::Sha1
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, YaSerialize, YaDeserialize)]
#[yaserde(rename = "hash")]
pub struct HashType {
    #[yaserde(attribute)]
    pub alg: HashAlg,
    #[yaserde(text)]
    pub value: String,
}

impl HashType {
    pub fn new(alg: HashAlg, value: String) -> HashType {
        HashType { alg, value }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::hash_type::{HashAlg, HashType};
    use yaserde::ser::Config;

    #[test]
    fn print_xml() {
        let expected = HashType::new(
            HashAlg::Sha1,
            "25ed8e31b995bb927966616df2a42b979a2717f0".to_string(),
        );
        let parsed = yaserde::ser::to_string_with_config(
            &expected,
            &Config {
                perform_indent: false,
                write_document_declaration: false,
                indent_string: None,
            },
        )
        .unwrap();

        let actual: HashType = yaserde::de::from_str(parsed.as_str()).unwrap();

        assert_eq!(expected, actual);
    }
}
