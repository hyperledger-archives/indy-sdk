use indy_api_types::errors::{IndyResult, IndyErrorKind, IndyResultExt};
use rust_base58::ToBase58;
use super::proof::Proof;

/// Embodies the "proof" property at the root of a verifiable presentation.
/// This comes from the "aggregated_proof" field in the CL-based Ursa
/// struct. It essentially proves that all the credentials are associated
/// with the same link secret.
#[derive(Debug, Serialize, Deserialize)]
struct W3cPresentationProof {
    #[serde(rename = "type")]
    typ: String,
    #[serde(rename = "aggregateProof")]
    aggregate_proof: Option<String>,
}

impl W3cPresentationProof {
    fn from_proof(proof: &Proof) -> W3cPresentationProof {
        //let crypto_proof: &CryptoProof = &proof.proof;
        //let ag_result = serde_json::to_string(&crypto_proof.aggregated_proof);
        let mut ag: Option<String> = None;
        // Kludge. We need the JSON text for aggregated_proof. We'd like to just serialize
        // it here. However, it comes from a data member of our struct that is private. The
        // correct fix is probably to make that member public, but that requires a change to Ursa.
        // For now, just serialize the containing data structure, then pick out from it the part
        // we need. This is highly inefficient at runtime, but it's efficient in terms of
        // developer effort right now. TODO: go back and fix this tech debt by modifying Ursa.
        let crypto_proof_json = serde_json::to_string(&proof.proof).unwrap();
        use serde_json::Value;
        let v: Value = serde_json::from_str(&crypto_proof_json).unwrap(); // <-- yuck! expensive
        let ap = &v["aggregated_proof"];
        if ap.is_object() {
            ag = serde_json::to_string(ap).ok();
        }
        if ag.is_some() {
            ag = Some(ag.unwrap().as_bytes().to_base58());
        }
        W3cPresentationProof {
            typ: "AnonCredPresentationProofv1".to_string(),
            aggregate_proof: ag
        }
    }
}

/// Embodies a VC inside a verifiable presentation, as opposed to a VC
/// as issued. Disclosed fields in the CL proof become fields under
/// "credentialSubject" in this credential. The "proof" field for each
/// of these credentials contains a primaryProof as well as a
/// nonRevocationProof.
#[derive(Debug, Serialize, Deserialize)]
struct DerivedCredential {
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(rename = "type")]
    typ: Vec<String>,
    issuer: String,
}

/// Embodies a verifiable presentation containing one or more derived
/// credentials, plus an aggregate proof for the presentation as a whole.
/// This is the data structure that should be sent and received for
/// standards-based interoperability.
#[derive(Debug, Serialize, Deserialize)]
struct VerifiablePresentation {
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(rename = "type")]
    typ: String,
    #[serde(rename = "verifiableCredential")]
    creds: Vec<DerivedCredential>,
    proof: Option<W3cPresentationProof>,
}

/// Convert the JSON for a ZKP into the JSON for a W3C Verifiable
/// Presentation.
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
            typ: vec!["VerifiableCredential".to_string()],
            issuer: "insert DID here".to_string(),
        }],
        proof: Some(W3cPresentationProof::from_proof(proof)),
    };
    serde_json::to_string(&preso)
        .to_indy(IndyErrorKind::InvalidState, "Cannot serialize FullProof")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn to_vp_works() {
        let proof: Proof = serde_json::from_str(FAKE_PROOF_JSON).unwrap();
        let vp = to_vp(&proof).unwrap();
        let mut errors: Vec<String> = Vec::new();
        let v: Value = serde_json::from_str(&vp).unwrap();

        check_structure(&v, "@context", "HAS https://www.w3.org/2018/credentials/v1", &mut errors);
        check_structure(&v, "type", "LIKE VerifiablePresentation", &mut errors);
        if check_structure(&v, "verifiableCredential", "is array", &mut errors) {
            let vcs = v["verifiableCredential"].as_array().unwrap();
            let mut i: usize = 0;
            for vc in vcs {
                check_vc(&vc, i, &mut errors);
                i += 1;
            }
        }
        if check_structure(&v, "proof", "is object", &mut errors) {
            check_structure(&v["proof"], "proof/type", "LIKE AnonCredPresentationProofv1", &mut errors);
            check_structure(&v["proof"], "proof/aggregateProof", "LIKE ^[123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz]{50,}$", &mut errors);
        }

        if !errors.is_empty() {
            panic!("W3C presentation structure has errors: {}.\n\nPresentation was: {}",
                   &errors.join(". "), &vp);
        }
    }

    fn array_has_value(candidate: &Value, value: &str) -> bool {
        if candidate.is_array() {
            let ar = candidate.as_array().unwrap();
            for i in 0..ar.len() {
                let item = ar[i].to_string();
                // Ignore the delimiting quotes around str value. Compare inner only.
                let mut txt = item.as_str();
                let bytes = txt.as_bytes();
                if bytes.len() >= 2 && (bytes[0] == b'"') && (bytes[bytes.len() - 1] == b'"') {
                    txt = &item.as_str()[1..item.len() - 1];
                }
                if txt.eq(value) {
                    return true;
                }
            }
        }
        false
    }

    fn text_matches_regex(candidate: &Value, regex: &str) -> bool {
        if candidate.is_string() {
            use regex::Regex;
            let pat = Regex::new(regex).unwrap();
            if pat.is_match(candidate.as_str().unwrap()) {
                return true;
            }
        }
        false
    }

    fn check_structure(container: &Value, path: &str, expected: &str, errors: &mut Vec<String>) -> bool {
        let mut ok = false;
        let i = path.rfind('/');
        let subitem = if i.is_some() { &path[i.unwrap() + 1..] } else { &path[..] };
        let item = &container[subitem];
        if !item.is_null() {
            match expected {
                "is array" => ok = item.is_array(),
                "is object" => ok = item.is_object(),
                "is number" => ok = item.is_number(),
                "is string" => ok = item.is_string(),
                _ => {
                    if expected[0..4].eq("HAS ") {
                        ok = array_has_value(item, &expected[4..]);
                    } else if expected[0..5].eq("LIKE ") {
                        ok = text_matches_regex(item, &expected[5..]);
                    }
                }
            }
        }
        if !ok {
            errors.push(format!("Expected {} {}", path.to_string(), expected));
        }
        ok
    }

    fn check_vc(vc: &Value, i: usize, errors: &mut Vec<String>) {
        let prefix = format!("verifiableCredential[{}]", i);
        // Make this function a bit less verbose/repetitive.
        macro_rules! check {( $item:expr, $path:expr, $ex:expr ) => {
            check_structure($item, format!($path, &prefix).as_str(), $ex, errors) }}
        check!(&vc, "{}/type", "HAS VerifiableCredential");
        check!(&vc, "{}/@context", "HAS https://www.w3.org/2018/credentials/v1");
        if check!(&vc, "{}/credentialSchema", "is object") {
            let sch = &vc["credentialSchema"];
            check!(&sch, "{}/credentialSchema/id", "LIKE ^did:");
            check!(&sch, "{}/credentialSchema/type", "LIKE ^did:");
        }
        check!(&vc, "{}/issuer", "LIKE ^did:");
        check!(&vc, "{}/credentialSubject", "is object");
    }

    // This JSON exhibits the actual structure of a proof, but numeric values
    // are wrong and strings have been shortened. Thus, it should deserialize
    // correctly but will not validate.
    const FAKE_PROOF_JSON: &'static str = r#"{
  "proof":{
    "proofs":[
      {
        "primary_proof":{
          "eq_proof":{
            "revealed_attrs":{
              "height":"175",
              "name":"1139481716457488690172217916278103335"
            },
            "a_prime":"5817705...096889",
            "e":"1270938...756380",
            "v":"1138...39984052",
            "m":{
              "master_secret":"375275...0939395",
              "sex":"3511483...897083518",
              "age":"13430...63372249"
            },
            "m2":"1444497...2278453"
          },
          "ge_proofs":[
            {
              "u":{
                "1":"152500...3999140",
                "2":"147748...2005753",
                "0":"8806...77968",
                "3":"10403...8538260"
              },
              "r":{
                "2":"15706...781609",
                "3":"343...4378642",
                "0":"59003...702140",
                "DELTA":"9607...28201020",
                "1":"180097...96766"
              },
              "mj":"134300...249",
              "alpha":"827896...52261",
              "t":{
                "2":"7132...47794",
                "3":"38051...27372",
                "DELTA":"68025...508719",
                "1":"32924...41082",
                "0":"74906...07857"
              },
              "predicate":{
                "attr_name":"age",
                "p_type":"GE",
                "value":18
              }
            }
          ]
        },
        "non_revoc_proof":null
      }
    ],
    "aggregated_proof":{
      "c_hash":"108743...92564",
      "c_list":[
        [0,1,2,3,4,255],
        [0,1,2,3,4,255],
        [0,1,2,3,4,255],
        [0,1,2,3,4,255],
        [0,1,2,3,4,255],
        [0,1,2,3,4,255]
      ]
    }
  },
  "requested_proof":{
    "revealed_attrs":{
      "attr1_referent":{
        "sub_proof_index":0,
        "raw":"Alex",
        "encoded":"1139481716457488690172217916278103335"
      }
    },
    "revealed_attr_groups":{
      "attr4_referent":{
        "sub_proof_index":0,
        "values":{
          "name":{
            "raw":"Alex",
            "encoded":"1139481716457488690172217916278103335"
          },
          "height":{
            "raw":"175",
            "encoded":"175"
          }
        }
      }
    },
    "self_attested_attrs":{
      "attr3_referent":"8-800-300"
    },
    "unrevealed_attrs":{
      "attr2_referent":{
        "sub_proof_index":0
      }
    },
    "predicates":{
      "predicate1_referent":{
        "sub_proof_index":0
      }
    }
  },
  "identifiers":[
    {
      "schema_id":"NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0",
      "cred_def_id":"NcYxi...cYDi1e:2:gvt:1.0:TAG_1",
      "rev_reg_id":null,
      "timestamp":null
    }
  ]
}"#;

}
