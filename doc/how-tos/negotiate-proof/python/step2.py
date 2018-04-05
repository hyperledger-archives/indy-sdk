        # 1.
        print_log('\n1. Creates Issuer wallet and opens it to get handle.\n')
        await
        wallet.create_wallet(pool_name, issuer_wallet_name, None, None, None)
        issuer_wallet_handle = await
        wallet.open_wallet(issuer_wallet_name, None, None)

        # 2.
        print_log('\n2. Creates Prover wallet and opens it to get handle.\n')
        await
        wallet.create_wallet(pool_name, prover_wallet_name, None, None, None)
        prover_wallet_handle = await
        wallet.open_wallet(prover_wallet_name, None, None)

        # 3.
        print_log('\n3. Issuer creates Claim Definition for Schema\n')
        schema = {
            'seqNo': seq_no,
            'dest': issuer_did,
            'data': {
                'name': 'gvt',
                'version': '1.0',
                'attr_names': ['age', 'sex', 'height', 'name']
            }
        }
        schema_json = json.dumps(schema)
        schema_key = {
            'name': schema['data']['name'],
            'version': schema['data']['version'],
            'did': schema['dest'],
        }
        claim_def_json = await
        anoncreds.issuer_create_and_store_claim_def(issuer_wallet_handle, issuer_did, schema_json, 'CL', False)
        print_log('Claim Definition: ')
        pprint.pprint(json.loads(claim_def_json))

        # 4.
        print_log('\n4. Prover creates Link Secret\n')
        link_secret_name = 'link_secret'
        await
        anoncreds.prover_create_master_secret(prover_wallet_handle, link_secret_name)

        # 5.
        print_log('\n5. Issuer create Cred Offer\n')
        claim_offer_json = await
        anoncreds.issuer_create_claim_offer(issuer_wallet_handle, schema_json, issuer_did, prover_did)
        print_log('Claim Offer: ')
        pprint.pprint(json.loads(claim_offer_json))

        # 6.
        print_log('\n6. Prover creates and stores Cred Request\n')
        claim_req_json = await
        anoncreds.prover_create_and_store_claim_req(prover_wallet_handle, prover_did, claim_offer_json,
                                                    claim_def_json, link_secret_name)
        print_log('Claim Request: ')
        pprint.pprint(json.loads(claim_req_json))

        # 7.
        print_log('\n7. Issuer creates Credential for received Cred Request\n')
        claim_json = json.dumps({
            'sex': ['male', '5944657099558967239210949258394887428692050081607692519917050011144233115103'],
            'name': ['Alex', '1139481716457488690172217916278103335'],
            'height': ['175', '175'],
            'age': ['28', '28']
        })
        (_, claim_json) = await
        anoncreds.issuer_create_claim(issuer_wallet_handle, claim_req_json, claim_json, -1)

        # 8.
        print_log('\n8. Prover processes and stores received Credential\n')
        await
        anoncreds.prover_store_claim(prover_wallet_handle, claim_json, None)