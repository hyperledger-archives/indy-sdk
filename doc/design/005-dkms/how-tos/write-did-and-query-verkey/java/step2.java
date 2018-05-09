        // Tell SDK which pool you are going to use. You should have already started
        // this pool using docker compose or similar.
        System.out.println("\n1. Creating a new local pool ledger configuration that can be used later to connect pool nodes.\n");
        Pool.createPoolLedgerConfig(poolName, poolConfig).get();

        System.out.println("\n2. Open pool ledger and get the pool handle from libindy.\n");
        Pool pool = Pool.openPoolLedger(poolName, "{}").get();

        System.out.println("\n3. Creates a new identity wallet\n");
        Wallet.createWallet(poolName, walletName, "default", null, null).get();

        System.out.println("\n4. Open identity wallet and get the wallet handle from libindy\n");
        Wallet walletHandle = Wallet.openWallet(walletName, null, null).get();

