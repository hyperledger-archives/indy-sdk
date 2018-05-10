        print_log('\n10. Prover creates Proof for Proof Request\n')
        cred_for_attr_1 = creds_for_proof_request['attrs']['attr1_referent']
        referent = cred_for_attr_1[0]['referent']
        print_log('Referent: ')
        pprint.pprint(referent)
        chosen_claims_json = json.dumps({
            'self_attested_attributes': {},
            'requested_attrs': {
                'attr1_referent': [referent, True]
            },
            'requested_predicates': {
                'predicate1_referent': referent
            }
        })
        pprint.pprint(json.loads(chosen_claims_json))
        schemas_json = json.dumps({referent: schema})
        cdefs_json = json.dumps({referent: json.loads(cred_def_json)})
        revoc_regs_json = json.dumps({})
        proof_json = await
        anoncreds.prover_create_proof(prover_wallet_handle, proof_req_json, chosen_claims_json, schemas_json,
                                      'link_secret', cdefs_json, revoc_regs_json)
        proof = json.loads(proof_json)

        assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_referent'][1]
