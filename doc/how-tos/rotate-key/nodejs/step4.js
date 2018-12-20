    // 13.
    log('13. Reading new verkey from wallet')
    const trustAnchorVerkeyInWallet = await indy.keyForLocalDid(walletHandle, trustAnchorDid)
    logValue('Trust Anchor Verkey in wallet: ', trustAnchorVerkeyInWallet)

    // 14.
    log('14. Building GET_NYM request to get Trust Anchor verkey')
    const getNymRequest = await indy.buildGetNymRequest(trustAnchorDid, trustAnchorDid)

    // 15.
    log('15. Sending GET_NYM request to ledger')
    const getNymResponse = await indy.submitRequest(poolHandle, getNymRequest)

    // 16.
    log('16. Comparing Trust Anchor verkeys: written by Steward (original), current in wallet and current from ledger')
    logValue('Written by Steward: ', trustAnchorVerkey)
    logValue('Trust Anchor Verkey in wallet: ', trustAnchorVerkeyInWallet)
    const trustAnchorVerkeyFromLedger = JSON.parse(getNymResponse['result']['data'])['verkey']
    logValue('Trust Anchor Verkey from ledger: ', trustAnchorVerkeyFromLedger)
    logValue('Matching: ', trustAnchorVerkeyFromLedger == trustAnchorVerkeyInWallet != trustAnchorVerkey)

    // Do some cleanup.

    // 17.
    log('17. Closing wallet and pool')
    await indy.closeWallet(walletHandle)
    await indy.closePoolLedger(poolHandle)

    // 18.
    log('18. Deleting created wallet')
    await indy.deleteWallet(walletConfig, walletCredentials)

    // 19.
    log('19. Deleting pool ledger config')
    await indy.deletePoolLedgerConfig(poolName)
