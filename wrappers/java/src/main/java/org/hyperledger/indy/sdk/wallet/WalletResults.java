package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.SovrinJava;

/**
 * wallet.rs Results
 */
public final class WalletResults {

	private WalletResults() {

	}

	public static class CreateWalletResult extends SovrinJava.Result {

		CreateWalletResult() { }
	}

	public static class OpenWalletResult extends SovrinJava.Result {

		private Wallet wallet;
		OpenWalletResult(Wallet wallet) { this.wallet = wallet; }
		public Wallet getWallet() { return this.wallet; }
	}

	public static class CloseWalletResult extends SovrinJava.Result {

		CloseWalletResult() { }
	}

	public static class DeleteWalletResult extends SovrinJava.Result {

		DeleteWalletResult() { }
	}

	public static class WalletSetSeqNoForValueResult extends SovrinJava.Result {

		WalletSetSeqNoForValueResult() { }
	}
}
