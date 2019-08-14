println!("18. Prover gets Credentials for Proof Request");
let proof_req_json = json!({
    "nonce": "123432421212",
    "name": "proof_req_1",
    "version": "0.1",
    "requested_attributes": {
        "attr1_referent": {
            "name": "name",
            "restrictions": {
                "issuer_did": trustee_did,
                "schema_id": schema_id
            }
        }
    },
    "requested_predicates": {
        "predicate1_referent": {
            "name": "age",
            "p_type": ">=",
            "p_value": 18,
            "restrictions": {
                "issuer_did": trustee_did
            }
        }
    }
});
println!("Proof Request: {}", proof_req_json);

let creds_for_proof_request_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_req_json.to_string()).wait().unwrap();
println!("Credentials for Proof Request: {}", creds_for_proof_request_json);
