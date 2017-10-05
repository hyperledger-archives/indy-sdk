package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * signus.rs results
 */
/**
 * Result classes for Signus operations.
 */
public final class SignusResults {

	private SignusResults() {

	}

	/**
	 * Result from calling createAndStoreMyDid.
	 */
	public static class CreateAndStoreMyDidResult extends IndyJava.Result {

		private String did, verkey, pk;
		CreateAndStoreMyDidResult(String did, String verkey, String pk) { this.did = did; this.verkey = verkey; this.pk = pk; }
		
		/**
		 * Gets the DID.
		 * 
		 * @return The DID.
		 */
		public String getDid() { return this.did; }
		
		/**
		 * Gets the verification key.
		 * 
		 * @return The verification key.
		 */
		public String getVerkey() { return this.verkey; }
		
		/**
		 * Gets the public key.
		 * 
		 * @return The public key.
		 */
		public String getPk() { return this.pk; }
	}

	/**
	 * Result from calling replaceKeys.
	 */
	public static class ReplaceKeysStartResult extends IndyJava.Result {

		private String verkey, pk;
		ReplaceKeysStartResult(String verkey, String pk) { this.verkey = verkey; this.pk = pk; }
		
		/**
		 * Gets the verification key.
		 * 
		 * @return The verification key.
		 */
		public String getVerkey() { return this.verkey; }
		
		/**
		 * Gets the public key.
		 * 
		 * @return The public key.
		 */
		public String getPk() { return this.pk; }
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
