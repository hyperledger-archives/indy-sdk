        # 12.
        print_log('\n12. Creating Prover wallet and opening it to get the handle\n')
        prover_did = 'VsKV7grR1BUE29mG2Fm2kX'
        prover_wallet_config = json.dumps({"id": "prover_wallet"})
        prover_wallet_credentials = json.dumps({"key": "prover_wallet_key"})
        await wallet.create_wallet(prover_wallet_config, prover_wallet_credentials)
        prover_wallet_handle = await wallet.open_wallet(prover_wallet_config, prover_wallet_credentials)

        # 13.
        print_log('\n13. Prover is creating Master Secret\n')
        master_secret_name = 'master_secret'
        master_secret_id = await anoncreds.prover_create_master_secret(prover_wallet_handle, master_secret_name)
