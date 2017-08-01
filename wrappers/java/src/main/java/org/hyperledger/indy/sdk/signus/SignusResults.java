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
	public static class ReplaceKeysResult extends IndyJava.Result {

		private String verkey, pk;
		ReplaceKeysResult(String verkey, String pk) { this.verkey = verkey; this.pk = pk; }
		
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
}
