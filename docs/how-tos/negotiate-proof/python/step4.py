        # 20.
        print_log('\n20. Prover creates Proof for Proof Request\n')
        prover_requested_creds = json.dumps({
            'self_attested_attributes': {},
            'requested_attributes': {
                'attr1_referent': {
                    'cred_id': prover_cred_for_attr1['referent'],
                    'revealed': True
                }
            },
            'requested_predicates': {
                'predicate1_referent': {
                    'cred_id': prover_cred_for_predicate1['referent']
                }
            }
        })
        print_log('Requested Credentials for Proving: ')
        pprint.pprint(json.loads(prover_requested_creds))

        prover_schema_id = json.loads(cred_offer_json)['schema_id']
        schemas_json = json.dumps({prover_schema_id: json.loads(issuer_schema_json)})
        cred_defs_json = json.dumps({cred_def_id: json.loads(cred_def_json)})
        proof_json = await anoncreds.prover_create_proof(prover_wallet_handle,
                                                         proof_req_json,
                                                         prover_requested_creds,
                                                         link_secret_id,
                                                         schemas_json,
                                                         cred_defs_json,
                                                         "{}")
        proof = json.loads(proof_json)
        assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_referent']["raw"]