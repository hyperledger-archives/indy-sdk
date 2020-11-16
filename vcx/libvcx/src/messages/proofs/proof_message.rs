use serde_json;
use serde_json::Value;
use error::prelude::*;
use api::VcxStateType;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofMessage {
    version: Option<String>,
    to_did: Option<String>,
    from_did: Option<String>,
    proof_request_id: Option<String>,
    pub libindy_proof: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<VcxStateType>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CredInfo {
    pub schema_id: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub timestamp: Option<u64>,
}

impl ProofMessage {
    pub fn new() -> ProofMessage {
        ProofMessage {
            version: None,
            to_did: None,
            from_did: None,
            proof_request_id: None,
            libindy_proof: String::new(),
            state: None
        }
    }

    pub fn new_reject() -> ProofMessage {
        ProofMessage {
            version: None,
            to_did: None,
            from_did: None,
            proof_request_id: None,
            libindy_proof: String::new(),
            state: Some(VcxStateType::VcxStateRejected)
        }
    }

    pub fn to_string(&self) -> VcxResult<String> {
        serde_json::to_string(&self)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidProof, format!("Cannot serialize proof: {}", err)))
    }

    pub fn from_str(payload: &str) -> VcxResult<ProofMessage> {
        serde_json::from_str(payload)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidProof, format!("Cannot deserialize proof: {}", err)))
    }

    pub fn get_credential_info(&self) -> VcxResult<Vec<CredInfo>> {
        get_credential_info(&self.libindy_proof)
    }
}

pub fn get_credential_info(proof: &str) -> VcxResult<Vec<CredInfo>> {
    let mut rtn = Vec::new();

    let credentials: Value = serde_json::from_str(&proof)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot deserialize libndy proof: {}", err)))?;

    if let Value::Array(ref identifiers) = credentials["identifiers"] {
        for identifier in identifiers {
            if let (Some(schema_id), Some(cred_def_id)) = (identifier["schema_id"].as_str(),
                                                           identifier["cred_def_id"].as_str()) {
                let rev_reg_id = identifier["rev_reg_id"]
                    .as_str()
                    .map(|x| x.to_string());

                let timestamp = identifier["timestamp"].as_u64();
                rtn.push(
                    CredInfo {
                        schema_id: schema_id.to_string(),
                        cred_def_id: cred_def_id.to_string(),
                        rev_reg_id,
                        timestamp,
                    }
                );
            } else { return Err(VcxError::from_msg(VcxErrorKind::InvalidProofCredentialData, "Cannot get identifiers")); }
        }
    }

    Ok(rtn)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use ::utils::constants::{SCHEMA_ID, CRED_DEF_ID, REV_REG_ID};
    use utils::devsetup::*;

    pub fn create_default_proof() -> ProofMessage {
        let mut proof = ProofMessage::new();
        proof.libindy_proof = ::utils::constants::INDY_PROOF_JSON.to_string();
        proof.from_did = Some(::settings::get_config_value(::settings::CONFIG_INSTITUTION_DID).unwrap());
        proof
    }

    #[test]
    fn test_proof_struct() {
        let _setup = SetupMocks::init();

        let offer = create_default_proof();
        assert_eq!(offer.from_did, Some(::settings::get_config_value(::settings::CONFIG_INSTITUTION_DID).unwrap()));
    }

    #[test]
    fn test_proof_reject() {
        let _setup = SetupMocks::init();

        let proof = ProofMessage::new_reject();
        assert_eq!(proof.state, Some(VcxStateType::VcxStateRejected));
    }

    #[test]
    fn test_serialize_deserialize() {
        let _setup = SetupDefaults::init();

        let proof = create_default_proof();
        let serialized = proof.to_string().unwrap();
        let proof2 = ProofMessage::from_str(&serialized).unwrap();
        assert_eq!(proof, proof2);
    }

    #[test]
    fn test_get_credential_data() {
        let _setup = SetupMocks::init();

        let mut proof = ProofMessage::new();
        proof.libindy_proof = "".to_string();
        assert_eq!(proof.get_credential_info().unwrap_err().kind(), VcxErrorKind::InvalidJson);

        proof.libindy_proof = "{}".to_string();
        assert_eq!(proof.get_credential_info().unwrap(), Vec::new());

        proof.libindy_proof = json!({"identifiers": []}).to_string();
        assert_eq!(proof.get_credential_info().unwrap(), Vec::new());

        proof.libindy_proof = json!({"identifiers": [{}]}).to_string();
        assert_eq!(proof.get_credential_info().unwrap_err().kind(), VcxErrorKind::InvalidProofCredentialData);

        proof.libindy_proof = json!({"identifiers": [{
            "schema_id": null,
            "cred_def_id": null,
        }]}).to_string();
        assert_eq!(proof.get_credential_info().unwrap_err().kind(), VcxErrorKind::InvalidProofCredentialData);

        proof.libindy_proof = json!({"identifiers": [{
            "schema_id": SCHEMA_ID,
            "cred_def_id": CRED_DEF_ID,
        }]}).to_string();
        let mut cred_info = CredInfo {
            schema_id: SCHEMA_ID.to_string(),
            cred_def_id: CRED_DEF_ID.to_string(),
            rev_reg_id: None,
            timestamp: None
        };
        assert_eq!(&proof.get_credential_info().unwrap()[0], &cred_info);

        proof.libindy_proof = json!({"identifiers": [{
            "schema_id": SCHEMA_ID,
            "cred_def_id": CRED_DEF_ID,
            "rev_reg_id": REV_REG_ID,
            "timestamp": 123
        }]}).to_string();
        cred_info.rev_reg_id = Some(REV_REG_ID.to_string());
        cred_info.timestamp = Some(123);
        assert_eq!(&proof.get_credential_info().unwrap()[0], &cred_info);
    }
}
