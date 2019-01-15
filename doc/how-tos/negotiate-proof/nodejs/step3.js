    log("9. Prover gets Credentials for Proof Request")
    const proofRequest = {
        'nonce': '123432421212',
        'name': 'proof_req_1',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {
                'name': 'name',
                'restrictions': [{
                    'cred_def_id': credDefId
                    /*
                    'issuer_did': issuerDid,
                    'schema_key': schemaKey
                    */
                }]
            }
        },
        'requested_predicates': {
            'predicate1_referent': {
                'name': 'age',
                'p_type': '>=',
                'p_value': 18,
                'restrictions': [{'issuer_did': issuerDid}]
            }
        }
    }
    const credsForProofRequest = await indy.proverGetCredentialsForProofReq(proverWalletHandle, proofRequest)
