pub struct CycloneDX {
    xmlns: &'static str,
    bom_format: &'static str,
    spec_version: &'static str,
    version: &'static str
}

const XML_NAMESPACE: &'static str = "http://cyclonedx.org/schema/bom/1.2";
const BOM_FORMAT: &'static str = "CycloneDX";
const SPEC_VERSION: &'static str = "1.2";
const DEFAULT_VERSION: &'static str = "1";

impl CycloneDX {
    pub fn new() -> CycloneDX {
        CycloneDX {
            xmlns: XML_NAMESPACE,
            bom_format: BOM_FORMAT,
            spec_version: SPEC_VERSION,
            version: DEFAULT_VERSION
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::CycloneDX;

    #[test]
    fn new_bpm_has_defaults() {
        let bom = CycloneDX::new();

        assert_eq!(bom.xmlns, "http://cyclonedx.org/schema/bom/1.2");
        assert_eq!(bom.bom_format, "CycloneDX");
        assert_eq!(bom.spec_version, "1.2");
        assert_eq!(bom.version, "1");
    }
}
