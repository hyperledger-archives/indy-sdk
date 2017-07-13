package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * anoncreds.rs results
 */
public final class AnoncredsResults {

	private AnoncredsResults() {

	}

	public static class IssuerCreateAndStoreRevocRegResult extends IndyJava.Result {

		private String revocRegJson, revocRegUuid;
		IssuerCreateAndStoreRevocRegResult(String revocRegJson, String revocRegUuid) { this.revocRegJson = revocRegJson; this.revocRegUuid = revocRegUuid; }
		public String getRevocRegJson() { return this.revocRegJson; }
		public String getRevocRegUuid() { return this.revocRegUuid; }
	}

	public static class IssuerCreateClaimResult extends IndyJava.Result {

		private String revocRegUpdateJson, claimJson;
		IssuerCreateClaimResult(String revocRegUpdateJson, String claimJson) { this.revocRegUpdateJson = revocRegUpdateJson; this.claimJson = claimJson; }
		public String getRevocRegUpdateJson() { return this.revocRegUpdateJson; }
		public String getClaimJson() { return this.claimJson; }
	}
}
