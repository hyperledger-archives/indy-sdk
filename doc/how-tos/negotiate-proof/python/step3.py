        print_log('\n9. Prover gets Credentials for Proof Request\n')
        proof_request = {
            'nonce': '123432421212',
            'name': 'proof_req_1',
            'version': '0.1',
            'requested_attrs': {
                'attr1_referent': {
                    'name': 'name',
                    'restrictions': [{
                        'issuer_did': issuer_did,
                        'schema_key': schema_key
                    }]
                }
            },
            'requested_predicates': {
                'predicate1_referent': {
                    'attr_name': 'age',
                    'p_type': '>=',
                    'value': 18,
                    'restrictions': [{'issuer_did': issuer_did}]
                }
            }
        }
        print_log('Proof Request: ')
        pprint.pprint(proof_request)
        proof_req_json = json.dumps(proof_request)
        creds_for_proof_request_json = await
        anoncreds.prover_get_claims_for_proof_req(prover_wallet_handle, proof_req_json)
        creds_for_proof_request = json.loads(creds_for_proof_request_json)
        print_log('Credentials for Proof Request: ')
        pprint.pprint(creds_for_proof_request)
