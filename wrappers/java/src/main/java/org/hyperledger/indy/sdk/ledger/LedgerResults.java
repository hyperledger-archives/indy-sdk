package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.SovrinJava;

/**
 * ledger.rs results
 */
public final class LedgerResults {

	private LedgerResults() {

	}

	public static class SignAndSubmitRequestResult extends SovrinJava.Result {

		private String requestResultJson;
		SignAndSubmitRequestResult(String requestResultJson) { this.requestResultJson = requestResultJson; }
		public String getRequestResultJson() { return this.requestResultJson; }
	}

	public static class SubmitRequestResult extends SovrinJava.Result {

		private String requestResultJson;
		SubmitRequestResult(String requestResultJson) { this.requestResultJson = requestResultJson; }
		public String getRequestResultJson() { return this.requestResultJson; }
	}

	public static class BuildGetDdoRequestResult extends SovrinJava.Result {

		private String requestJson;
		BuildGetDdoRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildNymRequestResult extends SovrinJava.Result {

		private String requestJson;
		BuildNymRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildAttribRequestResult extends SovrinJava.Result {

		private String requestJson;
		BuildAttribRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildGetAttribRequestResult extends SovrinJava.Result {

		private String requestJson;
		BuildGetAttribRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildGetNymRequestResult extends SovrinJava.Result {

		private String requestJson;
		BuildGetNymRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildSchemaRequestResult extends SovrinJava.Result {

		private String requestJson;
		BuildSchemaRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildGetSchemaRequestResult extends SovrinJava.Result {

		private String requestJson;
		BuildGetSchemaRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildClaimDefTxnResult extends SovrinJava.Result {

		private String requestJson;
		BuildClaimDefTxnResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildGetClaimDefTxnResult extends SovrinJava.Result {

		private String requestResultJson;
		BuildGetClaimDefTxnResult(String requestResultJson) { this.requestResultJson = requestResultJson; }
		public String getRequestResultJson() { return this.requestResultJson; }
	}

	public static class BuildNodeRequestResult extends SovrinJava.Result {

		private String requestJson;
		BuildNodeRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}
}
