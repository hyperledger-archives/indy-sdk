		System.out.println("\n9. Generating new Verkey of Trust Anchor in the wallet\n");
		String newTrustAnchorVerkey = Did.replaceKeysStart(walletHandle, trustAnchorDID, "{}").get();
		System.out.println("New Trust Anchor's Verkey: " + newTrustAnchorVerkey);

		System.out.println("\n10. Building NYM request to update new verkey to ledger\n");
		String nymUpdateRequest = buildNymRequest(trustAnchorDID, trustAnchorDID, newTrustAnchorVerkey, null, "TRUST_ANCHOR").get();
		System.out.println("NYM request:\n" + nymUpdateRequest);

		System.out.println("\n11. Sending NYM request to the ledger\n");
		String nymUpdateResponse = signAndSubmitRequest(pool, walletHandle, trustAnchorDID, nymUpdateRequest).get();
		System.out.println("NYM response:\n" + nymUpdateRequest);

		System.out.println("\n12. Applying new Trust Anchor's Verkey in wallet\n");
		Did.replaceKeysApply(walletHandle, trustAnchorDID);

