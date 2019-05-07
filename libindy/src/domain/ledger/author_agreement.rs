use super::constants::{TXN_AUTHR_AGRMT, GET_TXN_AUTHR_AGRMT, TXN_AUTHR_AGRMT_AML, GET_TXN_AUTHR_AGRMT_AML};

use std::collections::HashMap;

#[derive(Serialize, PartialEq, Debug)]
pub struct TxnAuthorAgreementOperation {
    #[serde(rename = "type")]
    _type: String,
    text: String,
    version: String,
}

impl TxnAuthorAgreementOperation {
    pub fn new(text: String, version: String) -> TxnAuthorAgreementOperation {
        TxnAuthorAgreementOperation {
            _type: TXN_AUTHR_AGRMT.to_string(),
            text,
            version
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GetTxnAuthorAgreementData {
    pub hash: Option<String>,
    pub version: Option<String>,
    pub timestamp: Option<u64>,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetTxnAuthorAgreementOperation {
    #[serde(rename = "type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<u64>,
}

impl GetTxnAuthorAgreementOperation {
    pub fn new(data: Option<&GetTxnAuthorAgreementData>) -> GetTxnAuthorAgreementOperation {
        GetTxnAuthorAgreementOperation {
            _type: GET_TXN_AUTHR_AGRMT.to_string(),
            hash: data.as_ref().and_then(|d| d.hash.clone()),
            version: data.as_ref().and_then(|d| d.version.clone()),
            timestamp: data.as_ref().and_then(|d| d.timestamp.clone()),
        }
    }
}

pub type AcceptanceMechanisms = HashMap<String, ::serde_json::Value>;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetAcceptanceMechanismOperation {
    #[serde(rename = "type")]
    _type: String,
    aml: AcceptanceMechanisms,
    #[serde(skip_serializing_if = "Option::is_none")]
    aml_context: Option<String>,
}

impl SetAcceptanceMechanismOperation {
    pub fn new(aml: AcceptanceMechanisms, aml_context: Option<String>) -> SetAcceptanceMechanismOperation {
        SetAcceptanceMechanismOperation {
            _type: TXN_AUTHR_AGRMT_AML.to_string(),
            aml,
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
}

impl GetAcceptanceMechanismOperation {
    pub fn new(timestamp: Option<u64>) -> GetAcceptanceMechanismOperation {
        GetAcceptanceMechanismOperation {
            _type: GET_TXN_AUTHR_AGRMT_AML.to_string(),
            timestamp,
        }
    }
}