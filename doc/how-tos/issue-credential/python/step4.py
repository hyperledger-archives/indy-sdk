        # 14.
        print_log('\n14. Issuer (Trust Anchor) is creating a Credential Offer for Prover\n')
        schema_json = json.dumps(schema)
        cred_offer_json = await anoncreds.issuer_create_credential_offer(wallet_handle, cred_def_id)
        print_log('Credential Offer: ')
        pprint.pprint(json.loads(cred_offer_json))

        # 15.
        print_log('\n15. Prover creates Credential Request for the given credential offer\n')
        (cred_req_json, cred_req_metadata_json) = await anoncreds.prover_create_credential_req(prover_wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id)
        print_log('Credential Request: ')
        pprint.pprint(json.loads(cred_req_json))

        # 16.
        print_log('\n16. Issuer (Trust Anchor) creates Credential for Credential Request\n')
        cred_values_json = json.dumps({
            'sex': ['male', '5944657099558967239210949258394887428692050081607692519917050011144233115103'],
            'name': ['Alex', '1139481716457488690172217916278103335'],
            'height': ['175', '175'],
            'age': ['28', '28']
        })
        (cred_json, _, _) = await anoncreds.issuer_create_credential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, None, None)
        print_log('Credential: ')
        pprint.pprint(json.loads(cred_json))

        # 17.
        print_log('\n17. Prover processes and stores Credential\n')
        await anoncreds.prover_store_credential(prover_wallet_handle, None, cred_req_metadata_json, cred_json, cred_def_json, None)

        # 18.
        print_log('\n18. Closing both wallet_handles and pool\n')
        await wallet.close_wallet(wallet_handle)
        await wallet.close_wallet(prover_wallet_handle)
        await pool.close_pool_ledger(pool_handle)

        # 19.
        print_log('\n19. Deleting created wallet_handles\n')
        await wallet.delete_wallet(wallet_config, wallet_credentials)
        await wallet.delete_wallet(prover_wallet_config, prover_wallet_credentials)

        # 20.
        print_log('\n20. Deleting pool ledger config\n')
        await pool.delete_pool_ledger_config(pool_name)
