        // First, put a steward DID and its keypair in the wallet. This doesn't write anything to the ledger,
        // but it gives us a key that we can use to sign a ledger transaction that we're going to submit later.
        System.out.println("\n5. Generating and storing steward DID and Verkey\n");

        // The DID and public verkey for this steward key are already in the ledger; they were part of the genesis
        // transactions we told the SDK to start with in the previous step. But we have to also put the DID, verkey,
        // and private signing key into our wallet, so we can use the signing key to submit an acceptably signed
        // transaction to the ledger, creating our *next* DID (which is truly new). This is why we use a hard-coded seed
        // when creating this DID--it guarantees that the same DID and key material are created that the genesis txns
        // expect.
        String did_json = "{\"seed\": \"" + stewardSeed + "\"}";
        DidResults.CreateAndStoreMyDidResult stewardResult = Did.createAndStoreMyDid(walletHandle, did_json).get();
        String defautStewardDid = stewardResult.getDid();
        System.out.println("Steward DID: " + defautStewardDid);
        System.out.println("Steward Verkey: " + stewardResult.getVerkey());

        // Now, create a new DID and verkey for a trust anchor, and store it in our wallet as well. Don't use a seed;
        // this DID and its keyas are secure and random. Again, we're not writing to the ledger yet.
        System.out.println("\n6. Generating and storing Trust Anchor DID and Verkey\n");
        DidResults.CreateAndStoreMyDidResult trustAnchorResult = Did.createAndStoreMyDid(walletHandle, "{}").get();
        String trustAnchorDID = trustAnchorResult.getDid();
        String trustAnchorVerkey = trustAnchorResult.getVerkey();
        System.out.println("Trust anchor DID: " + trustAnchorDID);
        System.out.println("Trust anchor Verkey: " + trustAnchorVerkey);
