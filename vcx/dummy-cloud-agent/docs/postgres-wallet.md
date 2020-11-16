# VCX Agency with PostgreSQL wallet

In order for things to work, you will have to compile
 - IndySDK postgresql wallet [plugin](https://github.com/hyperledger/indy-sdk/tree/master/experimental/plugins/postgres_storage)
and make sure it's in your system's library directory (`/usr/local/lib` for Mac, `/usr/lib` for Linux).
- Adjust agency configuration. Here is sample agency pgsql [configuration](../config/pgsql-config.json). Note that
it's important to use `MultiWalletSingleTableSharedPool` pgsql storage strategy, otherwise, at current implementation 
the postgres DB will easily get overloaded with too many connections.
- You can run PostgreSQL in docker like this: 
```
docker run -d --name postgres-agency \
           -v pgdata-agency:/var/lib/postgresql/data \
           -e POSTGRES_PASSWORD=mysecretpassword \
           -p 5432:5432 postgres:12.1
```

 
     
