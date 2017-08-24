package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * anoncreds.rs results
 */
/**
 * Result classes related to anonymous credentials calls.
 */
public final class AnoncredsResults {

	private AnoncredsResults() {

	}

	/**
	 * Result from calling issuerCreateClaim.
	 */
	public static class IssuerCreateClaimResult extends IndyJava.Result {

		private String revocRegUpdateJson, claimJson;
		IssuerCreateClaimResult(String revocRegUpdateJson, String claimJson) { this.revocRegUpdateJson = revocRegUpdateJson; this.claimJson = claimJson; }
		
		/**
		 * Gets the revocation registration update JSON.
		 * 
		 * @return The revocation registration update JSON.
		 */
		public String getRevocRegUpdateJson() { return this.revocRegUpdateJson; }
		
		/**
		 * Gets the claim JSON.
		 * 
		 * @return The claim JSON.
		 */
		public String getClaimJson() { return this.claimJson; }
	}
}
