package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.SovrinJava;

/**
 * signus.rs results
 */
public final class SignusResults {

	private SignusResults() {

	}

	public static class CreateAndStoreMyDidResult extends SovrinJava.Result {

		private String did, verkey, pk;
		CreateAndStoreMyDidResult(String did, String verkey, String pk) { this.did = did; this.verkey = verkey; this.pk = pk; }
		public String getDid() { return this.did; }
		public String getVerkey() { return this.verkey; }
		public String getPk() { return this.pk; }
	}

	public static class ReplaceKeysResult extends SovrinJava.Result {

		private String verkey, pk;
		ReplaceKeysResult(String verkey, String pk) { this.verkey = verkey; this.pk = pk; }
		public String getVerkey() { return this.verkey; }
		public String getPk() { return this.pk; }
	}

	public static class StoreTheirDidResult extends SovrinJava.Result {

		StoreTheirDidResult() { }
	}

	public static class SignResult extends SovrinJava.Result {

		private String signature;
		SignResult(String signature) { this.signature = signature; }
		public String getSignature() { return this.signature; }
	}

	public static class VerifySignatureResult extends SovrinJava.Result {

		private boolean valid;
		VerifySignatureResult(boolean valid) { this.valid = valid; }
		public boolean isValid() { return this.valid; }
	}

	public static class EncryptResult extends SovrinJava.Result {

		private String encryptedMsg;
		EncryptResult(String encryptedMsg) { this.encryptedMsg = encryptedMsg; }
		public String getEncryptedMsg() { return this.encryptedMsg; }
	}

	public static class DecryptResult extends SovrinJava.Result {

		private String decryptedMsg;
		DecryptResult(String decryptedMsg) { this.decryptedMsg = decryptedMsg; }
		public String getDecryptedMsg() { return this.decryptedMsg; }
	}
}
