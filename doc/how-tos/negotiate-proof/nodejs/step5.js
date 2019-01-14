    log("11. Verifier is verifying proof from Prover")
    const verified = await indy.verifierVerifyProof(proofRequest, proof, schemas, credentialDefs, revocRegs, revRegs)

    logValue("Proof :")
    logValue(". Name="+proof['requested_proof']['revealed_attrs']['attr1_referent']['raw'])
    logValue(". Verified="+verified)

    // 12
    log("12. Closing both wallet_handles")
    await indy.closeWallet(issuerWalletHandle)
    await indy.closeWallet(proverWalletHandle)

    // 13
    log("13. Deleting created wallet_handles")
    await indy.deleteWallet(proverWalletName, proverWalletCredentials)
    await indy.deleteWallet(issuerWalletName, issuerWalletCredentials)
