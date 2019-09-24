#  Javascript for Postgres Plugin

This javascript library is for using the [postgres plugin](https://github.com/hyperledger/indy-sdk/tree/master/experimental/plugins/postgres_storage)
for [IndySdk](https://github.com/hyperledger/indy-sdk) wallets. 

## Use
1. Start with [installing](https://github.com/hyperledger/indy-sdk/blob/master/README.md#installation) indysdk and [installing](https://github.com/hyperledger/indy-sdk/tree/master/experimental/plugins/postgres_storage#installing-and-testing-the-postgres-plug-in) the postgres plugin.
2. Copy [postgres.plugin.ts](./postgres.plugin.ts) into your project.
3. Copy the postgres plugin library to resources directory (in the same directory as src)
4. In your application startup, need to make two calls to initialize postgres with indysdk
4.1. call `postgresPlugin.init();`  
4.2. call `postgresPlugin.setStoragetype(initConfig, initCredentials);`
5. Continue with calling IndySdk functions as normal.

### initConfig
`{ "url": "postgress-server-db:5432", "wallet_scheme": "MultiWalletSingleTable" }`


### initCredentials
`{ "account": "user_name",
          "password": "user_name_password",
          "admin_account": "admin_name",
          "admin_password": "admin_name_password }`

## Additional Information
If you need to build the posgres plugin, see the [build instructions](https://github.com/hyperledger/indy-sdk/tree/master/experimental/plugins/postgres_storage).  
