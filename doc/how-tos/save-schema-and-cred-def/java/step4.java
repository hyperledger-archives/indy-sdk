		System.out.println("\n11. Creating and storing CRED DEF using anoncreds as Trust Anchor, for the given Schema\n");
		String credDefJSON = "{\"seqNo\": 1, \"dest\": \"" + defaultStewardDid + "\", \"data\": " + schemaDataJSON + "}";
		System.out.println("Cred Def JSON:\n" + credDefJSON);
		String credDef = issuerCreateAndStoreClaimDef(walletHandle, trustAnchorDID, credDefJSON, "CL", false).get();
		System.out.println("Returned Cred Definition:\n" + credDef);

		// Some cleanup code.
		System.out.println("\n12. Close and delete wallet\n");
		walletHandle.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();

		System.out.println("\n13. Close pool\n");
		pool.closePoolLedger().get();

		System.out.println("\n14. Delete pool ledger config\n");
		Pool.deletePoolLedgerConfig(poolName).get();

