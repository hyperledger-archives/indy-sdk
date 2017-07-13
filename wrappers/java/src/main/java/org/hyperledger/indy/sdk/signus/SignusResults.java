package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyJava;

/**
 * signus.rs results
 */
public final class SignusResults {

	private SignusResults() {

	}

	public static class CreateAndStoreMyDidResult extends IndyJava.Result {

		private String did, verkey, pk;
		CreateAndStoreMyDidResult(String did, String verkey, String pk) { this.did = did; this.verkey = verkey; this.pk = pk; }
		public String getDid() { return this.did; }
		public String getVerkey() { return this.verkey; }
		public String getPk() { return this.pk; }
	}

	public static class ReplaceKeysResult extends IndyJava.Result {

		private String verkey, pk;
		ReplaceKeysResult(String verkey, String pk) { this.verkey = verkey; this.pk = pk; }
		public String getVerkey() { return this.verkey; }
		public String getPk() { return this.pk; }
	}
}
