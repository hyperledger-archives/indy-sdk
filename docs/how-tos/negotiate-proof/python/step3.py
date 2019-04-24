        # 18.
        print_log('\n18. Prover gets Credentials for Proof Request\n')
        proof_request = {
            'nonce': '123432421212',
            'name': 'proof_req_1',
            'version': '0.1',
            'requested_attributes': {
                'attr1_referent': {
                    'name': 'name',
                    "restrictions": {
                        "issuer_did": trust_anchor_did,
                        "schema_id": issuer_schema_id
                    }
                }
            },
            'requested_predicates': {
                'predicate1_referent': {
                    'name': 'age',
                    'p_type': '>=',
                    'p_value': 18,
                    "restrictions": {
                       "issuer_did": trust_anchor_did
                    }
                }
            }
        }
        print_log('Proof Request: ')
        pprint.pprint(proof_request)

        # 19. 
        print_log('\n19. Prover gets Credentials for attr1_referent anf predicate1_referent\n')
        proof_req_json = json.dumps(proof_request)
        prover_cred_search_handle = \
            await anoncreds.prover_search_credentials_for_proof_req(prover_wallet_handle, proof_req_json, None)

        creds_for_attr1 = await anoncreds.prover_fetch_credentials_for_proof_req(prover_cred_search_handle,
                                                                                 'attr1_referent', 1)
        prover_cred_for_attr1 = json.loads(creds_for_attr1)[0]['cred_info']
        print_log('Prover credential for attr1_referent: ')
        pprint.pprint(prover_cred_for_attr1)

        creds_for_predicate1 = await anoncreds.prover_fetch_credentials_for_proof_req(prover_cred_search_handle,
                                                                                      'predicate1_referent', 1)
        prover_cred_for_predicate1 = json.loads(creds_for_predicate1)[0]['cred_info']
        print_log('Prover credential for predicate1_referent: ')
        pprint.pprint(prover_cred_for_predicate1)

        await anoncreds.prover_close_credentials_search_for_proof_req(prover_cred_search_handle)
        