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
	 * Result from calling issuerCreateSchema.
	 */
	public static class IssuerCreateSchemaResult extends IndyJava.Result {

		private String schemaId, schemaJson;

		IssuerCreateSchemaResult(String schemaId, String schemaJson) {
			this.schemaId = schemaId;
			this.schemaJson = schemaJson;
		}

		/**
		 * Gets the schema Id.
		 *
		 * @return Schema Id.
		 */
		public String getSchemaId() {
			return this.schemaId;
		}

		/**
		 * Gets the schema JSON.
		 *
		 * @return The schema JSON.
		 */
		public String getSchemaJson() {
			return this.schemaJson;
		}
	}

	/**
	 * Result from calling issuerCreateAndStoreClaimDef.
	 */
	public static class IssuerCreateAndStoreClaimDefResult extends IndyJava.Result {

		private String claimDefId, claimDefJson;

		IssuerCreateAndStoreClaimDefResult(String claimDefId, String claimDefJson) {
			this.claimDefId = claimDefId;
			this.claimDefJson = claimDefJson;
		}

		/**
		 * Gets the claim def Id.
		 *
		 * @return claim def Id.
		 */
		public String getClaimDefId() {
			return this.claimDefId;
		}

		/**
		 * Gets the claim definition JSON.
		 *
		 * @return The claim definition JSON.
		 */
		public String getClaimDefJson() {
			return this.claimDefJson;
		}
	}

	/**
	 * Result from calling issuerCreateAndStoreRevocReg.
	 */
	public static class IssuerCreateAndStoreRevocRegResult extends IndyJava.Result {

		private String revRegid, revRegDefJson, revRegEntryJson;

		IssuerCreateAndStoreRevocRegResult(String revRegid, String revRegDefJson, String revRegEntryJson) {
			this.revRegid = revRegid;
			this.revRegDefJson = revRegDefJson;
			this.revRegEntryJson = revRegEntryJson;
		}

		/**
		 * Gets the revocation registry Id.
		 *
		 * @return revocation registry Id.
		 */
		public String getRevRegId() {
			return this.revRegid;
		}

		/**
		 * Gets the revocation registry definition JSON.
		 *
		 * @return The revocation registry definition JSON.
		 */
		public String getRevRegDefJson() {
			return this.revRegDefJson;
		}

		/**
		 * Gets the revocation registry entry JSON.
		 *
		 * @return The revocation registry entry JSON.
		 */
		public String getRevRegEntryJson() {
			return this.revRegEntryJson;
		}
	}

	/**
	 * Result from calling issuerCreateClaim.
	 */
	public static class IssuerCreateClaimResult extends IndyJava.Result {

		private String revocRegUpdateJson, claimJson;

		IssuerCreateClaimResult(String revocRegUpdateJson, String claimJson) {
			this.revocRegUpdateJson = revocRegUpdateJson;
			this.claimJson = claimJson;
		}

		/**
		 * Gets the revocation registration update JSON.
		 *
		 * @return The revocation registration update JSON.
		 */
		public String getRevocRegDeltaJson() {
			return this.revocRegUpdateJson;
		}

		/**
		 * Gets the claim JSON.
		 *
		 * @return The claim JSON.
		 */
		public String getClaimJson() {
			return this.claimJson;
		}
	}
}
