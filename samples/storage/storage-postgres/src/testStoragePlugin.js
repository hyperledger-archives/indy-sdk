"use strict";

const indy = require('indy-sdk');
const storagePlugin = require('./storagePlugin');

async function run() {

    console.log("testStoragePlugin.js -> started");

    const result = storagePlugin.postgresstorage_init();
    console.log(result);

    console.log("Create wallet");
    
    let stewardWalletConfig = {
        'id': 'wallet_psx2',
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

    console.log('Opened wallet');

    let stewardWallet = await indy.openWallet(stewardWalletConfig, stewardWalletCredentials);

    console.log("Create and store in Wallet DID from seed");

    let [did, verkyey] = await indy.createAndStoreMyDid(stewardWallet, stewardDidInfo);

    console.log(did);

    // console.log("Close and Delete wallet");
    // await indy.closeWallet(stewardWallet);
    // await indy.deleteWallet(stewardWalletConfig, stewardWalletCredentials);

    console.log("testStoragePlugin -> done")
}

if (require.main.filename == __filename) {
    run()
}

module.exports = {
    run
}
