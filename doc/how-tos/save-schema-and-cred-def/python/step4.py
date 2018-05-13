        # 11.
        print_log('\n11. Creating and storing CRED DEFINITION using anoncreds as Trust Anchor, for the given Schema\n')
        cred_def = await anoncreds.issuer_create_and_store_claim_def(
            wallet_handle, trust_anchor_did, json.dumps(schema), 'CL', False)
        print_log('Returned Cred Definition:\n')
        pprint.pprint(json.loads(cred_def))

        # 12.
        print_log('\n12. Closing wallet and pool\n')
        await wallet.close_wallet(wallet_handle)
        await pool.close_pool_ledger(pool_handle)

        # 13.
        print_log('\n13. Deleting created wallet\n')
        await wallet.delete_wallet(wallet_name, None)

        # 14.
        print_log('\n14. Deleting pool ledger config\n')
        await pool.delete_pool_ledger_config(pool_name)