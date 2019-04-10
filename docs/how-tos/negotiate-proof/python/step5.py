        # 21.
        print_log('\n21. Verifier is verifying proof from Prover\n')
        assert await anoncreds.verifier_verify_proof(proof_req_json,
                                                             proof_json,
                                                             schemas_json,
                                                             cred_defs_json,
                                                             "{}", "{}")

        # 22.
        print_log('\n22. Closing both wallet_handles and pool\n')
        await wallet.close_wallet(issuer_wallet_handle)
        await wallet.close_wallet(prover_wallet_handle)
        await pool.close_pool_ledger(pool_handle)

        # 23.
        print_log('\n23. Deleting created wallet_handles\n')
        await wallet.delete_wallet(issuer_wallet_config, issuer_wallet_credentials)
        await wallet.delete_wallet(prover_wallet_config, prover_wallet_credentials)

        # 24.
        print_log('\n24. Deleting pool ledger config\n')
        await pool.delete_pool_ledger_config(pool_name)