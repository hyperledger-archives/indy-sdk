package org.hyperledger.indy.sdk.anoncreds;

import static org.hyperledger.indy.sdk.utils.EnvironmentUtils.getIndyHomePath;
import static org.junit.Assert.*;


import blob_storage.BlobStorage;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.*;


public class IssuerRevokeCredentialTest extends AnoncredsIntegrationTest {

	private String tailsWriterConfig = String.format("{\"base_dir\":\"%s\", \"uri_pattern\":\"\"}", getIndyHomePath("tails"));

	@Test
	public void testIssuerRevokeProofWorks() throws Exception {
		//1. Create wallet, get wallet handle
		String walletName = "revocationWallet";
		Wallet.createWallet("default", walletName, "default", null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();

		//2. Issuer create Schema
		AnoncredsResults.IssuerCreateSchemaResult createSchemaResult = Anoncreds.issuerCreateSchema(issuerDid, gvtSchemaName, schemaVersion, gvtSchemaAttributes).get();
		String schemaJson = createSchemaResult.getSchemaJson();

		//3. Issuer create credential definition
		String revocationCredentialDefConfig = "{\"support_revocation\":true}";
		AnoncredsResults.IssuerCreateAndStoreCredentialDefResult createCredentialDefResult = Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, schemaJson, tag, null, revocationCredentialDefConfig).get();
		String credentialDefId = createCredentialDefResult.getCredDefId();
		String credentialDefJson = createCredentialDefResult.getCredDefJson();

		//4. Issuer create revocation registry
		String revRegConfig = "{\"issuance_type\":null,\"max_cred_num\":5}";
		AnoncredsResults.IssuerCreateAndStoreRevocRegResult createRevRegResult = Anoncreds.issuerCreateAndStoreRevocReg(wallet, issuerDid, null, tag, credentialDefId, revRegConfig, "default", tailsWriterConfig).get();
		String revRegId = createRevRegResult.getRevRegId();
		String revRegDef = createRevRegResult.getRevRegDefJson();

		//5. Prover create Master Secret
		Anoncreds.proverCreateMasterSecret(wallet, masterSecretName).get();

		//6. Issuer create Credential Offer
		String credentialOfferJson = Anoncreds.issuerCreateCredentialOffer(wallet, credentialDefId, issuerDid, proverDid).get();

		//7. Prover store Credential Offer received from Issuer
		Anoncreds.proverStoreCredentialOffer(wallet, credentialOfferJson).get();

		//8. Prover create Credential Request
		String credentialReq = Anoncreds.proverCreateAndStoreCredentialReq(wallet, proverDid, credentialOfferJson, credentialDefJson, masterSecretName).get();

		//9. Issuer open TailsReader
		JSONObject revRegDeg = new JSONObject(revRegDef);
		BlobStorage tailsReader = BlobStorage.openReader("default",
				tailsWriterConfig,
				revRegDeg.getJSONObject("value").getString("tails_location"),
				revRegDeg.getJSONObject("value").getString("tails_hash")).get();

		//10. Issuer create Credential
		int userRevocIndex = 1;
		AnoncredsResults.IssuerCreateCredentialResult createCredentialResult = Anoncreds.issuerCreateCredentail(wallet, credentialReq, gvtCredentialValuesJson, revRegId, tailsReader.getTailsReaderHandle(), userRevocIndex).get();
		String credential = createCredentialResult.getCredentialJson();
		String revRegDelta = createCredentialResult.getRevocRegDeltaJson();

		//11. Prover create RevocationInfo
		int timestamp = 100;
		String revInfo = Anoncreds.createRevocationInfo(tailsReader.getTailsReaderHandle(), revRegDef, revRegDelta, timestamp, userRevocIndex).get();

		//12. Prover store RevocationInfo
		Anoncreds.storeRevocationInfo(wallet, credentialId1, revInfo).get();

		//13. Prover store received Credential
		Anoncreds.proverStoreCredential(wallet, credentialId1, credential, revRegDef).get();

		//14. Prover gets Credentials for Proof Request
		String credentialsJson = Anoncreds.proverGetCredentialsForProofReq(wallet, proofRequest).get();
		JSONObject credentials = new JSONObject(credentialsJson);
		JSONArray credentialsForAttr1 = credentials.getJSONObject("attrs").getJSONArray("attr1_referent");

		String credentialUuid = credentialsForAttr1.getJSONObject(0).getJSONObject("cred_info").getString("referent");

		//15. Prover create Proof
		String requestedCredentialsJson = String.format("{" +
				"\"self_attested_attributes\":{}," +
				"\"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%s\", \"revealed\":true, \"timestamp\":%d }}," +
				"\"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%s\", \"timestamp\":%d}}" +
				"}", credentialUuid, timestamp, credentialUuid, timestamp);

		String schemasJson = String.format("{\"%s\":%s}", credentialUuid, schemaJson);
		String credentialDefsJson = String.format("{\"%s\":%s}", credentialUuid, credentialDefJson);
		String revInfos = String.format("{\"%s\": { \"%s\":%s }}", credentialUuid, timestamp, revInfo);

		String proofJson = Anoncreds.proverCreateProof(wallet, proofRequest, requestedCredentialsJson, schemasJson, masterSecretName,
				credentialDefsJson, revInfos).get();
		JSONObject proof = new JSONObject(proofJson);

		//16. Issuer revoke Credential
		revRegDelta = Anoncreds.issuerRevokeCredential(wallet, tailsReader.getTailsReaderHandle(), revRegId, userRevocIndex).get();

		//17. Verifier verify proof
		JSONObject revealedAttr1 = proof.getJSONObject("requested_proof").getJSONObject("revealed_attrs").getJSONObject("attr1_referent");
		assertEquals("Alex", revealedAttr1.getString("raw"));

		String id = revealedAttr1.getString("referent");

		schemasJson = String.format("{\"%s\":%s}", id, schemaJson);
		credentialDefsJson = String.format("{\"%s\":%s}", id, credentialDefJson);
		String revRegDefsJson = String.format("{\"%s\":%s}", id, revRegDef);
		String revRegs = String.format("{\"%s\": { \"%s\":%s }}", id, timestamp, revRegDelta);

		boolean valid = Anoncreds.verifierVerifyProof(proofRequest, proofJson, schemasJson, credentialDefsJson, revRegDefsJson, revRegs).get();
		assertFalse(valid);

		// 17. Close and Delete Wallet
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}
}
