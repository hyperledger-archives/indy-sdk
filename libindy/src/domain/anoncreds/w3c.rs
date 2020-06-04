use indy_api_types::errors::{IndyResult, IndyErrorKind, IndyResultExt};
use super::proof::Proof;

#[derive(Debug, Serialize, Deserialize)]
struct W3cProof {
    #[serde(rename = "type")]
    typ: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DerivedCredential {
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(rename = "type")]
    typ: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VerifiablePresentation {
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(rename = "type")]
    typ: String,
    #[serde(rename = "verifiableCredential")]
    creds: Vec<DerivedCredential>,
    proof: W3cProof,
}

#[allow(dead_code)]
pub fn to_vp(proof: &Proof) -> IndyResult<String> {
    let preso = VerifiablePresentation {
        context: vec![
            "https://www.w3.org/2018/credentials/v1".to_string(),
            proof.identifiers[0].cred_def_id.0.to_string()
        ],
        typ: "VerifiablePresentation".to_string(),
        creds: vec![DerivedCredential {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
            ],
            typ: vec!["VerifiableCredential".to_string()]
        }],
        proof: W3cProof { typ: "AnonCredPresentationProofv1".to_string() },
    };
    serde_json::to_string(&preso)
        .to_indy(IndyErrorKind::InvalidState, "Cannot serialize FullProof")
}
