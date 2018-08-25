		System.out.println("\n14. Issuer (Trust Anchor) is creating a Credential Offer for Prover\n");
		String credOfferJSON = issuerCreateClaimOffer(walletHandle, schemaJSON, trustAnchorDID, proverDID).get();
		System.out.println("Claim Offer:\n" + credOfferJSON);

		System.out.println("\n15. Prover creates Credential Request\n");
		String credRequestJSON = proverCreateAndStoreClaimReq(proverWalletHandle, proverDID, credOfferJSON,
				claimDef, masterSecretName).get();
		System.out.println("Cred Request:\n" + credRequestJSON);

		System.out.println("\n16. Issuer (Trust Anchor) creates Credential for Credential Request\n");
		// Encoded value of non-integer attribute is SHA256 converted to decimal
		// note that encoding is not standardized by Indy except that 32-bit integers are encoded as themselves. IS-786
		String credAttribsJson = "{\n" +
		"               \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
		"               \"name\":[\"Alex\",\"99262857098057710338306967609588410025648622308394250666849665532448612202874\"],\n" +
		"               \"height\":[\"175\",\"175\"],\n" +
		"               \"age\":[\"28\",\"28\"]\n" +
		"        }";
		AnoncredsResults.IssuerCreateClaimResult createClaimResult = issuerCreateClaim(walletHandle, credRequestJSON,
				credAttribsJson, - 1).get();
		String credJSON = createClaimResult.getClaimJson();
		System.out.println("Claim:\n" + credJSON);

		System.out.println("\n17. Prover processes and stores credential\n");
		Anoncreds.proverStoreClaim(proverWalletHandle, credJSON, null).get();

		// We have issued, received, and stored a credential. Mission accomplished!

		// Now do some cleanup.
		System.out.println("\n18. Close and delete wallet\n");
		walletHandle.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();

		System.out.println("\n19. Close pool\n");
		pool.closePoolLedger().get();

		System.out.println("\n20. Delete pool ledger config\n");
		Pool.deletePoolLedgerConfig(poolName).get();
