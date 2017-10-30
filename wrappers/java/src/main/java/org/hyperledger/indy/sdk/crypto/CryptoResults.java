package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * crypto.rs results
 */

/**
 * Result classes for Crypto operations.
 */
public final class CryptoResults {

	private CryptoResults() {

	}

	/**
	 * Result from calling encrypt.
	 */
	public static class EncryptResult extends IndyJava.Result {

		private byte[] encryptedMessage, nonce;
		EncryptResult(byte[] encryptedMessage, byte[] nonce) { this.encryptedMessage = encryptedMessage; this.nonce = nonce; }

		/**
		 * Gets the encrypted message.
		 *
		 * @return The encrypted message.
		 */
		public byte[] getEncryptedMessage() { return this.encryptedMessage; }

		/**
		 * Gets the nonce.
		 *
		 * @return The nonce.
		 */
		public byte[] getNonce() { return this.nonce; }
	}
}
