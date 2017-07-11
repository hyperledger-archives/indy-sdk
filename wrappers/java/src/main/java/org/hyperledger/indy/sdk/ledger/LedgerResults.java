package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * ledger.rs results
 */
public final class LedgerResults {

	private LedgerResults() {

	}

	public static class SignAndSubmitRequestResult extends IndyJava.Result {

		private String requestResultJson;
		SignAndSubmitRequestResult(String requestResultJson) { this.requestResultJson = requestResultJson; }
		public String getRequestResultJson() { return this.requestResultJson; }
	}

	public static class SubmitRequestResult extends IndyJava.Result {

		private String requestResultJson;
		SubmitRequestResult(String requestResultJson) { this.requestResultJson = requestResultJson; }
		public String getRequestResultJson() { return this.requestResultJson; }
	}

	public static class BuildGetDdoRequestResult extends IndyJava.Result {

		private String requestJson;
		BuildGetDdoRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildNymRequestResult extends IndyJava.Result {

		private String requestJson;
		BuildNymRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildAttribRequestResult extends IndyJava.Result {

		private String requestJson;
		BuildAttribRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildGetAttribRequestResult extends IndyJava.Result {

		private String requestJson;
		BuildGetAttribRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildGetNymRequestResult extends IndyJava.Result {

		private String requestJson;
		BuildGetNymRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildSchemaRequestResult extends IndyJava.Result {

		private String requestJson;
		BuildSchemaRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildGetSchemaRequestResult extends IndyJava.Result {

		private String requestJson;
		BuildGetSchemaRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildClaimDefTxnResult extends IndyJava.Result {

		private String requestJson;
		BuildClaimDefTxnResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}

	public static class BuildGetClaimDefTxnResult extends IndyJava.Result {

		private String requestResultJson;
		BuildGetClaimDefTxnResult(String requestResultJson) { this.requestResultJson = requestResultJson; }
		public String getRequestResultJson() { return this.requestResultJson; }
	}

	public static class BuildNodeRequestResult extends IndyJava.Result {

		private String requestJson;
		BuildNodeRequestResult(String requestJson) { this.requestJson = requestJson; }
		public String getRequestJson() { return this.requestJson; }
	}
}
