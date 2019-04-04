"use strict";

const indy = require('indy-sdk');
const storagePlugin = require('./storagePlugin');

async function run() {

    console.log("testStoragePlugin.js -> started");

    let result = storagePlugin.postgresstorage_init();
    console.log(result);

    console.log("Create wallet");
    
    let walletConfig = {
        'id': 'wallet_psx',
        'storage_type': 'postgres_storage',
        'storage_config': {
            "url": "postgres-db:5432"
        }
    }
    let walletCredentials = {
        'key': '1',
        'storage_credentials': {
            "account": "postgres",
            "password": "mysecretpassword",
            "admin_account": "postgres",
            "admin_password": "mysecretpassword"
        }
    }

    try {
        result = await indy.createWallet(walletConfig, walletCredentials);
        console.log(result);
    } catch(e) {
        if(e.message !== "WalletAlreadyExistsError") {
            throw e;
        } else {
            console.log('WalletAlreadyExists')
        }
    }

    console.log('Opened wallet');

    let wallet = await indy.openWallet(walletConfig, walletCredentials);

    console.log("Create and store in Wallet DID");

    let [did, verkey] = await indy.createAndStoreMyDid(wallet, {});

    console.log(did);

    console.log("Close and Delete wallet");
    await indy.closeWallet(wallet);
    await indy.deleteWallet(walletConfig, walletCredentials);

    console.log("testStoragePlugin -> done")
}

if (require.main.filename == __filename) {
    run()
}

module.exports = {
    run
}
