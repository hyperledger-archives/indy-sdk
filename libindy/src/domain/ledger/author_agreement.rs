use std::collections::HashMap;

use indy_api_types::validation::Validatable;

use super::constants::{GET_TXN_AUTHR_AGRMT, GET_TXN_AUTHR_AGRMT_AML, TXN_AUTHR_AGRMT, TXN_AUTHR_AGRMT_AML, DISABLE_ALL_TXN_AUTHR_AGRMTS};

#[derive(Serialize, PartialEq, Debug)]
pub struct TxnAuthorAgreementOperation {
    #[serde(rename = "type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ratification_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retirement_ts: Option<u64>,
}

impl TxnAuthorAgreementOperation {
    pub fn new(text: Option<String>, version: String, ratification_ts: Option<u64>, retirement_ts: Option<u64>) -> TxnAuthorAgreementOperation {
        TxnAuthorAgreementOperation {
            _type: TXN_AUTHR_AGRMT.to_string(),
            text,
            version,
            ratification_ts,
            retirement_ts,
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct DisableAllTxnAuthorAgreementsOperation {
    #[serde(rename = "type")]
    _type: String,
}

impl DisableAllTxnAuthorAgreementsOperation {
    pub fn new() -> DisableAllTxnAuthorAgreementsOperation {
        DisableAllTxnAuthorAgreementsOperation {
            _type: DISABLE_ALL_TXN_AUTHR_AGRMTS.to_string(),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GetTxnAuthorAgreementData {
    pub digest: Option<String>,
    pub version: Option<String>,
    pub timestamp: Option<u64>,
}

impl Validatable for GetTxnAuthorAgreementData{
    fn validate(&self) -> Result<(), String> {
        match (self.digest.as_ref(), self.version.as_ref(), self.timestamp.as_ref()) {
            (Some(_), None, None) => Ok(()),
            (None, Some(_), None) => Ok(()),
            (None, None, Some(_)) => Ok(()),
            (None, None, None) => Ok(()),
            (digest, version, timestamp) => Err(format!("Only one of field can be specified: digest: {:?}, version: {:?}, timestamp: {:?}", digest, version, timestamp))
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetTxnAuthorAgreementOperation {
    #[serde(rename = "type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<u64>,
}

impl GetTxnAuthorAgreementOperation {
    pub fn new(data: Option<&GetTxnAuthorAgreementData>) -> GetTxnAuthorAgreementOperation {
        GetTxnAuthorAgreementOperation {
            _type: GET_TXN_AUTHR_AGRMT.to_string(),
            digest: data.as_ref().and_then(|d| d.digest.clone()),
            version: data.as_ref().and_then(|d| d.version.clone()),
            timestamp: data.as_ref().and_then(|d| d.timestamp),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AcceptanceMechanisms(pub HashMap<String, ::serde_json::Value>);

impl AcceptanceMechanisms {
    #[allow(dead_code)]
    pub fn new() -> Self {
        AcceptanceMechanisms(HashMap::new())
    }
}

impl Validatable for AcceptanceMechanisms {
    fn validate(&self) -> Result<(), String> {
        if self.0.is_empty() {
            return Err(String::from("Empty list of Acceptance Mechanisms has been passed"));
        }
        Ok(())
    }
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetAcceptanceMechanismOperation {
    #[serde(rename = "type")]
    _type: String,
    aml: AcceptanceMechanisms,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    aml_context: Option<String>,
}

impl SetAcceptanceMechanismOperation {
    pub fn new(aml: AcceptanceMechanisms, version: String, aml_context: Option<String>) -> SetAcceptanceMechanismOperation {
        SetAcceptanceMechanismOperation {
            _type: TXN_AUTHR_AGRMT_AML.to_string(),
            aml,
            version,
            aml_context
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetAcceptanceMechanismOperation {
    #[serde(rename = "type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}

impl GetAcceptanceMechanismOperation {
    pub fn new(timestamp: Option<u64>, version: Option<String>) -> GetAcceptanceMechanismOperation {
        GetAcceptanceMechanismOperation {
            _type: GET_TXN_AUTHR_AGRMT_AML.to_string(),
            timestamp,
            version,
        }
    }
}