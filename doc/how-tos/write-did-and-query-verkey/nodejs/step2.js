    // Tell SDK which pool you are going to use. You should have already started
    // this pool using docker compose or similar. Here, we are dumping the config
    // just for demonstration purposes.

    // 1.
    log('1. Creates a new local pool ledger configuration that is used later when connecting to ledger.')
    const poolName = 'pool'
    const genesisFilePath = await util.getPoolGenesisTxnPath(poolName)
    const poolConfig = {'genesis_txn': genesisFilePath}
    await indy.createPoolLedgerConfig(poolName, poolConfig)

    // 2.
    log('2. Open pool ledger and get handle from libindy')
    const poolHandle = await indy.openPoolLedger(poolName, undefined)

    // 3.
    log('3. Creating new secure wallet')
    const walletName = {"id": "wallet"}
    const walletCredentials = {"key": "wallet_key"}
    await indy.createWallet(walletName, walletCredentials)

    // 4.
    log('4. Open wallet and get handle from libindy')
    const walletHandle = await indy.openWallet(walletName, walletCredentials)

