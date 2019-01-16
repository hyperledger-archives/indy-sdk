    // 1.
    log("1. Creates Issuer wallet and opens it to get handle.")
    const issuerWalletName = {"id": "issuer_wallet"}
    const issuerWalletCredentials = {"key": "issuer_wallet_key"}
    await indy.createWallet(issuerWalletName, issuerWalletCredentials)
    const issuerWalletHandle = await indy.openWallet(issuerWalletName, issuerWalletCredentials)

    // 2.
    log("2. Creates Prover wallet and opens it to get handle.")
    const proverWalletName = {"id": "prover_wallet"}
    const proverWalletCredentials = {"key": "prover_wallet_key"}
    await indy.createWallet(proverWalletName, proverWalletCredentials)
    const proverWalletHandle = await indy.openWallet(proverWalletName, proverWalletCredentials)

    // 3.
    log("3. Issuer creates credential definition for schema")
    const schemaId = "1"
    const schema = {
        "id": schemaId,
        "ver": "1.0",
        "name": "gvt",
        "version": "1.0",
        "attrNames": ["age", "sex", "height", "name"]
    }
    const schemaKey = {
        "name": schema["name"],
        "version": schema["version"],
        "did": schema["dest"],
    }
    const [credDefId, credDef] = await indy.issuerCreateAndStoreCredentialDef(issuerWalletHandle, issuerDid, schema, "tag1", "CL", '{"support_revocation": false}')

    // 4.
    log("4. Prover creates Link Secret")
    const proverMasterSecret = await indy.proverCreateMasterSecret(proverWalletHandle, "link_secret")

    // 5.
    log("5. Issuer create Cred Offer")
    const credOffer = await indy.issuerCreateCredentialOffer(issuerWalletHandle, credDefId)

    // 6.
    log("6. Prover creates and stores Cred Request")
    const [credReq, credReqMetadata] = await indy.proverCreateCredentialReq(proverWalletHandle, proverDid, credOffer,
        credDef, proverMasterSecret)

    // 7.
    log("7. Issuer creates Credential for received Cred Request")
    const credValues = {
        "sex": {"raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233115103"},
        "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
        "height": {"raw": "175", "encoded": "175"},
        "age": {"raw": "28", "encoded": "28"}
    }
    const tailsWriterConfig = {'base_dir': util.getPathToIndyClientHome() + "/tails", 'uri_pattern': ''}
    const blobStorageReaderHandle = await indy.openBlobStorageReader('default', tailsWriterConfig)
    const [cred] = await indy.issuerCreateCredential(issuerWalletHandle, credOffer, credReq, credValues, undefined, blobStorageReaderHandle)

    // 8.
    log("8. Prover processes and stores received Credential")
    await indy.proverStoreCredential(proverWalletHandle, undefined, credReqMetadata, cred, credDef,undefined)
