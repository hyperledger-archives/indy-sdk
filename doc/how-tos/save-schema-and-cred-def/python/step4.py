    
        # 11.
        print_log('\n11. Creating and storing CRED DEFINITION using anoncreds as Trust Anchor, for the given Schema\n')
        cred_def_tag = 'cred_def_tag'
        cred_def_type = 'CL'
        cred_def_config = json.dumps({"support_revocation": False})
    
        (cred_def_id, cred_def_json) = await anoncreds.issuer_create_and_store_credential_def(wallet_handle, trust_anchor_did, json.dumps(schema_data),
                                                                                cred_def_tag, cred_def_type, cred_def_config)
        print_log('Credential definition: ')
        pprint.pprint(json.loads(cred_def_json))
    
        # 12.
        print_log('\n12. Closing wallet and pool\n')
        await wallet.close_wallet(wallet_handle)
        await pool.close_pool_ledger(pool_handle)
    
        # 13.
        print_log('\n13. Deleting created wallet\n')
        await wallet.delete_wallet(wallet_config, wallet_credentials)
    
        # 14.
        print_log('\n14. Deleting pool ledger config\n')
        await pool.delete_pool_ledger_config(pool_name)
