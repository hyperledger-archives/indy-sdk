# Common errors

If one of the following possible errors happened to you, try to apply the solutions listed below.

Note that the errors presented here are language agnostic and as a matter of illustration, the example is given in python, but similar logic can be applied for the other languages.

1. Error `PoolLedgerConfigAlreadyExistsError`.   

    Delete config before creating:
    ```python
    try:
        await pool.create_pool_ledger_config(config_name=pool_name, config=pool_config)
    except IndyError:
        await pool.delete_pool_ledger_config(config_name=pool_name)
        await pool.create_pool_ledger_config(config_name=pool_name, config=pool_config)
    ```

2. Error `WalletAlreadyExistsError`.   

    Delete wallet before creating:
    ```python
    try:
        await wallet.create_wallet(wallet_config, wallet_credentials)
    except IndyError:
        await wallet.delete_wallet(wallet_name, wallet_credentials)
        await wallet.create_wallet(wallet_config, wallet_credentials)
    ```

3. Error `CommonIOError`.

    Make sure that you have set `genesis_file_path` to point to your `indy-sdk/cli/docker_pool_transactions_genesis`. 

4. Error `PoolLedgerTimeout`.
   
    Make sure that the pool of local nodes in Docker is running on the same ip/ports as 
    in the `docker_pool_transactions_genesis` (for further details see [How to start local nodes pool with docker](https://github.com/hyperledger/indy-sdk/blob/master/README.md#how-to-start-local-nodes-pool-with-docker))
