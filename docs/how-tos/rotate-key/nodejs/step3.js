    // 9.
    log('9. Generating new verkey of trust anchor in wallet')
    const newVerkey = await indy.replaceKeysStart(walletHandle, trustAnchorDid, "{}")
    logValue('New Trust Anchor Verkey: ', newVerkey)

    // 10.
    log('10. Building NYM request to update new verkey to ledger')
    const nymRequestForNewVerkey = await indy.buildNymRequest(trustAnchorDid, trustAnchorDid, newVerkey, undefined, 'TRUST_ANCHOR')

    // 11.
    log('11. Sending NYM request to the ledger')
    await indy.signAndSubmitRequest(poolHandle, walletHandle, trustAnchorDid, nymRequestForNewVerkey)

    // 12.
    log('12. Apply new verkey in wallet')
    await indy.replaceKeysApply(walletHandle, trustAnchorDid)

