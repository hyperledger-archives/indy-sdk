    // Here we are creating a third DID. This one is never written to the ledger, but we do have to have it in the
    // wallet, because every request to the ledger has to be signed by some requester. By creating a DID here, we
    // are forcing the wallet to allocate a keypair and identity that we can use to sign the request that's going
    // to read the trust anchor's info from the ledger.

    // 9.
    log('9. Generating and storing DID and verkey representing a Client that wants to obtain Trust Anchor Verkey')
    const [clientDid, clientVerkey] = await indy.createAndStoreMyDid(walletHandle, "{}")
    logValue('Client DID: ', clientDid)
    logValue('Client Verkey: ', clientVerkey)

    // 10.
    log('10. Building the GET_NYM request to query trust anchor verkey')
    const getNymRequest = await indy.buildGetNymRequest(/*submitter_did*/ clientDid,
        /*target_did*/ trustAnchorDid)

    // 11.
    log('11. Sending the Get NYM request to the ledger')
    const getNymResponse = await indy.submitRequest(/*pool_handle*/ poolHandle,
        /*request_json*/ getNymRequest)

    // See whether we received the same info that we wrote the ledger in step 4.

    // 12.
    log('12. Comparing Trust Anchor verkey as written by Steward and as retrieved in GET_NYM response submitted by Client')
    logValue('Written by Steward: ', trustAnchorVerkey)
    const verkeyFromLedger = JSON.parse(getNymResponse['result']['data'])['verkey']
    logValue('Queried from ledger: ', verkeyFromLedger)
    logValue('Matching: ', verkeyFromLedger == trustAnchorVerkey)

    // Do some cleanup.

    // 13.
    log('13. Closing wallet and pool')
    await indy.closeWallet(walletHandle)
    await indy.closePoolLedger(poolHandle)

    // 14.
    log('14. Deleting created wallet')
    await indy.deleteWallet(walletName, walletCredentials)

    // 15.
    log('15. Deleting pool ledger config')
    await indy.deletePoolLedgerConfig(poolName)
