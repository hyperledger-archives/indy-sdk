        // Here we are creating a third DID. This one is never written to the ledger, but we do have to have it in the
        // wallet, because every request to the ledger has to be signed by some requester. By creating a DID here, we
        // are forcing the wallet to allocate a keypair and identity that we can use to sign the request that's going
        // to read the trust anchor's info from the ledger.
        System.out.println("\n9. Generating and storing DID and Verkey to query the ledger with\n");
        DidResults.CreateAndStoreMyDidResult clientResult = Did.createAndStoreMyDid(walletHandle, "{}").get();
        String clientDID = clientResult.getDid();
        String clientVerkey = clientResult.getVerkey();
        System.out.println("Client DID: " + clientDID);
        System.out.println("Client Verkey: " + clientVerkey);

        System.out.println("\n10. Building the GET_NYM request to query Trust Anchor's Verkey as the Client\n");
        String getNymRequest = buildGetNymRequest(clientDID, trustAnchorDID).get();
        System.out.println("GET_NYM request json:\n" + getNymRequest);

        System.out.println("\n11. Sending the GET_NYM request to the ledger\n");
        String getNymResponse = submitRequest(pool, getNymRequest).get();
        System.out.println("GET_NYM response json:\n" + getNymResponse);

        // See whether we received the same info that we wrote the ledger in step 4.
        System.out.println("\n12. Comparing Trust Anchor Verkey as written by Steward and as retrieved in Client's query\n");
        String responseData = new JSONObject(getNymResponse).getJSONObject("result").getString("data");
        String trustAnchorVerkeyFromLedger = new JSONObject(responseData).getString("verkey");
        System.out.println("Written by Steward: " + trustAnchorVerkey);
        System.out.println("Queried from Ledger: " + trustAnchorVerkeyFromLedger);
        System.out.println("Matching: " + trustAnchorVerkey.equals(trustAnchorVerkeyFromLedger));

        // Do some cleanup.
        System.out.println("\n13. Close and delete wallet\n");
        walletHandle.closeWallet().get();
        Wallet.deleteWallet(walletName, null).get();

        System.out.println("\n14. Close pool\n");
        pool.closePoolLedger().get();

        System.out.println("\n15. Delete pool ledger config\n");
        Pool.deletePoolLedgerConfig(poolName).get();
