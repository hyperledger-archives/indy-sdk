#  Javascript for Postgres Plugin

This javascript library is for using the [postgres plugin](https://github.com/hyperledger/indy-sdk/tree/master/experimental/plugins/postgres_storage)
for [IndySdk](https://github.com/hyperledger/indy-sdk) wallets. 

## Coding the javascript
1. Start with [installing](https://github.com/hyperledger/indy-sdk/blob/master/README.md#installation) indysdk and [building](https://github.com/hyperledger/indy-sdk/tree/master/experimental/plugins/postgres_storage#installing-and-testing-the-postgres-plug-in) the postgres plugin.
2. Copy the postgres plugin library to your project (such as the resource directory in the same directory as src).
3. Create a file, such as `postgres.plugin.ts`, in your project.  Implement 
javascripts FFI functionality to load and use postgres library functions: 
`postgresstorage_init` and `init_storagetype`.  
3.1. See the example below.  Make sure the library name and path fit your setup.    
4. Use `import` or `require` to import `postgres.plugin.ts`.
5. In your application startup, need to make two calls to initialize postgres with indysdk.  
5.1. call `storagePlugin.postgresstorage_init();`  
5.2. call `storagePlugin.init_storagetype(initConfig, initCredentials);`.  
5.3. see below examples for the structure of the inputs. 
6. Both indy `createWallet` and `openWallet` calls require some postgres specific inputs as well.  Both create and openWallet calls  are the same. Call `indy.openWallet(walletConfig, walletCredentials);` 

### javascript FFI 
```
import * as ffi from 'ffi';
import * as ref from 'ref';
const int = ref.types.int;

const storagePlugin = ffi.Library(
    './resources/libindystrgpostgres.so',
    {
        postgresstorage_init: [int, []],
        init_storagetype: [int, ['string', 'string']],
    },
);
export = storagePlugin;
```

### initConfig
```
{ 
    "url": "postgress-server-db:5432", 
    "wallet_scheme": "MultiWalletSingleTable" 
}
```


### initCredentials
```
{ 
   "account": "user_name",
   "password": "user_name_password",
   "admin_account": "admin_name",
   "admin_password": "admin_name_password" 
}
```

### walletConfig
```
    { 'id': id,
       'storage_type': 'postgres_storage',
       'storage_config': {
            'url': 'postgress-server-db:5432',
            'wallet_scheme': 'MultiWalletSingleTable',
        },
    }
```

## walletCredentials
```
    {'key': 'key',
      'storage_credentials': {
           'account': 'user_name',
           'password': 'user_name_password',
           'admin_account': 'admin_name',
           'admin_password': 'admin_name_password' 
        },
    }
```

## Additional Information
If you need to build the posgres plugin, see the [build instructions](https://github.com/hyperledger/indy-sdk/tree/master/experimental/plugins/postgres_storage).  
