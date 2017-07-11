package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * anoncreds.rs results
 */
public final class AnoncredsResults {

	private AnoncredsResults() {

	}

	public static class IssuerCreateAndStoreClaimDefResult extends IndyJava.Result {

		private String claimDefJson, claimDefUuid;
		IssuerCreateAndStoreClaimDefResult(String claimDefJson, String claimDefUuid) { this.claimDefJson = claimDefJson; this.claimDefUuid = claimDefUuid; }
		public String getClaimDefJson() { return this.claimDefJson; }
		public String getClaimDefUuid() { return this.claimDefUuid; }
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

	public static class IssuerRevokeClaimResult extends IndyJava.Result {

		private String revocRegUpdateJson;
		IssuerRevokeClaimResult(String revocRegUpdateJson) { this.revocRegUpdateJson = revocRegUpdateJson; }
		public String getRevocRegUpdateJson() { return this.revocRegUpdateJson; }
	}

	public static class ProverStoreClaimOfferResult extends IndyJava.Result {

		ProverStoreClaimOfferResult() { }
	}

	public static class ProverGetClaimOffersResult extends IndyJava.Result {

		private String claimOffersJson;
		ProverGetClaimOffersResult(String claimOffersJson) { this.claimOffersJson = claimOffersJson; }
		public String getClaimOffersJson() { return this.claimOffersJson; }
	}

	public static class ProverCreateMasterSecretResult extends IndyJava.Result {

		ProverCreateMasterSecretResult() { }
	}

	public static class ProverCreateAndStoreClaimReqResult extends IndyJava.Result {

		private String claimReqJson;
		ProverCreateAndStoreClaimReqResult(String claimReqJson) { this.claimReqJson = claimReqJson; }
		public String getClaimReqJson() { return this.claimReqJson; }
	}

	public static class ProverStoreClaimResult extends IndyJava.Result {

		ProverStoreClaimResult() { }
	}

	public static class ProverGetClaimsResult extends IndyJava.Result {

		private String claimsJson;
		ProverGetClaimsResult(String claimsJson) { this.claimsJson = claimsJson; }
		public String getClaimsJson() { return this.claimsJson; }
	}

	public static class ProverGetClaimsForProofReqResult extends IndyJava.Result {

		private String claimsJson;
		ProverGetClaimsForProofReqResult(String claimsJson) { this.claimsJson = claimsJson; }
		public String getClaimsJson() { return this.claimsJson; }
	}

	public static class ProverCreateProofResult extends IndyJava.Result {

		private String proofJson;
		ProverCreateProofResult(String proofJson) { this.proofJson = proofJson; }
		public String getProofJson() { return this.proofJson; }
	}

	public static class VerifierVerifyProofResult extends IndyJava.Result {

		private boolean valid;
		VerifierVerifyProofResult(boolean valid) { this.valid = valid; }
		public boolean isValid() { return this.valid; }
	}
}
