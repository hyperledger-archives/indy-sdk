		System.out.println("\n13. Reading new Verkey from wallet\n");
		String trustAnchorVerkeyFromWallet = Did.keyForLocalDid(walletHandle, trustAnchorDID).get();

		System.out.println("\n14. Building GET_NYM request to get Trust Anchor from Verkey\n");
		String getNymRequest = buildGetNymRequest(trustAnchorDID, trustAnchorDID).get();
		System.out.println("GET_NYM request:\n" + getNymRequest);

		System.out.println("\n15. Sending GET_NYM request to ledger\n");
		String getNymResponse = submitRequest(pool, getNymRequest).get();
		System.out.println("GET_NYM response:\n" + getNymResponse);

		System.out.println("\n16. Comparing Trust Anchor verkeys\n");
		System.out.println("Written by Steward: " + trustAnchorDID);
		System.out.println("Current from wallet: " + trustAnchorVerkeyFromWallet);
		String responseData = new JSONObject(getNymResponse).getJSONObject("result").getString("data");
		String trustAnchorVerkeyFromLedger = new JSONObject(responseData).getString("verkey");
		System.out.println("Current from ledger: " + trustAnchorVerkeyFromLedger);
		boolean match = !trustAnchorDID.equals(trustAnchorVerkeyFromWallet) && trustAnchorVerkeyFromWallet.equals(trustAnchorVerkeyFromWallet);
		System.out.println("Matching: " + match);

        // Do some cleanup.
		System.out.println("\n17. Close and delete wallet\n");
		walletHandle.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();

		System.out.println("\n18. Close pool\n");
		pool.closePoolLedger().get();

		System.out.println("\n19. Delete pool ledger config\n");
		Pool.deletePoolLedgerConfig(poolName).get();
