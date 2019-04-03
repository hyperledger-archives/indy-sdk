"use strict";

const indy = require('indy-sdk');
//const util = require('./util');
const assert = require('assert');
const storagePlugin = require('./storagePlugin');

async function run() {

    console.log("gettingStarted.js -> started");

    // let poolName = 'pool1';
    // console.log(`Open Pool Ledger: ${poolName}`);
    // let poolGenesisTxnPath = await util.getPoolGenesisTxnPath(poolName);
    // let poolConfig = {
    //     "genesis_txn": poolGenesisTxnPath
    // };
    // try {
    //     await indy.createPoolLedgerConfig(poolName, poolConfig);
    // } catch(e) {
    //     if(e.message !== "PoolLedgerConfigAlreadyExistsError") {
    //         throw e;
    //     }
    // }

    // await indy.setProtocolVersion(2)

    // let poolHandle = await indy.openPoolLedger(poolName);

    console.log("==============================");
    console.log("=== Getting Trust Anchor credentials for Faber, Acme, Thrift and Government  ==");
    console.log("------------------------------");

    const result = storagePlugin.postgresstorage_init();
    console.log(result);

    console.log("\"Sovrin Steward\" -> Create wallet");
    
    let stewardWalletConfig = {
        'id': 'wallet_psx',
        'storage_type': 'postgres_storage',
        'storage_config': {
            "url": "postgres-db:5432"
        }
    }
    let stewardWalletCredentials = {
        'key': '1',
        'storage_credentials': {
            "account": "postgres",
            "password": "mysecretpassword",
            "admin_account": "postgres",
            "admin_password": "mysecretpassword"
        }
    }

    try {
        const result2 = await indy.createWallet(stewardWalletConfig, stewardWalletCredentials);
        console.log(result2);
    } catch(e) {
        if(e.message !== "WalletAlreadyExistsError") {
            throw e;
        } else {
            console.log('WalletAlreadyExists')
        }
    }

    console.log('open');

    let stewardWallet = await indy.openWallet(stewardWalletConfig, stewardWalletCredentials);

    console.log("\"Sovrin Steward\" -> Create and store in Wallet DID from seed");
    let stewardDidInfo = {
        'seed': '000000000000000000000000Steward1'
    };

    let [stewardDid, stewardKey] = await indy.createAndStoreMyDid(stewardWallet, stewardDidInfo);

    console.log(stewardDid);

    console.log(" \"Sovrin Steward\" -> Close and Delete wallet");
    await indy.closeWallet(stewardWallet);
    await indy.deleteWallet(stewardWalletConfig, stewardWalletCredentials);


    // console.log("Close and Delete pool");
    // await indy.closePoolLedger(poolHandle);
    // await indy.deletePoolLedgerConfig(poolName);

    console.log("Getting started -> done")
}

if (require.main.filename == __filename) {
    run()
}

module.exports = {
    run
}
