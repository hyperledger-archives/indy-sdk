		System.out.println("\n12. Creating Prover wallet and opening it to get the handle\n");
		String proverDID = "VsKV7grR1BUE29mG2Fm2kX";
		String proverWalletName = "prover_wallet";
		Wallet.createWallet(poolName, proverWalletName, null, null, null);
		Wallet proverWalletHandle = Wallet.openWallet(proverWalletName, null, null).get();

		System.out.println("\n13. Prover is creating Link Secret\n");
		String linkSecretName = "link_secret";
		Anoncreds.proverCreateMasterSecret(proverWalletHandle, linkSecretName).get();

