        # 12.
        print_log('\n12. Creating Prover wallet and opening it to get the handle.\n')
        prover_did = 'VsKV7grR1BUE29mG2Fm2kX'
        prover_wallet_config = json.dumps({"id": "prover_wallet"})
        prover_wallet_credentials = json.dumps({"key": "prover_wallet_key"})
        try:
            await wallet.create_wallet(prover_wallet_config,prover_wallet_credentials)
        except IndyError as err:
            if err.error_code == ErrorCode.WalletAlreadyExistsError:
                pass        
        prover_wallet_handle = await wallet.open_wallet(prover_wallet_config, prover_wallet_credentials)

        # 13.
        print_log('\n13. Prover is creating Link Secret\n')
        prover_link_secret_name = 'link_secret'
        link_secret_id = await anoncreds.prover_create_master_secret(prover_wallet_handle,
                                                                     prover_link_secret_name)
