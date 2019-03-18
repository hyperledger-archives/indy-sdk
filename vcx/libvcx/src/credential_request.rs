
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CredentialRequest {
    pub libindy_cred_req: String,
    pub libindy_cred_req_meta: String,
    pub cred_def_id: String,
    pub tid: String,
    pub to_did: String,
    pub from_did: String,
    pub version: String,
    pub mid: String,
    pub msg_ref_id: Option<String>,
}

impl CredentialRequest {
    pub fn new(did: &str) -> CredentialRequest {
        CredentialRequest {
            to_did: String::new(),
            from_did: did.to_string(),
            mid: String::new(),
            tid: String::new(),
            version: String::new(),
            libindy_cred_req: String::new(),
            libindy_cred_req_meta: String::new(),
            cred_def_id: String::new(),
            msg_ref_id: None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use utils::constants::{CREDENTIAL_REQ_STRING, CRED_REQ, CRED_REQ_META};

    static TEMP_ISSUER_DID: &'static str = "4reqXeZVm7JZAffAoaNLsb";

    fn create_credential_req() -> CredentialRequest {
        ::settings::set_defaults();
        let issuer_did = ::settings::get_config_value(::settings::CONFIG_INSTITUTION_DID).unwrap();
        CredentialRequest::new(&issuer_did)
    }

    #[test]
    fn test_credential_request_struct() {
        let req = create_credential_req();
        let issuer_did = ::settings::get_config_value(::settings::CONFIG_INSTITUTION_DID).unwrap();
        assert_eq!(req.from_did, issuer_did);
    }

    #[test]
    fn test_serialize() {
        let cred1: CredentialRequest = serde_json::from_str(CREDENTIAL_REQ_STRING).unwrap();
        let serialized = serde_json::to_string(&cred1).unwrap();
        assert_eq!(serialized, CREDENTIAL_REQ_STRING)
    }

    #[test]
    fn test_deserialize() {
        let issuer_did = String::from("4reqXeZVm7JZAffAoaNLsb");
        let req: CredentialRequest = serde_json::from_str(CREDENTIAL_REQ_STRING).unwrap();
        assert_eq!(&req.libindy_cred_req, CRED_REQ);
    }

    #[test]
    fn test_create_credential_request_from_raw_message() {
        let credential_req: CredentialRequest = serde_json::from_str(CREDENTIAL_REQ_STRING).unwrap();

        assert_eq!(credential_req.tid, "cCanHnpFAD");
        assert_eq!(credential_req.to_did, "BnRXf8yDMUwGyZVDkSENeq");
        assert_eq!(credential_req.from_did, "GxtnGN6ypZYgEqcftSQFnC");
        assert_eq!(credential_req.version, "0.1");
        assert_eq!(credential_req.mid, "");
        assert_eq!(&credential_req.libindy_cred_req, CRED_REQ);
        assert_eq!(&credential_req.libindy_cred_req_meta, CRED_REQ_META);
    }
}

