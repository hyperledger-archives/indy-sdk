    // First, put a steward DID and its keypair in the wallet. This doesn't write anything to the ledger,
    // but it gives us a key that we can use to sign a ledger transaction that we're going to submit later.
    // The DID and public verkey for this steward key are already in the ledger; they were part of the genesis
    // transactions we told the SDK to start with in the previous step. But we have to also put the DID, verkey,
    // and private signing key into our wallet, so we can use the signing key to submit an acceptably signed
    // transaction to the ledger, creating our *next* DID (which is truly new). This is why we use a hard-coded seed
    // when creating this DID--it guarantees that the same DID and key material are created that the genesis txns
    // expect.

    // 5.
    log('5. Generating and storing steward DID and verkey')
    const stewardSeed = '000000000000000000000000Steward1'
    const did = {'seed': stewardSeed}
    const [stewardDid, stewardVerkey] = await indy.createAndStoreMyDid(walletHandle, did)
    logValue('Steward DID: ', stewardDid)
    logValue('Steward Verkey: ', stewardVerkey)

    // Now, create a new DID and verkey for a trust anchor, and store it in our wallet as well. Don't use a seed;
    // this DID and its keyas are secure and random. Again, we're not writing to the ledger yet.

    // 6.
    log('6. Generating and storing trust anchor DID and verkey')
    const [trustAnchorDid, trustAnchorVerkey] = await indy.createAndStoreMyDid(walletHandle, "{}")
    logValue('Trust anchor DID: ', trustAnchorDid)
    logValue('Trust anchor Verkey: ', trustAnchorVerkey)

