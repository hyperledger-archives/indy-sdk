package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * wallet.rs Results
 */
public final class WalletResults {

	private WalletResults() {

	}

	public static class CreateWalletResult extends IndyJava.Result {

		CreateWalletResult() { }
	}

	public static class OpenWalletResult extends IndyJava.Result {

		private Wallet wallet;
		OpenWalletResult(Wallet wallet) { this.wallet = wallet; }
		public Wallet getWallet() { return this.wallet; }
	}

	public static class CloseWalletResult extends IndyJava.Result {

		CloseWalletResult() { }
	}

	public static class DeleteWalletResult extends IndyJava.Result {

		DeleteWalletResult() { }
	}

	public static class WalletSetSeqNoForValueResult extends IndyJava.Result {

		WalletSetSeqNoForValueResult() { }
	}
}
