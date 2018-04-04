package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * ledger.rs results
 */
public final class LedgerResults {

	private LedgerResults() {

	}

	/**
	 * Result from calling parseResponse functions.
	 */
	public static class ParseResponseResult extends IndyJava.Result {

		private String id, objectJson;

		ParseResponseResult(String id, String objectJson) {
			this.id = id;
			this.objectJson = objectJson;
		}

		/**
		 * Gets the Id.
		 *
		 * @return Id.
		 */
		public String getId() {
			return this.id;
		}

		/**
		 * Gets the object JSON.
		 *
		 * @return The object JSON.
		 */
		public String getObjectJson() {
			return this.objectJson;
		}
	}
}
