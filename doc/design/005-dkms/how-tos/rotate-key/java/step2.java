        // Tell SDK which pool you are going to use. You should have already started
        // this pool using docker compose or similar. Here, we are dumping the config
        // just for demonstration purposes.
        System.out.println("\n1. Creating a new local pool ledger configuration that can be used later to connect pool nodes.\n");
		Pool.createPoolLedgerConfig(poolName, poolConfig).get();

		System.out.println("\n2. Open pool ledger and get the pool handle from libindy.\n");
		Pool pool = Pool.openPoolLedger(poolName, "{}").get();

		System.out.println("\n3. Creates a new secure wallet\n");
		Wallet.createWallet(poolName, walletName, "default", null, null).get();

		System.out.println("\n4. Open wallet and get the wallet handle from libindy\n");
		Wallet walletHandle = Wallet.openWallet(walletName, null, null).get();

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
		System.out.println("Steward did: " + defautStewardDid);

        // Now, create a new DID and verkey for a trust anchor, and store it in our wallet as well. Don't use a seed;
        // this DID and its keyas are secure and random. Again, we're not writing to the ledger yet.
		System.out.println("\n6. Generating and storing Trust Anchor DID and Verkey\n");
		DidResults.CreateAndStoreMyDidResult trustAnchorResult = Did.createAndStoreMyDid(walletHandle, "{}").get();
		String trustAnchorDID = trustAnchorResult.getDid();
		String trustAnchorVerkey = trustAnchorResult.getVerkey();
		System.out.println("Trust anchor DID: " + trustAnchorDID);
		System.out.println("Trust anchor Verkey: " + trustAnchorVerkey);

        // Here, we are building the transaction payload that we'll send to write the Trust Anchor identity to the ledger.
        // We submit this transaction under the authority of the steward DID that the ledger already recognizes.
        // This call will look up the private key of the steward DID in our wallet, and use it to sign the transaction.
		System.out.println("\n7. Build NYM request to add Trust Anchor to the ledger\n");
		String nymRequest = buildNymRequest(defautStewardDid, trustAnchorDID, trustAnchorVerkey, null, "TRUST_ANCHOR").get();
		System.out.println("NYM request JSON:\n" + nymRequest);

        // Now that we have the transaction ready, send it. The building and the sending are separate steps because some
        // clients may want to prepare transactions in one piece of code (e.g., that has access to privileged backend systems),
        // and communicate with the ledger in a different piece of code (e.g., that lives outside the safe internal
        // network).
		System.out.println("\n8. Sending NYM request to ledger\n");
		String nymResponseJson = signAndSubmitRequest(pool, walletHandle, defautStewardDid, nymRequest).get();
		System.out.println("NYM transaction response:\n" + nymResponseJson);

        // At this point, we have successfully written a new identity to the ledger.
