        # Tell SDK which pool you are going to use. You should have already started
        # this pool using docker compose or similar. Here, we are dumping the config
        # just for demonstration purposes.
        pool_config = json.dumps({'genesis_txn': str(genesis_file_path)})
        print_log('\n1. Create new pool ledger configuration to connect to ledger.\n')
        await pool.create_pool_ledger_config(config_name=pool_name, config=pool_config)

        print_log('\n2. Open ledger and get handle\n')
        pool_handle = await pool.open_pool_ledger(config_name=pool_name, config=None)

        print_log('\n3. Create new identity wallet\n')
        await wallet.create_wallet(wallet_config, wallet_credentials)

        print_log('\n4. Open identity wallet and get handle\n')
        wallet_handle = await wallet.open_wallet(wallet_config, wallet_credentials)
